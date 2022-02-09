use crate::error::BindError;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub struct Socket {
    socket: UdpSocket,
}

impl Socket {
    pub fn new(host: String) -> Result<Self, BindError> {
        Ok(Self {
            socket: UdpSocket::bind(host)?,
        })
    }

    pub fn set_write_timeout(&self, duration: Option<Duration>) -> std::io::Result<()> {
        self.socket.set_write_timeout(duration)
    }

    pub fn set_read_timeout(&self, duration: Option<Duration>) -> std::io::Result<()> {
        self.socket.set_read_timeout(duration)
    }

    pub fn send_to(&self, message: &[u8], receiver: String) -> std::io::Result<usize> {
        self.socket.send_to(message, receiver)
    }

    pub fn recv_from(&self, buff: &mut [u8]) -> std::io::Result<(usize, SocketAddr)> {
        self.socket.recv_from(buff)
    }
}
