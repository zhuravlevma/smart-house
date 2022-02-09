use crate::error::{BindError, ReceiveError, SendError};
use crate::Socket;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub struct UdpServer {
    socket: Socket,
}

impl UdpServer {
    pub fn new(address: String) -> Result<Self, BindError> {
        let socket = Socket::new(address)?;
        Ok(Self { socket })
    }

    pub fn receive(&self) -> Result<(usize, SocketAddr, String), ReceiveError> {
        let mut buff = [0; 1024];
        let (usize, address) = self.socket.recv_from(&mut buff)?;
        let data = String::from_utf8(Vec::from(&buff[0..usize]))?;
        println!("Len: {}", usize);
        println!("Data: {}", data);
        println!("Address: {}", address);
        Ok((usize, address, data))
    }

    pub fn response(&self, data: String, receiver: SocketAddr) -> Result<usize, SendError> {
        Ok(self.socket.send_to(data.as_bytes(), receiver.to_string())?)
    }
}

pub struct UdpServerAsync {
    socket: UdpSocket,
}

impl UdpServerAsync {
    pub async fn new(address: String) -> Result<Self, BindError> {
        let socket = UdpSocket::bind(address).await?;
        Ok(Self { socket })
    }

    pub async fn receive(&self) -> Result<(usize, SocketAddr, String), ReceiveError> {
        let mut buff = [0; 1024];
        let (usize, src_addr) = self.socket.recv_from(&mut buff).await?;
        let data = String::from_utf8(Vec::from(&buff[0..usize])).unwrap();
        println!("Len: {}", usize);
        println!("Data: {}", data);
        println!("Address: {}", src_addr);
        Ok((usize, src_addr, data))
    }

    pub async fn response(&self, data: String, receiver: SocketAddr) -> Result<usize, SendError> {
        Ok(self
            .socket
            .send_to(data.as_bytes(), receiver.to_string())
            .await?)
    }
}
