use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::time::{Duration, Instant};

pub trait Sonda: Send + Sync {
    fn medir(&self, destino: SocketAddr) -> Option<f64>;
}
pub struct PingSocket;
impl Sonda for PingSocket {
    fn medir(&self, destino: SocketAddr) -> Option<f64> {
        let ipv4 = destino.is_ipv4();
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
                let inicio = Instant::now();
                let mut tcp = destino;
                tcp.set_port(443);
                return TcpStream::connect_timeout(&tcp, Duration::from_millis(700))
                    .ok()
                    .map(|_| inicio.elapsed().as_secs_f64() * 1000.0);
            }
        };
        socket
            .set_read_timeout(Some(Duration::from_millis(700)))
            .ok()?;
        socket
            .set_write_timeout(Some(Duration::from_millis(700)))
            .ok()?;
        socket.connect(&SockAddr::from(destino)).ok()?;
        let mut pacote = [0u8; 16];
        pacote[0] = if ipv4 { 8 } else { 128 };
        pacote[6..8].copy_from_slice(&1u16.to_be_bytes());
        pacote[8..].copy_from_slice(b"kvojps01");
        if ipv4 {
            let mut soma: u32 = pacote
                .as_chunks::<2>()
                .0
                .iter()
                .map(|p| u16::from_be_bytes([p[0], p[1]]) as u32)
                .sum();
            while soma > 65535 {
                soma = (soma & 65535) + (soma >> 16);
            }
            pacote[2..4].copy_from_slice(&(!(soma as u16)).to_be_bytes());
        }
        let socket: UdpSocket = socket.into();
        let inicio = Instant::now();
        socket.send(&pacote).ok()?;
        let mut resposta = [0u8; 128];
        let n = socket.recv(&mut resposta).ok()?;
        (n >= 16
            && resposta[0] == if ipv4 { 0 } else { 129 }
            && resposta[1] == 0
            && resposta[6..16] == pacote[6..16])
            .then(|| inicio.elapsed().as_secs_f64() * 1000.0)
    }
}
