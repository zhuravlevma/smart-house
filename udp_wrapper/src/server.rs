use std::error::Error;
use crate::Socket;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;

pub struct UdpServer {
    socket: Socket,
}

impl UdpServer {
    pub fn new(address: String) -> Self {
        let socket = Socket::new(address);
        socket.set_write_timeout(Some(Duration::new(1, 0))).unwrap();
        Self { socket }
    }

    pub fn receive(&self) -> (usize, SocketAddr, String) {
        let mut buff = [0; 1024];
        let (usize, src_addr) = self.socket.recv_from(&mut buff).unwrap();
        let data = String::from_utf8(Vec::from(&buff[0..usize])).unwrap();
        println!("Len: {}", usize);
        println!("Data: {}", data);
        println!("Address: {}", src_addr);
        (usize, src_addr, data)
    }

    pub fn response(&self, data: String, receiver: SocketAddr) -> usize {
        self.socket
            .send_to(data.as_bytes(), receiver.to_string())
            .unwrap()
    }
}

pub struct UdpServerAsync {
    socket: UdpSocket,
}

impl UdpServerAsync {
    pub async fn new(address: String) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(address).await?;
        Ok(Self { socket })
    }

    pub async fn receive(&self) -> (usize, SocketAddr, String) {
        let mut buff = [0; 1024];
        let (usize, src_addr) = self.socket.recv_from(&mut buff).await.unwrap();
        let data = String::from_utf8(Vec::from(&buff[0..usize])).unwrap();
        println!("Len: {}", usize);
        println!("Data: {}", data);
        println!("Address: {}", src_addr);
        (usize, src_addr, data)
    }

    pub async fn response(&self, data: String, receiver: SocketAddr) -> Result<usize, Box<dyn Error>> {
        Ok(self.socket
            .send_to(data.as_bytes(), receiver.to_string()).await?)
    }
}
