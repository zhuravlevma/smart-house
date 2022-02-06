use crate::Socket;
use std::time::Duration;

pub struct UdpPusher {
    socket: Socket,
}

impl UdpPusher {
    pub fn new(address: String) -> Self {
        let socket = Socket::new(address);
        socket
            .set_write_timeout(Some(Duration::new(11, 0)))
            .unwrap();
        Self { socket }
    }

    pub fn send(&self, message: String, receiver: String) -> usize {
        println!("sending message: {:?}", message);
        let buff = message.as_bytes();

        self.socket.send_to(buff, receiver).unwrap()
    }
}
