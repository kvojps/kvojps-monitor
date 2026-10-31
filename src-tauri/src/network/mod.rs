//! Security, quality and traffic: continuous collection without disk caching.
mod network_manager;
mod probe;
mod security;
pub use security::*;
#[cfg(test)]
mod tests;

use crate::ports::{Clock, Files};
pub use network_manager::{NetworkManager, NetworkManagerDbus};
pub use probe::{PingSocket, Probe};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::net::{IpAddr, SocketAddr, SocketAddrV6};

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct Connection {
    pub id: String,
    pub kind: String,
    pub connectivity: u32,
    pub vpn: bool,
    pub gateway: Option<IpAddr>,
    pub interface: String,
    pub wifi: Option<Wifi>,
    pub revision: u64,
}
impl Connection {
    fn connected(&self) -> bool {
        !self.id.is_empty()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Quality {
    Good,
    Regular,
    Poor,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Measurements {
    pub latency: Option<f64>,
    pub jitter: Option<f64>,
    pub loss: Option<f64>,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct Traffic {
    pub download: f64,
    pub upload: f64,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkData {
    pub connected: bool,
    pub wired: bool,
    pub vpn: bool,
    pub quality: Option<Quality>,
    pub security: Option<Security>,
    pub reasons: Vec<String>,
    pub gateway: Measurements,
    pub internet: Measurements,
    pub traffic: Option<Traffic>,
    pub history: Vec<Traffic>,
}

type Counters = HashMap<String, (u64, u64)>;

pub struct Network<A, N, S, R, V = NoopCheck, T = NoopNotifier> {
    check: V,
    notifier: T,
    assessment: Assessment,
    files: A,
    nm: N,
    probe: S,
    clock: R,
    next: u64,
    next_ping: u64,
    connection: Connection,
    samples: VecDeque<(u64, Option<f64>, Option<f64>)>,
    previous: Option<(u64, Counters)>,
    history: VecDeque<Traffic>,
}
impl<A: Files, N: NetworkManager, S: Probe, R: Clock> Network<A, N, S, R> {
    pub fn new(files: A, nm: N, probe: S, clock: R) -> Self {
        Self {
            check: NoopCheck,
            notifier: NoopNotifier,
            assessment: Assessment::default(),
            files,
            nm,
            probe,
            clock,
            next: 0,
            next_ping: 0,
            connection: Connection::default(),
            samples: VecDeque::new(),
            previous: None,
            history: VecDeque::new(),
        }
    }
    pub fn with_security<V: Check, T: crate::ports::Notifier>(
        self,
        check: V,
        notifier: T,
    ) -> Network<A, N, S, R, V, T> {
        Network {
            files: self.files,
            nm: self.nm,
            probe: self.probe,
            clock: self.clock,
            next: self.next,
            next_ping: self.next_ping,
            connection: self.connection,
            samples: self.samples,
            previous: self.previous,
            history: self.history,
            check,
            notifier,
            assessment: self.assessment,
        }
    }
}
impl<A: Files, N: NetworkManager, S: Probe, R: Clock, V: Check, T: crate::ports::Notifier>
    Network<A, N, S, R, V, T>
{
    pub fn step(&mut self) -> Option<(u64, NetworkData)> {
        let now = self.clock.now();
        if now < self.next {
            return None;
        }
        self.next = now + 1000;
        // Query NetworkManager even while the panel is closed. D-Bus errors never reuse an old link.
        let connection = self.nm.connection().unwrap_or_default();
        if self.connection.id != connection.id
            || self.connection.interface != connection.interface
            || self.connection.gateway != connection.gateway
        {
            self.samples.clear();
            self.previous = None;
            self.history.clear();
            self.next_ping = 0;
        }
        let next = self.connection.id != connection.id
            || self.connection.interface != connection.interface;
        if next {
            self.assessment = Assessment::default();
        }
        self.connection = connection;
        let (security, reasons) =
            self.assessment
                .assess(&self.connection, now, &self.check, &self.nm, &self.notifier);
        let connected = self.connection.connected();
        let traffic = self.traffic(self.clock.now());
        if connected && now >= self.next_ping {
            self.next_ping = now + 5000;
            let gateway = self.connection.gateway.map(|ip| match ip {
                IpAddr::V6(ip) if ip.is_unicast_link_local() => {
                    let index = self
                        .files
                        .read(&format!(
                            "/sys/class/net/{}/ifindex",
                            self.connection.interface
                        ))
                        .ok()
                        .and_then(|s| s.trim().parse().ok())
                        .unwrap_or(0);
                    SocketAddr::V6(SocketAddrV6::new(ip, 0, 0, index))
                }
                _ => SocketAddr::new(ip, 0),
            });
            let internet: SocketAddr = if gateway.is_some_and(|g| g.is_ipv6()) {
                "[2606:4700:4700::1111]:0".parse().unwrap()
            } else {
                "1.1.1.1:0".parse().unwrap()
            };
            let (gateway, internet) = std::thread::scope(|scope| {
                let gateway = scope.spawn(|| gateway.and_then(|ip| self.probe.measure(ip)));
                let internet = self.probe.measure(internet);
                (gateway.join().unwrap_or(None), internet)
            });
            self.samples.push_back((now, gateway, internet));
        }
        while self
            .samples
            .front()
            .is_some_and(|a| now.saturating_sub(a.0) >= 120_000)
        {
            self.samples.pop_front();
        }
        let gateway = if self.connection.gateway.is_some() {
            measurements(self.samples.iter().map(|a| a.1))
        } else {
            Measurements::default()
        };
        let internet = measurements(self.samples.iter().map(|a| a.2));
        let quality =
            connected.then(|| classify(&gateway, &internet, self.connection.connectivity));
        if let Some(t) = &traffic {
            self.history.push_back(t.clone());
            while self.history.len() > 60 {
                self.history.pop_front();
            }
        }
        Some((
            now,
            NetworkData {
                connected,
                wired: self.connection.kind == "802-3-ethernet",
                vpn: self.connection.vpn,
                quality,
                security,
                reasons,
                gateway,
                internet,
                traffic,
                history: self.history.iter().cloned().collect(),
            },
        ))
    }
    fn traffic(&mut self, now: u64) -> Option<Traffic> {
        let text = match self.files.read("/proc/net/dev") {
            Ok(t) => t,
            Err(_) => {
                self.previous = None;
                return None;
            }
        };
        let current_counters: HashMap<_, _> = text
            .lines()
            .filter_map(|line| {
                let (name, numbers) = line.split_once(':')?;
                let values: Vec<_> = numbers.split_whitespace().collect();
                Some((
                    name.trim().to_string(),
                    (
                        values.first()?.parse::<u64>().ok()?,
                        values.get(8)?.parse::<u64>().ok()?,
                    ),
                ))
            })
            .collect();
        let previous = self.previous.replace((now, current_counters.clone()));
        let (when, previous) = previous?;
        if !self.connection.connected() {
            return None;
        }
        // Count each physical link once; exclude tunnels, VPN and loopback to avoid duplication.
        let name = &self.connection.interface;
        let (rx, tx) = current_counters.get(name)?;
        let (previous_rx, previous_tx) = previous.get(name)?;
        let seconds = now.checked_sub(when)? as f64 / 1000.0;
        if seconds <= 0.0 || rx < previous_rx || tx < previous_tx {
            return None;
        }
        Some(Traffic {
            download: (rx - previous_rx) as f64 / seconds,
            upload: (tx - previous_tx) as f64 / seconds,
        })
    }
}
fn measurements(samples: impl Iterator<Item = Option<f64>>) -> Measurements {
    let samples: Vec<_> = samples.collect();
    if samples.is_empty() {
        return Measurements::default();
    }
    let mut values: Vec<f64> = samples.iter().flatten().copied().collect();
    values.sort_by(f64::total_cmp);
    let n = values.len();
    let latency = if n == 0 {
        None
    } else if n.is_multiple_of(2) {
        Some((values[n / 2 - 1] + values[n / 2]) / 2.0)
    } else {
        Some(values[n / 2])
    };
    let variations: Vec<_> = samples
        .windows(2)
        .filter_map(|p| Some((p[1]? - p[0]?).abs()))
        .collect();
    Measurements {
        latency,
        jitter: (!variations.is_empty())
            .then(|| variations.iter().sum::<f64>() / variations.len() as f64),
        loss: Some((samples.len() - n) as f64 * 100.0 / samples.len() as f64),
    }
}
fn classify(g: &Measurements, i: &Measurements, connectivity: u32) -> Quality {
    let above = |v: Option<f64>, threshold| v.is_some_and(|v| v > threshold);
    if above(g.loss, 5.0)
        || above(i.loss, 5.0)
        || above(i.latency, 300.0)
        || (g.latency.is_some() && (i.latency.is_none() || matches!(connectivity, 1 | 3)))
    {
        Quality::Poor
    } else if above(g.loss, 1.0)
        || above(i.loss, 1.0)
        || above(i.latency, 100.0)
        || above(g.latency, 30.0)
        || above(g.jitter, 30.0)
        || above(i.jitter, 30.0)
    {
        Quality::Regular
    } else {
        Quality::Good
    }
}
pub fn start(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let mut network = Network::new(
            crate::ports::RootFilesystem::system(),
            NetworkManagerDbus::connect(),
            PingSocket,
            crate::ports::SystemClock,
        )
        .with_security(SystemCheck(SystemReadings), crate::ports::DesktopNotifier);
        loop {
            if let Some((when, data)) = network.step() {
                if let Some(handle) =
                    tauri::Manager::try_state::<ksni::Handle<crate::indicator::Indicator>>(&app)
                {
                    let risk = data.risk_badge();
                    tauri::async_runtime::block_on(
                        handle.update(move |i| i.state.risk_badge = risk),
                    );
                }
                crate::snapshot::publish(&app, snapshot(when, &data));
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}
fn snapshot(when: u64, data: &NetworkData) -> crate::snapshot::Snapshot {
    crate::snapshot::Snapshot {
        module: crate::module::Module::Network,
        updated_at: Some(when),
        data: serde_json::to_value(data).unwrap(),
    }
}
