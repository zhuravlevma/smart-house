use crate::Socket;
use std::net::SocketAddr;
use std::time::Duration;

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
        let mut buf = [0; 10];
        let (number_of_bytes, src_addr) = self.socket.recv_from(&mut buf).unwrap();
        let data = String::from_utf8(Vec::from(buf)).unwrap();
        println!("Len: {}", number_of_bytes);
        println!("Data: {}", data);
        println!("Address: {}", src_addr);
        (number_of_bytes, src_addr, data)
    }

    pub fn response(&self, data: String, receiver: SocketAddr) -> usize {
        self.socket
            .send_to(data.as_bytes(), receiver.to_string())
            .unwrap()
    }
}
