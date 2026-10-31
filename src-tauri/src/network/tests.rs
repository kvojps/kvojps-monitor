use super::*;
use crate::ports::fakes::{FakeClock, FakeRoot};

use super::network_manager::{NetworkManagerProperties, NetworkManagerReading};
struct FakeDbus(NetworkManagerReading);
impl NetworkManagerProperties for FakeDbus {
    fn read(&self) -> std::io::Result<NetworkManagerReading> {
        Ok(self.0.clone())
    }
}
fn reading_nm() -> NetworkManagerReading {
    serde_json::from_str(include_str!("../../fixtures/network/networkmanager.json")).unwrap()
}
struct Nm;
impl NetworkManager for Nm {
    fn connection(&self) -> std::io::Result<Connection> {
        NetworkManagerDbus(FakeDbus(reading_nm())).connection()
    }
}
struct Ping(f64);
impl Probe for Ping {
    fn measure(&self, _: std::net::SocketAddr) -> Option<f64> {
        Some(self.0)
    }
}
#[test]
fn publishes_quality_with_injected_clock_while_panel_is_closed() {
    let clock = FakeClock::at(1000);
    let mut network = Network::new(FakeRoot::from("network"), Nm, Ping(20.0), clock.clone());
    let (time, data) = network.step().unwrap();
    assert_eq!(time, 1000);
    assert_eq!(data.quality, Some(Quality::Good));
    assert_eq!(data.internet.latency, Some(20.0));
    assert!(network.step().is_none());
    clock.advance(1000);
    assert!(network.step().is_some());
}

#[test]
fn no_replies_from_either_destination_means_poor_quality() {
    struct WithoutGateway;
    impl NetworkManager for WithoutGateway {
        fn connection(&self) -> std::io::Result<Connection> {
            let mut c = Nm.connection()?;
            c.gateway = None;
            Ok(c)
        }
    }
    struct WithoutResponse;
    impl Probe for WithoutResponse {
        fn measure(&self, _: std::net::SocketAddr) -> Option<f64> {
            None
        }
    }
    let mut network = Network::new(
        FakeRoot::from("network"),
        WithoutGateway,
        WithoutResponse,
        FakeClock::at(1000),
    );
    assert_eq!(network.step().unwrap().1.quality, Some(Quality::Poor));
}
#[test]
fn missing_gateway_does_not_count_as_packet_loss() {
    struct WithoutGateway;
    impl NetworkManager for WithoutGateway {
        fn connection(&self) -> std::io::Result<Connection> {
            let mut c = Nm.connection()?;
            c.gateway = None;
            Ok(c)
        }
    }
    let mut network = Network::new(
        FakeRoot::from("network"),
        WithoutGateway,
        Ping(20.0),
        FakeClock::at(1000),
    );
    let data = network.step().unwrap().1;
    assert_eq!(data.quality, Some(Quality::Good));
    assert_eq!(data.gateway.loss, None);
}

#[derive(Clone)]
struct FakeNetworkManager(std::sync::Arc<std::sync::Mutex<Connection>>);
impl NetworkManager for FakeNetworkManager {
    fn connection(&self) -> std::io::Result<Connection> {
        Ok(self.0.lock().unwrap().clone())
    }
}
#[derive(Clone)]
struct ProbeFake(std::sync::Arc<std::sync::Mutex<(Option<f64>, Option<f64>)>>);
impl Probe for ProbeFake {
    fn measure(&self, ip: std::net::SocketAddr) -> Option<f64> {
        let v = *self.0.lock().unwrap();
        if ip.ip() == "1.1.1.1".parse::<std::net::IpAddr>().unwrap() {
            v.1
        } else {
            v.0
        }
    }
}
struct Scenario {
    network: Network<FakeRoot, FakeNetworkManager, ProbeFake, FakeClock>,
    root: FakeRoot,
    nm: FakeNetworkManager,
    probe: ProbeFake,
    clock: FakeClock,
}
fn scenario() -> Scenario {
    let root = FakeRoot::from("network");
    let nm = FakeNetworkManager(std::sync::Arc::new(std::sync::Mutex::new(
        Nm.connection().unwrap(),
    )));
    let probe = ProbeFake(std::sync::Arc::new(std::sync::Mutex::new((
        Some(2.0),
        Some(20.0),
    ))));
    let clock = FakeClock::at(1_793_457_120_000);
    Scenario {
        network: Network::new(root.clone(), nm.clone(), probe.clone(), clock.clone()),
        root,
        nm,
        probe,
        clock,
    }
}
fn measure(c: &mut Scenario, g: Option<f64>, i: Option<f64>) -> NetworkData {
    *c.probe.0.lock().unwrap() = (g, i);
    let data = c.network.step().unwrap().1;
    c.clock.advance(5000);
    data
}
#[test]
fn latency_thresholds_are_strict() {
    for (gateway, internet, esperado) in [
        (30.0, 100.0, Quality::Good),
        (30.1, 100.0, Quality::Regular),
        (2.0, 100.1, Quality::Regular),
        (2.0, 300.0, Quality::Regular),
        (2.0, 300.1, Quality::Poor),
    ] {
        assert_eq!(
            measure(&mut scenario(), Some(gateway), Some(internet)).quality,
            Some(esperado)
        );
    }
}
#[test]
fn jitter_above_thirty_at_either_destination_means_regular() {
    for no_gateway in [true, false] {
        let mut c = scenario();
        measure(&mut c, Some(0.0), Some(20.0));
        let data = if no_gateway {
            measure(&mut c, Some(30.0), Some(20.0))
        } else {
            measure(&mut c, Some(0.0), Some(50.0))
        };
        assert_eq!(data.quality, Some(Quality::Good));
        let data = measure(&mut c, Some(0.0), Some(20.0));
        assert_eq!(data.quality, Some(Quality::Good));
        let data = if no_gateway {
            measure(&mut c, Some(31.0), Some(20.0))
        } else {
            measure(&mut c, Some(0.0), Some(51.0))
        };
        assert_eq!(data.quality, Some(Quality::Regular));
    }
}
#[test]
fn loss_between_one_and_five_is_regular_and_above_five_is_poor() {
    for no_gateway in [true, false] {
        let mut c = scenario();
        for _ in 0..19 {
            measure(&mut c, Some(2.0), Some(20.0));
        }
        let data = if no_gateway {
            measure(&mut c, None, Some(20.0))
        } else {
            measure(&mut c, Some(2.0), None)
        };
        assert_eq!(
            if no_gateway {
                data.gateway.loss
            } else {
                data.internet.loss
            },
            Some(5.0)
        );
        assert_eq!(data.quality, Some(Quality::Regular));
        let data = if no_gateway {
            measure(&mut c, None, Some(20.0))
        } else {
            measure(&mut c, Some(2.0), None)
        };
        assert_eq!(data.quality, Some(Quality::Poor));
    }
}
#[test]
fn rolling_window_expires_losses_after_two_minutes() {
    let mut c = scenario();
    measure(&mut c, None, None);
    for _ in 0..23 {
        measure(&mut c, Some(2.0), Some(20.0));
    }
    let data = measure(&mut c, Some(2.0), Some(20.0));
    assert_eq!(data.internet.loss, Some(0.0));
    assert_eq!(data.quality, Some(Quality::Good));
}
#[test]
fn median_and_jitter_use_rolling_window_with_five_second_pings() {
    let mut c = scenario();
    measure(&mut c, Some(2.0), Some(10.0));
    measure(&mut c, Some(2.0), Some(50.0));
    let data = measure(&mut c, Some(2.0), Some(20.0));
    assert_eq!(data.internet.latency, Some(20.0));
    assert_eq!(data.internet.jitter, Some(35.0));
    *c.probe.0.lock().unwrap() = (None, None);
    // measure already advanced five seconds; capture once and check the next four ticks.
    let previous = c.network.step().unwrap().1.internet;
    for _ in 0..4 {
        c.clock.advance(1000);
        assert_eq!(c.network.step().unwrap().1.internet, previous);
    }
}
#[test]
fn internet_failure_with_reachable_gateway_means_poor_quality() {
    let mut c = scenario();
    c.nm.0.lock().unwrap().connectivity = 1;
    assert_eq!(
        measure(&mut c, Some(2.0), Some(20.0)).quality,
        Some(Quality::Poor)
    );
}
#[test]
fn disconnect_and_network_change_reset_measurements_and_traffic() {
    let mut c = scenario();
    measure(&mut c, None, None);
    let connection = c.nm.0.lock().unwrap().clone();
    *c.nm.0.lock().unwrap() = Connection::default();
    let data = measure(&mut c, None, None);
    assert!(!data.connected);
    assert_eq!(data.quality, None);
    assert_eq!(data.internet.loss, None);
    assert!(data.history.is_empty());
    *c.nm.0.lock().unwrap() = connection;
    assert_eq!(
        measure(&mut c, Some(2.0), Some(20.0)).quality,
        Some(Quality::Good)
    );
}
#[test]
fn traffic_delta_excludes_loopback_and_tunnels_and_handles_counter_reset() {
    let mut c = scenario();
    assert!(c.network.step().unwrap().1.traffic.is_none());
    c.root.write("/proc/net/dev","wlo1: 818860497 0 0 0 0 0 0 0 122353707\nlo: 99999999 0 0 0 0 0 0 0 9999999\ntun0: 99999999 0 0 0 0 0 0 0 9999999\n");
    c.clock.advance(1000);
    let data = c.network.step().unwrap().1;
    assert_eq!(
        data.traffic,
        Some(Traffic {
            download: 2048.0,
            upload: 1024.0
        })
    );
    c.root.write("/proc/net/dev", "wlo1: 10 0 0 0 0 0 0 0 20\n");
    c.clock.advance(1000);
    assert!(c.network.step().unwrap().1.traffic.is_none());
}
#[test]
fn ui_fixtures_are_snapshots_published_by_core() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../src/lib/network/fixtures");
    for name in ["good", "poor", "disconnected", "vpn", "warning", "risk"] {
        let mut c = scenario();
        if name == "warning" {
            c.nm.0.lock().unwrap().connectivity = 2;
        }
        if name == "risk" {
            c.nm.0.lock().unwrap().wifi = Some(Wifi::default());
        }
        if name == "disconnected" {
            *c.nm.0.lock().unwrap() = Connection::default();
        }
        if name == "vpn" {
            let mut nm = c.nm.0.lock().unwrap();
            nm.vpn = true;
            nm.kind = "802-3-ethernet".into();
        }
        let internet = if name == "poor" { 301.0 } else { 20.0 };
        measure(&mut c, Some(2.0), Some(internet));
        c.root
            .write("/proc/net/dev", "wlo1: 818868689 0 0 0 0 0 0 0 122357803\n");
        c.network.step();
        c.root
            .write("/proc/net/dev", "wlo1: 818869713 0 0 0 0 0 0 0 122358315\n");
        c.clock.advance(1000);
        let (when, data) = c.network.step().unwrap();
        let json = serde_json::to_string_pretty(&snapshot(when, &data)).unwrap() + "\n";
        let file = dir.join(format!("{name}.json"));
        if std::env::var_os("UPDATE_FIXTURES").is_some() {
            std::fs::write(file, json).unwrap();
        } else {
            assert_eq!(std::fs::read_to_string(file).unwrap(), json);
        }
    }
}

#[test]
fn connectivity_change_preserves_window_on_same_network() {
    let mut c = scenario();
    measure(&mut c, None, None);
    c.nm.0.lock().unwrap().connectivity = 3;
    let data = measure(&mut c, Some(2.0), Some(20.0));
    assert_eq!(data.internet.loss, Some(50.0));
    assert_eq!(data.quality, Some(Quality::Poor));
}
#[test]
fn slow_probe_does_not_distort_traffic_rate() {
    use crate::ports::{Clock, Files, Space};
    struct FilesWithTraffic(FakeClock);
    impl Files for FilesWithTraffic {
        fn read(&self, _: &str) -> std::io::Result<String> {
            Ok(format!(
                "wlo1: {} 0 0 0 0 0 0 0 {}\n",
                self.0.now(),
                self.0.now() / 2
            ))
        }
        fn list(&self, _: &str) -> std::io::Result<Vec<String>> {
            unreachable!()
        }
        fn space(&self, _: &str) -> std::io::Result<Space> {
            unreachable!()
        }
    }
    struct SlowProbe(FakeClock);
    impl Probe for SlowProbe {
        fn measure(&self, ip: std::net::SocketAddr) -> Option<f64> {
            if ip.ip() == "1.1.1.1".parse::<std::net::IpAddr>().unwrap() {
                self.0.advance(700);
            }
            Some(20.0)
        }
    }
    let clock = FakeClock::at(1000);
    let mut network = Network::new(
        FilesWithTraffic(clock.clone()),
        Nm,
        SlowProbe(clock.clone()),
        clock.clone(),
    );
    network.step();
    clock.advance(300);
    assert_eq!(
        network.step().unwrap().1.traffic,
        Some(Traffic {
            download: 1000.0,
            upload: 500.0
        })
    );
}

#[test]
fn ipv6_link_local_gateway_preserves_interface_and_probes_ipv6_internet() {
    struct Ipv6Probe;
    impl Probe for Ipv6Probe {
        fn measure(&self, destination: std::net::SocketAddr) -> Option<f64> {
            match destination {
                std::net::SocketAddr::V6(v6) if v6.ip().is_unicast_link_local() => {
                    (v6.scope_id() == 7).then_some(2.0)
                }
                std::net::SocketAddr::V6(_) => Some(20.0),
                _ => None,
            }
        }
    }
    let c = scenario();
    c.root.write("/sys/class/net/wlo1/ifindex", "7\n");
    c.nm.0.lock().unwrap().gateway = Some("fe80::1".parse().unwrap());
    let mut network = Network::new(c.root, c.nm, Ipv6Probe, c.clock);
    let data = network.step().unwrap().1;
    assert_eq!(data.gateway.latency, Some(2.0));
    assert_eq!(data.internet.latency, Some(20.0));
    assert_eq!(data.quality, Some(Quality::Good));
}

#[test]
fn dbus_selects_physical_link_with_primary_vpn_and_ignores_loopback() {
    let mut reading = reading_nm();
    let mut vpn = reading.active[0].clone();
    vpn.path = "/org/freedesktop/NetworkManager/ActiveConnection/3".into();
    vpn.kind = "vpn".into();
    vpn.vpn = true;
    vpn.interface = "tun0".into();
    reading.primary = vpn.path.clone();
    reading.active.insert(0, vpn);
    let clock = FakeClock::at(1000);
    let mut network = Network::new(
        FakeRoot::from("network"),
        NetworkManagerDbus(FakeDbus(reading)),
        Ping(20.0),
        clock,
    );
    let data = network.step().unwrap().1;
    assert!(data.vpn);
    assert!(data.connected);
    assert_eq!(data.gateway.latency, Some(20.0));
    assert_eq!(data.quality, Some(Quality::Good));
}
#[test]
fn loopback_alone_is_not_an_external_connection() {
    let mut reading = reading_nm();
    reading.active.remove(0);
    reading.primary = "/".into();
    let mut network = Network::new(
        FakeRoot::from("network"),
        NetworkManagerDbus(FakeDbus(reading)),
        Ping(20.0),
        FakeClock::at(1000),
    );
    let data = network.step().unwrap().1;
    assert!(!data.connected);
    assert_eq!(data.quality, None);
}
#[test]
fn limited_connectivity_with_live_gateway_is_poor_despite_old_good_median() {
    let mut c = scenario();
    for _ in 0..23 {
        measure(&mut c, Some(2.0), Some(20.0));
    }
    c.nm.0.lock().unwrap().connectivity = 3;
    let data = measure(&mut c, Some(2.0), None);
    assert_eq!(data.internet.latency, Some(20.0));
    assert_eq!(data.quality, Some(Quality::Poor));
}

#[test]
fn risk_notifies_once_per_connection_and_disconnect_clears_badge() {
    let c = scenario();
    c.nm.0.lock().unwrap().wifi = Some(Wifi {
        flags: 0,
        wpa: 0,
        rsn: 0,
        key_mgmt: String::new(),
        pairwise: vec![],
    });
    let notices = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    struct FakeNotifier(std::sync::Arc<std::sync::Mutex<Vec<crate::ports::Notification>>>);
    impl crate::ports::Notifier for FakeNotifier {
        fn notify(&self, notice: crate::ports::Notification) {
            self.0.lock().unwrap().push(notice);
        }
    }
    let mut network = c
        .network
        .with_security(FakeCheck, FakeNotifier(notices.clone()));
    assert!(network.step().unwrap().1.risk_badge());
    c.clock.advance(1000);
    c.nm.0.lock().unwrap().wifi = None;
    assert!(!network.step().unwrap().1.risk_badge());
    c.clock.advance(1000);
    c.nm.0.lock().unwrap().wifi = Some(Wifi::default());
    assert!(network.step().unwrap().1.risk_badge());
    assert_eq!(notices.lock().unwrap().len(), 1);
    *c.nm.0.lock().unwrap() = Connection::default();
    c.clock.advance(1000);
    let data = network.step().unwrap().1;
    assert_eq!(data.security, None);
    assert!(!data.risk_badge());
    *c.nm.0.lock().unwrap() = Nm.connection().unwrap();
    c.nm.0.lock().unwrap().wifi = Some(Wifi::default());
    c.clock.advance(1000);
    network.step();
    assert_eq!(
        *notices.lock().unwrap(),
        vec![crate::ports::Notification::NetworkAtRisk; 2]
    );
}
struct FakeCheck;
impl Check for FakeCheck {
    fn dns(&self) -> Option<bool> {
        Some(false)
    }
    fn mac(&self, _: &Connection) -> Option<String> {
        None
    }
}

#[test]
fn physical_encryption_with_vpn_and_ethernet_respects_flags_and_profile() {
    for (flags, wpa, rsn, key, pairwise, esperado, reason) in [
        (0, 0, 0, "", "", Security::Risk, "Rede aberta sem OWE"),
        (1, 0, 0, "none", "", Security::Risk, "Criptografia WEP"),
        (1, 0x144, 0, "wpa-psk", "", Security::Risk, "WPA com TKIP"),
        (
            1,
            0,
            0x188,
            "wpa-psk",
            "tkip",
            Security::Risk,
            "WPA com TKIP",
        ),
        (0, 0, 0x800, "owe", "", Security::NoAlerts, ""),
        (1, 0, 0x188, "wpa-psk", "ccmp", Security::NoAlerts, ""),
        (1, 0, 0x400, "sae", "", Security::NoAlerts, ""),
    ] {
        let mut c = scenario();
        {
            let mut nm = c.nm.0.lock().unwrap();
            nm.vpn = true;
            nm.wifi = Some(Wifi {
                flags,
                wpa,
                rsn,
                key_mgmt: key.into(),
                pairwise: vec![pairwise.into()],
            });
        }
        let data = c.network.step().unwrap().1;
        assert_eq!(data.security, Some(esperado));
        if !reason.is_empty() {
            assert!(data.reasons.contains(&reason.to_string()));
        }
        c.nm.0.lock().unwrap().kind = "802-3-ethernet".into();
        c.clock.advance(1000);
        assert_eq!(
            c.network.step().unwrap().1.security,
            Some(Security::NoAlerts)
        );
    }
}

#[derive(Clone)]
struct ControlledCheck {
    dns: std::sync::Arc<std::sync::Mutex<(Option<bool>, usize)>>,
    mac: std::sync::Arc<std::sync::Mutex<Option<String>>>,
}
impl Check for ControlledCheck {
    fn dns(&self) -> Option<bool> {
        let mut d = self.dns.lock().unwrap();
        d.1 += 1;
        d.0
    }
    fn mac(&self, _: &Connection) -> Option<String> {
        self.mac.lock().unwrap().clone()
    }
}
#[test]
fn dns_and_portal_refresh_every_ten_minutes_and_mac_alert_persists() {
    let c = scenario();
    let v = ControlledCheck {
        dns: std::sync::Arc::new(std::sync::Mutex::new((Some(false), 0))),
        mac: std::sync::Arc::new(std::sync::Mutex::new(Some("aa:bb:cc:dd:ee:ff".into()))),
    };
    let mut network = c.network.with_security(v.clone(), NoopNotifier);
    assert_eq!(network.step().unwrap().1.security, Some(Security::NoAlerts));
    *v.mac.lock().unwrap() = Some("11:22:33:44:55:66".into());
    c.nm.0.lock().unwrap().revision += 1;
    c.clock.advance(1000);
    assert_eq!(network.step().unwrap().1.security, Some(Security::Warning));
    assert_eq!(v.dns.lock().unwrap().1, 1);
    v.dns.lock().unwrap().0 = Some(true);
    c.clock.advance(599_000);
    let data = network.step().unwrap().1;
    assert_eq!(data.security, Some(Security::Risk));
    assert!(data
        .reasons
        .contains(&"DNS respondeu a um nome inexistente".into()));
    assert_eq!(v.dns.lock().unwrap().1, 2);
    c.nm.0.lock().unwrap().id.push('2');
    v.dns.lock().unwrap().0 = Some(false);
    c.nm.0.lock().unwrap().connectivity = 2;
    c.clock.advance(1000);
    let data = network.step().unwrap().1;
    assert_eq!(data.security, Some(Security::Warning));
    assert_eq!(data.reasons, vec!["Portal cativo detectado"]);
    assert_eq!(v.dns.lock().unwrap().1, 3);
}

#[derive(Clone)]
struct RecordedReadings {
    dns: DnsResponse,
    neighbors: std::sync::Arc<std::sync::Mutex<String>>,
}
impl SecurityReadings for RecordedReadings {
    fn resolver(&self) -> Option<DnsResponse> {
        Some(self.dns.clone())
    }
    fn neighbors(&self, _: &Connection) -> Option<String> {
        Some(self.neighbors.lock().unwrap().clone())
    }
}
#[test]
fn recorded_responses_pass_through_adapters_and_failures_do_not_imply_risk() {
    let original: DnsResponse =
        serde_json::from_str(include_str!("../../fixtures/network/dns.json")).unwrap();
    for (answered, error, esperado) in [
        (original.answered, original.error, Security::NoAlerts),
        (
            false,
            Some("org.freedesktop.resolve1.DnsError.NXDOMAIN".into()),
            Security::NoAlerts,
        ),
        (true, None, Security::Risk),
        (
            false,
            Some("org.freedesktop.resolve1.Timeout".into()),
            Security::NoAlerts,
        ),
    ] {
        let c = scenario();
        let readings = RecordedReadings {
            dns: DnsResponse { answered, error },
            neighbors: std::sync::Arc::new(std::sync::Mutex::new(
                include_str!("../../fixtures/network/neighbors.json").into(),
            )),
        };
        let neighbors = readings.neighbors.clone();
        let mut network = c.network.with_security(SystemCheck(readings), NoopNotifier);
        assert_eq!(network.step().unwrap().1.security, Some(esperado));
        // Controlled recording variation: MAC changed on a NetworkManager event.
        let mut text = neighbors.lock().unwrap();
        *text = text.replace("02:00:00:00:00:01", "11:22:33:44:55:66");
        drop(text);
        c.nm.0.lock().unwrap().revision += 1;
        c.clock.advance(1000);
        assert!(network
            .step()
            .unwrap()
            .1
            .reasons
            .contains(&"MAC do gateway mudou nesta conexão".into()));
    }
}

#[test]
fn portal_recovers_on_event_and_mac_is_checked_only_on_event() {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    struct CountingCheck(Arc<AtomicUsize>);
    impl Check for CountingCheck {
        fn dns(&self) -> Option<bool> {
            Some(false)
        }
        fn mac(&self, _: &Connection) -> Option<String> {
            self.0.fetch_add(1, Ordering::SeqCst);
            None
        }
    }
    let c = scenario();
    c.nm.0.lock().unwrap().connectivity = 2;
    let calls = Arc::new(AtomicUsize::new(0));
    let mut network = c
        .network
        .with_security(CountingCheck(calls.clone()), NoopNotifier);
    assert_eq!(network.step().unwrap().1.security, Some(Security::Warning));
    for _ in 0..5 {
        c.clock.advance(1000);
        network.step();
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    {
        let mut nm = c.nm.0.lock().unwrap();
        nm.connectivity = 4;
        nm.revision += 1;
    }
    c.clock.advance(1000);
    assert_eq!(network.step().unwrap().1.security, Some(Security::NoAlerts));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

type SpeedState = (Vec<bool>, Option<speed::Output>);
#[derive(Clone, Default)]
struct FakeSpeed(std::sync::Arc<std::sync::Mutex<SpeedState>>);
impl speed::Transport for FakeSpeed {
    fn cancel(&mut self) {
        self.0.lock().unwrap().1 = Some(Err(()));
    }
    fn start(&mut self, full: bool) {
        self.0.lock().unwrap().0.push(full);
    }
    fn poll(&mut self) -> Option<Result<(f64, f64), ()>> {
        self.0.lock().unwrap().1.take()
    }
}
#[test]
fn automatic_speed_runs_on_connect_and_after_six_hours_but_never_metered() {
    let c = scenario();
    c.nm.0.lock().unwrap().metered = false;
    let http = FakeSpeed::default();
    let mut network = c
        .network
        .with_speed(http.clone(), crate::persistence::Volatile);
    assert!(network.step().unwrap().1.speed_running);
    http.0.lock().unwrap().1 = Some(Ok((1.5, 4.0)));
    c.clock.advance(1000);
    let data = network.step().unwrap().1;
    assert_eq!(data.quality, Some(Quality::Poor));
    assert_eq!(data.speed.unwrap().download, 1.5);
    c.clock.advance(21_598_000);
    network.step();
    assert_eq!(http.0.lock().unwrap().0.len(), 1);
    c.nm.0.lock().unwrap().metered = true;
    c.clock.advance(1000);
    network.step();
    assert_eq!(http.0.lock().unwrap().0.len(), 1);
    c.nm.0.lock().unwrap().metered = false;
    c.clock.advance(1000);
    network.step();
    assert_eq!(http.0.lock().unwrap().0, vec![false, false]);
}

#[test]
fn speed_cache_restores_only_numeric_result_and_age_even_offline() {
    use crate::persistence::{Cache, DiskStorage, LocalPersistence, Logs};
    let dir = tempfile::tempdir().unwrap();
    let cache = Cache(DiskStorage(dir.path().into()));
    cache
        .save(&crate::snapshot::Snapshot {
            module: crate::module::Module::Network,
            updated_at: Some(1000),
            data: serde_json::json!({"speed": {"download": 8.0, "upload": 3.0, "measuredAt":1000,
            "networkId":"known-network", "token":"secret", "prompt":"private"},
            "token":"secret", "history":[123]}),
        })
        .unwrap();
    let bytes = std::fs::read_to_string(dir.path().join("network.json")).unwrap();
    for forbidden in ["secret", "private", "token", "history", "prompt"] {
        assert!(!bytes.contains(forbidden));
    }
    let c = scenario();
    *c.nm.0.lock().unwrap() = Connection::default();
    let mut network = c.network.with_speed(
        FakeSpeed::default(),
        LocalPersistence {
            cache,
            logs: Logs(DiskStorage(dir.path().join("logs"))),
        },
    );
    let data = network.step().unwrap().1;
    assert!(!data.connected);
    assert_eq!(data.quality, None);
    assert_eq!(data.speed.unwrap().measured_at, 1000);
}

#[test]
fn speed_influences_quality_only_on_current_network_for_less_than_six_hours() {
    for (download, expected) in [
        (1.99, Quality::Poor),
        (2.0, Quality::Regular),
        (9.99, Quality::Regular),
        (10.0, Quality::Good),
    ] {
        let c = scenario();
        c.nm.0.lock().unwrap().metered = false;
        let http = FakeSpeed::default();
        let mut network = c
            .network
            .with_speed(http.clone(), crate::persistence::Volatile);
        network.step();
        http.0.lock().unwrap().1 = Some(Ok((download, 5.0)));
        c.clock.advance(1000);
        assert_eq!(network.step().unwrap().1.quality, Some(expected));
        network.automatic_speed(false);
        c.clock.advance(speed::INTERVAL - 1000);
        assert_eq!(network.step().unwrap().1.quality, Some(expected));
        c.clock.advance(1000);
        let data = network.step().unwrap().1;
        assert_eq!(data.quality, Some(Quality::Good));
        assert_eq!(data.speed.unwrap().download, download);
    }
    let c = scenario();
    c.nm.0.lock().unwrap().metered = false;
    let http = FakeSpeed::default();
    let mut network = c
        .network
        .with_speed(http.clone(), crate::persistence::Volatile);
    network.step();
    http.0.lock().unwrap().1 = Some(Ok((1.0, 5.0)));
    c.clock.advance(1000);
    network.step();
    c.nm.0.lock().unwrap().id.push('2');
    c.clock.advance(1000);
    let data = network.step().unwrap().1;
    assert_eq!(data.quality, Some(Quality::Good));
    assert!(data.speed.is_some());
}

#[test]
fn manual_test_is_full_and_duplicate_requests_do_not_overlap() {
    let c = scenario();
    c.nm.0.lock().unwrap().metered = false;
    let http = FakeSpeed::default();
    let mut network = c
        .network
        .with_speed(http.clone(), crate::persistence::Volatile);
    network.automatic_speed(false);
    network.step();
    assert!(http.0.lock().unwrap().0.is_empty());
    network.request_speed();
    c.clock.advance(1000);
    assert!(network.step().unwrap().1.speed_running);
    network.request_speed();
    c.clock.advance(1000);
    network.step();
    assert_eq!(http.0.lock().unwrap().0, vec![true]);
    c.nm.0.lock().unwrap().id.push('2');
    http.0.lock().unwrap().1 = Some(Ok((1.0, 5.0)));
    c.clock.advance(1000);
    let data = network.step().unwrap().1;
    assert!(data.speed.is_none());
    assert!(!data.speed_running);
    c.nm.0.lock().unwrap().metered = true;
    network.request_speed();
    c.clock.advance(1000);
    assert!(!network.step().unwrap().1.speed_running);
    assert_eq!(http.0.lock().unwrap().0, vec![true]);
}

#[test]
fn automatic_preference_defaults_on_and_survives_restart() {
    use crate::persistence::{DiskStorage, Volatile};
    let dir = tempfile::tempdir().unwrap();
    let c = scenario();
    let mut network = c
        .network
        .with_speed(FakeSpeed::default(), Volatile)
        .with_preferences(DiskStorage(dir.path().into()));
    assert!(network.step().unwrap().1.automatic_speed);
    network.automatic_speed(false);
    let mut network = scenario()
        .network
        .with_speed(FakeSpeed::default(), Volatile)
        .with_preferences(DiskStorage(dir.path().into()));
    assert!(!network.step().unwrap().1.automatic_speed);
}

#[test]
fn recorded_http_transfers_publish_mbps_and_respect_light_and_full_budgets() {
    struct RecordedHttp {
        full: bool,
    }
    impl speed::Http for RecordedHttp {
        async fn transfer(&self, upload: bool, bytes: usize) -> Result<(u64, f64), ()> {
            let expected = match (self.full, upload) {
                (false, false) => 25_000_000,
                (false, true) => 10_000_000,
                (true, false) => 100_000_000,
                (true, true) => 50_000_000,
            };
            assert_eq!(bytes, expected);
            let fixture: serde_json::Value =
                serde_json::from_str(include_str!("../../fixtures/network/speed.json")).unwrap();
            let transfer = &fixture[if upload { "upload" } else { "download" }];
            Ok((
                transfer["bytes"].as_u64().unwrap(),
                transfer["seconds"].as_f64().unwrap(),
            ))
        }
    }
    for full in [false, true] {
        let c = scenario();
        c.nm.0.lock().unwrap().metered = false;
        let mut network = c.network.with_speed(
            speed::Immediate {
                http: RecordedHttp { full },
                result: None,
            },
            crate::persistence::Volatile,
        );
        if full {
            network.request_speed();
        }
        network.step();
        c.clock.advance(1000);
        let result = network.step().unwrap().1.speed.unwrap();
        assert!((result.download - 0.20966).abs() < 0.00001);
        assert!((result.upload - 0.26627).abs() < 0.00001);
    }
}

#[test]
fn speed_ui_fixtures_are_published_by_core() {
    for name in ["speed", "speed-offline", "speed-running", "speed-failed"] {
        let c = scenario();
        c.nm.0.lock().unwrap().metered = false;
        let http = FakeSpeed::default();
        let mut network = c
            .network
            .with_speed(http.clone(), crate::persistence::Volatile);
        let mut result = network.step().unwrap();
        if name != "speed-running" {
            http.0.lock().unwrap().1 = Some(if name == "speed-failed" {
                Err(())
            } else {
                Ok((8.0, 3.0))
            });
            c.clock.advance(1000);
            result = network.step().unwrap();
        }
        if name == "speed-offline" {
            *c.nm.0.lock().unwrap() = Connection::default();
            c.clock.advance(1000);
            result = network.step().unwrap();
        }
        let json = serde_json::to_string_pretty(&snapshot(result.0, &result.1)).unwrap() + "\n";
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../src/lib/network/fixtures/{name}.json"));
        if std::env::var_os("UPDATE_FIXTURES").is_some() {
            std::fs::write(path, json).unwrap();
        } else {
            assert_eq!(std::fs::read_to_string(path).unwrap(), json);
        }
    }
}

#[test]
fn dbus_metering_blocks_yes_guessed_yes_and_unknown() {
    for metered in 0..=4 {
        let mut reading = reading_nm();
        reading.active[0].metered = metered;
        let http = FakeSpeed::default();
        let mut network = Network::new(
            FakeRoot::from("network"),
            NetworkManagerDbus(FakeDbus(reading)),
            Ping(20.0),
            FakeClock::at(1000),
        )
        .with_speed(http.clone(), crate::persistence::Volatile);
        let data = network.step().unwrap().1;
        assert_eq!(data.speed_running, matches!(metered, 2 | 4));
        assert_eq!(data.speed_available, matches!(metered, 2 | 4));
    }
}

#[test]
fn in_flight_speed_is_cancelled_when_connection_becomes_metered() {
    let c = scenario();
    c.nm.0.lock().unwrap().metered = false;
    let http = FakeSpeed::default();
    let mut network = c
        .network
        .with_speed(http.clone(), crate::persistence::Volatile);
    assert!(network.step().unwrap().1.speed_running);
    c.nm.0.lock().unwrap().metered = true;
    c.clock.advance(1000);
    let data = network.step().unwrap().1;
    assert!(!data.speed_running);
    assert!(data.speed.is_none());
    assert!(!data.speed_failed);
}

#[test]
fn switching_vpn_profiles_invalidates_speed_and_runs_new_light_test() {
    let c = scenario();
    let mut reading = reading_nm();
    reading.active[0].metered = 2;
    reading.active[0].network_id = "physical-profile".into();
    let mut vpn = reading.active[0].clone();
    vpn.vpn = true;
    vpn.path.push('3');
    vpn.network_id = "vpn-a".into();
    reading.primary = vpn.path.clone();
    reading.active.push(vpn);
    *c.nm.0.lock().unwrap() = NetworkManagerDbus(FakeDbus(reading.clone()))
        .connection()
        .unwrap();
    let http = FakeSpeed::default();
    let mut network = c
        .network
        .with_speed(http.clone(), crate::persistence::Volatile);
    network.step();
    http.0.lock().unwrap().1 = Some(Ok((1.0, 5.0)));
    c.clock.advance(1000);
    assert_eq!(network.step().unwrap().1.quality, Some(Quality::Poor));
    // Recreated D-Bus object paths must not invalidate a result for the same profiles.
    reading.active[0].path.push('4');
    reading.active.last_mut().unwrap().path.push('4');
    reading.primary = reading.active.last().unwrap().path.clone();
    *c.nm.0.lock().unwrap() = NetworkManagerDbus(FakeDbus(reading.clone()))
        .connection()
        .unwrap();
    network.automatic_speed(false);
    c.clock.advance(1000);
    assert_eq!(network.step().unwrap().1.quality, Some(Quality::Poor));
    network.automatic_speed(true);
    reading.active.last_mut().unwrap().network_id = "vpn-b".into();
    *c.nm.0.lock().unwrap() = NetworkManagerDbus(FakeDbus(reading)).connection().unwrap();
    c.clock.advance(1000);
    let data = network.step().unwrap().1;
    assert_eq!(data.quality, Some(Quality::Good));
    assert!(data.speed.is_some());
    assert!(data.speed_running);
    assert_eq!(http.0.lock().unwrap().0, vec![false, false]);
}

#[test]
fn metered_transition_drops_a_pending_http_future_without_waiting_for_timeout() {
    struct DropSignal(std::sync::mpsc::Sender<()>);
    impl Drop for DropSignal {
        fn drop(&mut self) {
            let _ = self.0.send(());
        }
    }
    struct PendingHttp {
        started: std::sync::mpsc::Sender<()>,
        cancelled: std::sync::mpsc::Sender<()>,
    }
    impl speed::Http for PendingHttp {
        async fn transfer(&self, _: bool, _: usize) -> Result<(u64, f64), ()> {
            let _guard = DropSignal(self.cancelled.clone());
            self.started.send(()).unwrap();
            std::future::pending().await
        }
    }
    let (started, began) = std::sync::mpsc::channel();
    let (cancelled, dropped) = std::sync::mpsc::channel();
    let c = scenario();
    c.nm.0.lock().unwrap().metered = false;
    let mut network = c.network.with_speed(
        speed::Worker::new(PendingHttp { started, cancelled }),
        crate::persistence::Volatile,
    );
    network.step();
    began
        .recv_timeout(std::time::Duration::from_secs(2))
        .unwrap();
    c.nm.0.lock().unwrap().metered = true;
    c.clock.advance(1000);
    network.step();
    dropped
        .recv_timeout(std::time::Duration::from_secs(2))
        .unwrap();
}

fn recorded_local_address() -> [u8; 4] {
    reading_nm().active[0]
        .addresses
        .iter()
        .find_map(|ip| match ip {
            std::net::IpAddr::V4(ip) => Some(ip.octets()),
            _ => None,
        })
        .unwrap()
}
#[derive(Clone)]
struct RecordedTcp(std::sync::Arc<std::sync::Mutex<usize>>);
impl super::process::Diagnostics for RecordedTcp {
    fn uid(&self) -> u32 {
        1000
    }
    fn dump(&self) -> std::io::Result<Vec<u8>> {
        let index = *self.0.lock().unwrap();
        assert_ne!(index, 98, "hidden module must not request diagnostics");
        if index == 99 {
            return Err(std::io::ErrorKind::PermissionDenied.into());
        }
        let mut bytes = std::fs::read(
            crate::ports::fakes::fixtures().join(format!("network/tcp/{index}.bin")),
        )?;
        // Replay the recorded socket on a non-loopback address with a stable test owner.
        bytes[24..28].copy_from_slice(&recorded_local_address());
        bytes[40..44].copy_from_slice(&[192, 0, 2, 2]);
        bytes[80..84].copy_from_slice(&1000u32.to_ne_bytes());
        Ok(bytes)
    }
}
#[test]
fn publishes_process_rates_only_while_visible_using_recorded_tcp_counters() {
    let clock = FakeClock::at(1000);
    let root = FakeRoot::from("network");
    let owner: serde_json::Value =
        serde_json::from_str(include_str!("../../fixtures/network/tcp/owner.json")).unwrap();
    root.write(
        "/proc/123/status",
        include_str!("../../fixtures/network/tcp/status.txt"),
    );
    root.write(
        "/proc/123/comm",
        &include_str!("../../fixtures/network/tcp/comm.txt").replace(
            include_str!("../../fixtures/network/tcp/comm.txt").trim(),
            "browser",
        ),
    );
    root.write("/proc/123/fd/4", "");
    std::os::unix::fs::symlink(
        owner["fdLink"].as_str().unwrap(),
        root.dir.path().join("proc/123/fd/5"),
    )
    .unwrap();
    let index = std::sync::Arc::new(std::sync::Mutex::new(98));
    let mut network = Network::new(root.clone(), Nm, Ping(20.0), clock.clone())
        .with_processes(RecordedTcp(index.clone()));
    assert!(network.step().unwrap().1.process_traffic.is_none());
    *index.lock().unwrap() = 0;
    network.process_visible(true);
    clock.advance(1000);
    assert!(network.step().unwrap().1.process_traffic.is_none());
    *index.lock().unwrap() = 1;
    root.write(
        "/proc/net/dev",
        "wlo1: 818878449 0 0 0 0 0 0 0 122362683 0 0 0 0 0 0 0\n",
    );
    clock.advance(2000);
    let data = network.step().unwrap().1;
    let traffic = data.process_traffic.unwrap();
    assert_eq!(traffic.apps[0].name, "browser");
    assert_eq!(traffic.apps[0].download, 2048.0);
    assert_eq!(traffic.apps[0].upload, 1024.0);
    assert_eq!(data.quality, Some(Quality::Good));
    network.process_visible(false);
    *index.lock().unwrap() = 98;
    clock.advance(2000);
    assert!(network.step().unwrap().1.process_traffic.is_none());
    *index.lock().unwrap() = 99;
    network.process_visible(true);
    clock.advance(1000);
    assert!(network.step().unwrap().1.process_traffic.is_none());
}

#[derive(Clone)]
struct TcpReplies(std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>);
impl super::process::Diagnostics for TcpReplies {
    fn uid(&self) -> u32 {
        1000
    }
    fn dump(&self) -> std::io::Result<Vec<u8>> {
        self.0
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| std::io::ErrorKind::PermissionDenied.into())
    }
}
fn socket_reply(index: usize, inode: u32, cookie: u64, uid: u32) -> Vec<u8> {
    let mut bytes =
        std::fs::read(crate::ports::fakes::fixtures().join(format!("network/tcp/{index}.bin")))
            .unwrap();
    bytes[24..28].copy_from_slice(&recorded_local_address());
    bytes[40..44].copy_from_slice(&[192, 0, 2, 2]);
    bytes[60..68].copy_from_slice(&cookie.to_ne_bytes());
    bytes[80..84].copy_from_slice(&uid.to_ne_bytes());
    bytes[84..88].copy_from_slice(&inode.to_ne_bytes());
    bytes
}
fn process_owner(root: &FakeRoot, pid: u32, uid: u32, inode: u32, name: &str) {
    let status = include_str!("../../fixtures/network/tcp/status.txt")
        .lines()
        .map(|line| {
            if line.starts_with("Uid:") {
                format!("Uid:\t{uid}\t{uid}\t{uid}\t{uid}")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    root.write(&format!("/proc/{pid}/status"), &status);
    root.write(
        &format!("/proc/{pid}/comm"),
        include_str!("../../fixtures/network/tcp/comm.txt"),
    );
    root.write(&format!("/proc/{pid}/fd/0"), "");
    let owner: serde_json::Value =
        serde_json::from_str(include_str!("../../fixtures/network/tcp/owner.json")).unwrap();
    let fd_link = owner["fdLink"]
        .as_str()
        .unwrap()
        .replace(&owner["inode"].to_string(), &inode.to_string());
    std::os::unix::fs::symlink(fd_link, root.dir.path().join(format!("proc/{pid}/fd/3"))).unwrap();
    std::os::unix::fs::symlink(
        std::path::Path::new(owner["exeLink"].as_str().unwrap()).with_file_name(name),
        root.dir.path().join(format!("proc/{pid}/exe")),
    )
    .unwrap();
}
#[test]
fn groups_shared_app_names_and_deduplicates_shared_sockets_before_selecting_top_five() {
    let root = FakeRoot::from("network");
    let clock = FakeClock::at(1000);
    let replies = TcpReplies(Default::default());
    let mut network =
        Network::new(root.clone(), Nm, Ping(20.0), clock.clone()).with_processes(replies.clone());
    for pid in 1..=8 {
        process_owner(
            &root,
            pid,
            1000,
            pid,
            if pid <= 2 {
                "browser"
            } else {
                [
                    "", "", "", "alpha", "beta", "gamma", "delta", "epsilon", "zeta",
                ][pid as usize]
            },
        );
    }
    process_owner(&root, 9, 1000, 1, "browser"); // Shared FD must count once.
    process_owner(&root, 10, 0, 10, "system");
    let dump = |index| {
        (1..=10)
            .flat_map(|inode| {
                socket_reply(
                    index,
                    inode,
                    inode as u64,
                    if inode == 10 { 0 } else { 1000 },
                )
            })
            .collect::<Vec<_>>()
    };
    *replies.0.lock().unwrap() = Some(dump(0));
    network.process_visible(true);
    network.step();
    *replies.0.lock().unwrap() = Some(dump(1));
    root.write(
        "/proc/net/dev",
        "wlo1: 818958449 0 0 0 0 0 0 0 122402683 0 0 0 0 0 0 0\n",
    );
    clock.advance(2000);
    let data = network.step().unwrap().1;
    let traffic = data.process_traffic.unwrap();
    assert_eq!(
        traffic
            .apps
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>(),
        vec!["browser", "alpha", "beta", "delta", "epsilon"]
    );
    assert_eq!(traffic.apps[0].download, 4096.0);
    assert_eq!(traffic.apps[0].upload, 2048.0);
    // Eight owned sockets; apps outside the top five still count as attributed.
    assert_eq!(traffic.others.download, 33616.0);
    assert_eq!(traffic.others.upload, 16808.0);
    assert!(!serde_json::to_string(&traffic).unwrap().contains("192.0.2"));
    assert!(!serde_json::to_string(&traffic).unwrap().contains("system"));
}
#[test]
fn averages_the_same_window_for_apps_and_others_then_expires_old_activity() {
    let root = FakeRoot::from("network");
    process_owner(&root, 1, 1000, 1, "browser");
    let clock = FakeClock::at(1000);
    let replies = TcpReplies(Default::default());
    let mut network =
        Network::new(root.clone(), Nm, Ping(20.0), clock.clone()).with_processes(replies.clone());
    *replies.0.lock().unwrap() = Some(socket_reply(0, 1, 1, 1000));
    network.process_visible(true);
    network.step();
    *replies.0.lock().unwrap() = Some(socket_reply(1, 1, 1, 1000));
    root.write(
        "/proc/net/dev",
        "wlo1: 818866641 0 0 0 0 0 0 0 122356779 0 0 0 0 0 0 0\n",
    );
    clock.advance(2000);
    let data = network.step().unwrap().1.process_traffic.unwrap();
    assert_eq!(data.apps[0].download, 2048.0);
    assert_eq!(data.others.download, 2048.0);
    clock.advance(1000);
    // New counters aren't sampled until the 2 s deadline.
    *replies.0.lock().unwrap() = None;
    assert_eq!(
        network.step().unwrap().1.process_traffic.unwrap().apps[0].download,
        2048.0
    );
    *replies.0.lock().unwrap() = Some(socket_reply(1, 1, 1, 1000));
    clock.advance(1000);
    let data = network.step().unwrap().1.process_traffic.unwrap();
    assert_eq!(data.apps[0].download, 1024.0);
    assert_eq!(data.others.download, 1024.0);
    clock.advance(2000);
    network.step();
    clock.advance(2000);
    network.step();
    clock.advance(2000);
    let data = network.step().unwrap().1.process_traffic.unwrap();
    assert!(data.apps.is_empty());
    assert_eq!(data.others.download, 0.0);
}
#[test]
fn inaccessible_or_malformed_diagnostics_clear_attribution_without_affecting_quality() {
    let root = FakeRoot::from("network");
    process_owner(&root, 1, 1000, 1, "browser");
    let clock = FakeClock::at(1000);
    let replies = TcpReplies(Default::default());
    let mut network =
        Network::new(root, Nm, Ping(20.0), clock.clone()).with_processes(replies.clone());
    network.process_visible(true);
    for bytes in [None, Some(vec![1, 2, 3]), Some(socket_reply(0, 1, 1, 1000))] {
        *replies.0.lock().unwrap() = bytes;
        let data = network.step().unwrap().1;
        assert!(data.process_traffic.is_none());
        assert_eq!(data.quality, Some(Quality::Good));
        clock.advance(2000);
    }
}

#[test]
fn process_ui_fixture_is_a_snapshot_published_by_core() {
    for (name, index) in [("processes", 1), ("processes-empty", 0)] {
        let root = FakeRoot::from("network");
        let clock = FakeClock::at(1_793_457_126_000);
        let replies = TcpReplies(Default::default());
        let mut network = Network::new(root.clone(), Nm, Ping(20.0), clock.clone())
            .with_processes(replies.clone());
        process_owner(&root, 1, 1000, 1, "firefox");
        process_owner(&root, 2, 1000, 2, "firefox");
        process_owner(&root, 3, 1000, 3, "codex");
        *replies.0.lock().unwrap() = Some(
            (1..=3)
                .flat_map(|i| socket_reply(0, i, i as u64, 1000))
                .collect(),
        );
        network.process_visible(true);
        network.step();
        *replies.0.lock().unwrap() = Some(
            (1..=3)
                .flat_map(|i| socket_reply(index, i, i as u64, 1000))
                .collect(),
        );
        root.write(
            "/proc/net/dev",
            "wlo1: 818872785 0 0 0 0 0 0 0 122359851 0 0 0 0 0 0 0\n",
        );
        clock.advance(2000);
        let (when, data) = network.step().unwrap();
        let json = serde_json::to_string_pretty(&snapshot(when, &data)).unwrap() + "\n";
        let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../src/lib/network/fixtures/{name}.json"));
        if std::env::var_os("UPDATE_FIXTURES").is_some() {
            std::fs::write(file, json).unwrap();
        } else {
            assert_eq!(std::fs::read_to_string(file).unwrap(), json);
        }
    }
}

#[test]
fn excludes_loopback_foreign_users_and_reused_or_new_sockets() {
    let root = FakeRoot::from("network");
    let clock = FakeClock::at(1000);
    let replies = TcpReplies(Default::default());
    for pid in 1..=7 {
        process_owner(
            &root,
            pid,
            if pid == 3 { 2000 } else { 1000 },
            pid,
            &format!("app{pid}"),
        );
    }
    let mut network =
        Network::new(root.clone(), Nm, Ping(20.0), clock.clone()).with_processes(replies.clone());
    let dump = |index| {
        let mut bytes = Vec::new();
        for inode in 1..=7 {
            let mut socket = socket_reply(
                index,
                inode,
                if index == 1 && inode == 4 {
                    444
                } else {
                    inode as u64
                },
                if inode == 2 { 2000 } else { 1000 },
            );
            if inode == 5 {
                socket[24..28].copy_from_slice(&[127, 0, 0, 1]);
            }
            if inode == 6 && index == 0 {
                continue;
            }
            if inode == 7 {
                // IPv6 loopback uses the same inet_diag_msg layout.
                socket[16] = 10;
                socket[24..40].copy_from_slice(&std::net::Ipv6Addr::LOCALHOST.octets());
                socket[40..56].copy_from_slice(&std::net::Ipv6Addr::LOCALHOST.octets());
            }
            bytes.extend(socket);
        }
        bytes
    };
    *replies.0.lock().unwrap() = Some(dump(0));
    network.process_visible(true);
    network.step();
    *replies.0.lock().unwrap() = Some(dump(1));
    root.write(
        "/proc/net/dev",
        "wlo1: 818878449 0 0 0 0 0 0 0 122362683 0 0 0 0 0 0 0\n",
    );
    clock.advance(2000);
    let data = network.step().unwrap().1.process_traffic.unwrap();
    assert_eq!(data.apps.len(), 1);
    assert_eq!(data.apps[0].name, "app1");
    assert_eq!(data.apps[0].download, 2048.0);
    assert_eq!(data.others.download, 7952.0);
}

#[test]
fn restarting_visibility_or_changing_connection_discards_stale_process_history() {
    let root = FakeRoot::from("network");
    process_owner(&root, 1, 1000, 1, "browser");
    let clock = FakeClock::at(1000);
    let replies = TcpReplies(Default::default());
    let nm = FakeNetworkManager(std::sync::Arc::new(std::sync::Mutex::new(
        Nm.connection().unwrap(),
    )));
    let mut network =
        Network::new(root, nm.clone(), Ping(20.0), clock.clone()).with_processes(replies.clone());
    network.process_visible(true);
    *replies.0.lock().unwrap() = Some(socket_reply(0, 1, 1, 1000));
    network.step();
    *replies.0.lock().unwrap() = Some(socket_reply(1, 1, 1, 1000));
    clock.advance(2000);
    assert_eq!(
        network.step().unwrap().1.process_traffic.unwrap().apps[0].download,
        2048.0
    );
    network.process_visible(false);
    clock.advance(20_000);
    *replies.0.lock().unwrap() = Some(socket_reply(2, 1, 1, 1000));
    network.process_visible(true);
    assert!(network.step().unwrap().1.process_traffic.is_none());
    *replies.0.lock().unwrap() = Some(socket_reply(3, 1, 1, 1000));
    clock.advance(2000);
    assert_eq!(
        network.step().unwrap().1.process_traffic.unwrap().apps[0].download,
        2048.0
    );
    nm.0.lock().unwrap().id = "new-network".into();
    clock.advance(1000);
    assert!(network.step().unwrap().1.process_traffic.is_none());
    *nm.0.lock().unwrap() = Connection::default();
    clock.advance(1000);
    assert!(network.step().unwrap().1.process_traffic.is_none());
}

#[test]
fn attributes_primary_link_and_its_vpn_addresses_but_excludes_a_second_physical_link() {
    let mut reading = reading_nm();
    reading.active[0].addresses = vec!["192.0.2.1".parse().unwrap()];
    let mut secondary = reading.active[0].clone();
    secondary.path = "/secondary".into();
    secondary.interface = "eth0".into();
    secondary.addresses = vec!["192.0.2.10".parse().unwrap()];
    let mut vpn = reading.active[0].clone();
    vpn.path = "/vpn".into();
    vpn.vpn = true;
    vpn.interface = "tun0".into();
    vpn.addresses = vec!["10.8.0.2".parse().unwrap()];
    reading.primary = vpn.path.clone();
    reading.active.extend([secondary, vpn]);
    let root = FakeRoot::from("network");
    for pid in 1..=3 {
        process_owner(&root, pid, 1000, pid, &format!("app{pid}"));
    }
    let clock = FakeClock::at(1000);
    let replies = TcpReplies(Default::default());
    let mut network = Network::new(
        root.clone(),
        NetworkManagerDbus(FakeDbus(reading)),
        Ping(20.0),
        clock.clone(),
    )
    .with_processes(replies.clone());
    network.process_visible(true);
    let dump = |index| {
        (1..=3)
            .flat_map(|inode| {
                let mut socket = socket_reply(index, inode, inode as u64, 1000);
                let address = match inode {
                    1 => [192, 0, 2, 1],
                    2 => [192, 0, 2, 10],
                    _ => [10, 8, 0, 2],
                };
                socket[24..28].copy_from_slice(&address);
                socket
            })
            .collect()
    };
    *replies.0.lock().unwrap() = Some(dump(0));
    network.step();
    *replies.0.lock().unwrap() = Some(dump(1));
    root.write(
        "/proc/net/dev",
        "wlo1: 818878449 0 0 0 0 0 0 0 122362683 0 0 0 0 0 0 0\n",
    );
    clock.advance(2000);
    let data = network.step().unwrap().1.process_traffic.unwrap();
    assert_eq!(
        data.apps
            .iter()
            .map(|app| app.name.as_str())
            .collect::<Vec<_>>(),
        vec!["app1", "app3"]
    );
    assert_eq!(data.others.download, 5904.0);
}

#[test]
fn accepts_native_ipv6_and_mapped_external_ipv4_but_never_mapped_loopback() {
    let root = FakeRoot::from("network");
    let clock = FakeClock::at(1000);
    let replies = TcpReplies(Default::default());
    for pid in 1..=3 {
        process_owner(&root, pid, 1000, pid, &format!("app{pid}"));
    }
    let mut network =
        Network::new(root.clone(), Nm, Ping(20.0), clock.clone()).with_processes(replies.clone());
    let local_v6 = reading_nm().active[0]
        .addresses
        .iter()
        .find_map(|ip| match ip {
            std::net::IpAddr::V6(ip) => Some(*ip),
            _ => None,
        })
        .unwrap();
    let dump = |index| {
        (1..=3)
            .flat_map(|inode| {
                let mut socket = socket_reply(index, inode, inode as u64, 1000);
                socket[16] = 10;
                let local = if inode == 1 {
                    local_v6
                } else if inode == 2 {
                    std::net::Ipv4Addr::from(recorded_local_address()).to_ipv6_mapped()
                } else {
                    std::net::Ipv4Addr::LOCALHOST.to_ipv6_mapped()
                };
                let remote: std::net::Ipv6Addr = "2001:db8::1".parse().unwrap();
                socket[24..40].copy_from_slice(&local.octets());
                socket[40..56].copy_from_slice(&remote.octets());
                socket
            })
            .collect()
    };
    *replies.0.lock().unwrap() = Some(dump(0));
    network.process_visible(true);
    network.step();
    *replies.0.lock().unwrap() = Some(dump(1));
    root.write(
        "/proc/net/dev",
        "wlo1: 818878449 0 0 0 0 0 0 0 122362683 0 0 0 0 0 0 0\n",
    );
    clock.advance(2000);
    let data = network.step().unwrap().1.process_traffic.unwrap();
    assert_eq!(
        data.apps
            .iter()
            .map(|app| app.name.as_str())
            .collect::<Vec<_>>(),
        vec!["app1", "app2"]
    );
    assert_eq!(data.others.download, 5904.0);
}
