use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub struct Socket {
    socket: UdpSocket,
}

impl Socket {
    pub fn create_udp_socket(host: String) -> UdpSocket {
        UdpSocket::bind(host).expect("failed to bind host socket")
    }
    pub fn new(host: String) -> Self {
        Self {
            socket: Socket::create_udp_socket(host),
        }
    }

    fn get_duration(&self) -> Option<Duration> {
        Some(Duration::new(1, 0))
    }

    pub fn set_duration(&self) {
        self.socket
            .set_read_timeout(self.get_duration())
            .expect("failed to set timeout")
    }

    pub fn send_to(&self, message: &[u8], receiver: String) -> std::io::Result<usize> {
        self.socket.send_to(message, receiver)
    }

    pub fn recv_from(&self, buff: &mut [u8]) -> std::io::Result<(usize, SocketAddr)> {
        self.socket.recv_from(buff)
    }
}
