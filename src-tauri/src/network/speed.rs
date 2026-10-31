//! Speed measurement transport and scheduling. Only numeric results cross the worker boundary.
use super::{Connection, Quality};
use crate::{module::Module, persistence::Persistence};
use serde::{Deserialize, Serialize};

pub const INTERVAL: u64 = 6 * 60 * 60 * 1000;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeedResult {
    pub download: f64,
    pub upload: f64,
    pub measured_at: u64,
    pub network_id: String,
}
impl SpeedResult {
    pub fn valid(&self) -> bool {
        self.download.is_finite()
            && self.download >= 0.0
            && self.upload.is_finite()
            && self.upload >= 0.0
    }
}
/// Nonblocking HTTP worker port: false selects the lightweight transfer budget.
pub type Output = Result<(f64, f64), ()>;
pub trait Transport: Send {
    fn cancel(&mut self);
    fn start(&mut self, full: bool);
    fn poll(&mut self) -> Option<Output>;
}
pub struct SpeedTests {
    transport: Box<dyn Transport>,
    persistence: Box<dyn Persistence>,
    pub last: Option<SpeedResult>,
    pub running: bool,
    pub failed: bool,
    pub automatic: bool,
    connection: String,
    generation: u64,
    pending: Option<(u64, String)>,
    next: u64,
    manual: bool,
    preferences: Option<Box<dyn crate::persistence::Storage>>,
}
impl SpeedTests {
    pub fn new(
        transport: impl Transport + 'static,
        persistence: impl Persistence + 'static,
    ) -> Self {
        let last = persistence
            .load(Module::Network)
            .and_then(|s| serde_json::from_value::<SpeedResult>(s.data["speed"].clone()).ok())
            .filter(SpeedResult::valid);
        Self {
            transport: Box::new(transport),
            persistence: Box::new(persistence),
            last,
            running: false,
            failed: false,
            automatic: true,
            connection: String::new(),
            generation: 0,
            pending: None,
            next: 0,
            manual: false,
            preferences: None,
        }
    }
    pub fn with_preferences(&mut self, storage: impl crate::persistence::Storage + 'static) {
        self.automatic = storage
            .read("network-preferences.json")
            .ok()
            .and_then(|s| serde_json::from_str::<bool>(&s).ok())
            .unwrap_or(true);
        self.preferences = Some(Box::new(storage));
    }
    pub fn set_automatic(&mut self, enabled: bool) {
        self.automatic = enabled;
        if let Some(storage) = &self.preferences {
            let _ = storage.write(
                "network-preferences.json",
                if enabled { "true" } else { "false" },
            );
        }
    }
    pub fn request(&mut self) {
        self.manual = true;
    }
    pub fn step(&mut self, c: &Connection, now: u64) {
        let identity = format!("{}:{}", c.id, c.network_identity());
        if self.connection != identity {
            self.connection = identity;
            self.generation += 1;
            self.next = 0;
            self.failed = false;
        }
        let interrupted = !c.connected()
            || c.metered
            || c.connectivity != 4
            || self
                .pending
                .as_ref()
                .is_some_and(|(generation, _)| *generation != self.generation);
        if self.running && interrupted {
            self.transport.cancel();
        }
        if let Some(result) = self.transport.poll() {
            self.running = false;
            if let Some((generation, id)) = self.pending.take() {
                if generation == self.generation && !interrupted {
                    match result {
                        Ok((download, upload)) => {
                            let value = SpeedResult {
                                download,
                                upload,
                                measured_at: now,
                                network_id: id,
                            };
                            if value.valid() {
                                self.last = Some(value);
                                self.failed = false;
                                self.persistence.save(&crate::snapshot::Snapshot {
                                    module: Module::Network,
                                    updated_at: Some(now),
                                    data: serde_json::json!({"speed": self.last}),
                                });
                            } else {
                                self.failed = true;
                            }
                        }
                        Err(()) => self.failed = true,
                    }
                }
            }
        }
        let manual = std::mem::take(&mut self.manual);
        // Unknown metering is treated conservatively by the NetworkManager adapter.
        if self.running || !c.connected() || c.connectivity != 4 || c.metered {
            return;
        }
        if manual || (self.automatic && now >= self.next) {
            self.transport.start(manual);
            self.running = true;
            self.failed = false;
            self.pending = Some((self.generation, c.network_identity()));
            self.next = now.saturating_add(INTERVAL);
        }
    }
    pub fn quality(&self, c: &Connection, now: u64, quality: Quality) -> Quality {
        let Some(last) = self.last.as_ref().filter(|last| {
            last.network_id == c.network_identity()
                && now >= last.measured_at
                && now - last.measured_at < INTERVAL
        }) else {
            return quality;
        };
        if last.download < 2.0 {
            Quality::Poor
        } else if last.download < 10.0 && quality == Quality::Good {
            Quality::Regular
        } else {
            quality
        }
    }
}

/// HTTP transfer measurements contain byte counts and elapsed time only.
pub trait Http: Send + Sync {
    fn transfer(
        &self,
        upload: bool,
        bytes: usize,
    ) -> impl std::future::Future<Output = Result<(u64, f64), ()>> + Send;
}
pub struct Cloudflare;
impl Http for Cloudflare {
    async fn transfer(&self, upload: bool, bytes: usize) -> Result<(u64, f64), ()> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|_| ())?;
        let request = if upload {
            client
                .post("https://speed.cloudflare.com/__up")
                .header("Content-Type", "application/octet-stream")
                .body(vec![0u8; bytes])
        } else {
            client.get(format!("https://speed.cloudflare.com/__down?bytes={bytes}"))
        };
        let start = std::time::Instant::now();
        let mut response = request
            .send()
            .await
            .map_err(|_| ())?
            .error_for_status()
            .map_err(|_| ())?;
        let mut count = 0;
        while let Some(chunk) = response.chunk().await.map_err(|_| ())? {
            count += chunk.len() as u64;
            if count > bytes as u64 {
                return Err(());
            }
        }
        if !upload && count != bytes as u64 {
            return Err(());
        }
        Ok((
            if upload { bytes as u64 } else { count },
            start.elapsed().as_secs_f64(),
        ))
    }
}
async fn measure(http: &impl Http, full: bool) -> Output {
    let budget = if full {
        (100_000_000, 50_000_000)
    } else {
        (25_000_000, 10_000_000)
    };
    let mut rates = [0.0; 2];
    for (index, (upload, bytes)) in [(false, budget.0), (true, budget.1)]
        .into_iter()
        .enumerate()
    {
        let (count, seconds) = http.transfer(upload, bytes).await?;
        if !seconds.is_finite() || seconds <= 0.0 {
            return Err(());
        }
        rates[index] = count as f64 * 8.0 / seconds / 1_000_000.0;
    }
    Ok((rates[0], rates[1]))
}
pub struct Worker<H = Cloudflare> {
    http: std::sync::Arc<H>,
    pending: Option<std::sync::mpsc::Receiver<Output>>,
    task: Option<tauri::async_runtime::JoinHandle<()>>,
}
impl<H> Worker<H> {
    pub fn new(http: H) -> Self {
        Self {
            http: std::sync::Arc::new(http),
            pending: None,
            task: None,
        }
    }
}
impl<H: Http + 'static> Transport for Worker<H> {
    fn cancel(&mut self) {
        // Aborting drops the HTTP future, including pending sends and body reads.
        if let Some(task) = &self.task {
            task.abort();
        }
    }
    fn start(&mut self, full: bool) {
        let (tx, rx) = std::sync::mpsc::channel();
        self.pending = Some(rx);
        let http = self.http.clone();
        self.task = Some(tauri::async_runtime::spawn(async move {
            let _ = tx.send(measure(http.as_ref(), full).await);
        }));
    }
    fn poll(&mut self) -> Option<Output> {
        let result = match self.pending.as_ref()?.try_recv() {
            Ok(value) => Some(value),
            Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(Err(())),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
        };
        if result.is_some() {
            self.pending = None;
            self.task = None;
        }
        result
    }
}

/// Deterministic HTTP worker for core tests; uses the same measurement path as the real worker.
#[cfg(test)]
pub struct Immediate<H: Http> {
    pub http: H,
    pub result: Option<Output>,
}
#[cfg(test)]
impl<H: Http> Transport for Immediate<H> {
    fn cancel(&mut self) {
        self.result = Some(Err(()));
    }
    fn start(&mut self, full: bool) {
        self.result = Some(tauri::async_runtime::block_on(measure(&self.http, full)));
    }
    fn poll(&mut self) -> Option<Output> {
        self.result.take()
    }
}
