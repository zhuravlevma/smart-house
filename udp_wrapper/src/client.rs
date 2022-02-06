use crate::Socket;
use std::time::Duration;

pub struct UdpClient {
    socket: Socket,
}

impl UdpClient {
    pub fn new(address: String) -> Self {
        let socket = Socket::new(address);
        socket
            .set_write_timeout(Some(Duration::new(10, 0)))
            .unwrap();
        Self { socket }
    }

    pub fn send(&self, message: String, receiver: String) -> String {
        println!("sending message: {:?}", message);
        let buff = message.as_bytes();

        self.socket.send_to(buff, receiver).unwrap();
        self.response()
    }

    fn response(&self) -> String {
        let mut buff = [0; 10];
        let (usize, _socket_address) = self.socket.recv_from(&mut buff).unwrap();
        String::from_utf8(Vec::from(&buff[0..usize])).unwrap()
    }
}
