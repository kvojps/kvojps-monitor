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
