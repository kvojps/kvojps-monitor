use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::time::{Duration, Instant};

pub trait Probe: Send + Sync {
    fn measure(&self, destination: SocketAddr) -> Option<f64>;
}
pub struct PingSocket;
impl Probe for PingSocket {
    fn measure(&self, destination: SocketAddr) -> Option<f64> {
        let ipv4 = destination.is_ipv4();
        let socket = Socket::new(
            if ipv4 { Domain::IPV4 } else { Domain::IPV6 },
            Type::DGRAM,
            Some(if ipv4 {
                Protocol::ICMPV4
            } else {
                Protocol::ICMPV6
            }),
        );
        let socket = match socket {
            Ok(s) => s,
            Err(_) => {
                let start = Instant::now();
                let mut tcp = destination;
                tcp.set_port(443);
                return TcpStream::connect_timeout(&tcp, Duration::from_millis(700))
                    .ok()
                    .map(|_| start.elapsed().as_secs_f64() * 1000.0);
            }
        };
        socket
            .set_read_timeout(Some(Duration::from_millis(700)))
            .ok()?;
        socket
            .set_write_timeout(Some(Duration::from_millis(700)))
            .ok()?;
        socket.connect(&SockAddr::from(destination)).ok()?;
        let mut package = [0u8; 16];
        package[0] = if ipv4 { 8 } else { 128 };
        package[6..8].copy_from_slice(&1u16.to_be_bytes());
        package[8..].copy_from_slice(b"kvojps01");
        if ipv4 {
            let mut sum: u32 = package
                .as_chunks::<2>()
                .0
                .iter()
                .map(|p| u16::from_be_bytes([p[0], p[1]]) as u32)
                .sum();
            while sum > 65535 {
                sum = (sum & 65535) + (sum >> 16);
            }
            package[2..4].copy_from_slice(&(!(sum as u16)).to_be_bytes());
        }
        let socket: UdpSocket = socket.into();
        let start = Instant::now();
        socket.send(&package).ok()?;
        let mut response = [0u8; 128];
        let n = socket.recv(&mut response).ok()?;
        (n >= 16
            && response[0] == if ipv4 { 0 } else { 129 }
            && response[1] == 0
            && response[6..16] == package[6..16])
            .then(|| start.elapsed().as_secs_f64() * 1000.0)
    }
}
