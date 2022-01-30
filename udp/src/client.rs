use crate::Socket;

pub struct Client {
    socket: Socket,
}

impl Client {
    pub fn new(address: String) -> Self {
        Self {
            socket: Socket::new(address),
        }
    }

    pub fn send(&self, message: String, receiver: String) -> std::io::Result<usize> {
        println!("sending message: {:?}", message);
        let buff = message.as_bytes();

        self.socket.send_to(buff, receiver)
    }
}
