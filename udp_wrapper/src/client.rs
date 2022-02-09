use crate::error::{BindError, ReceiveError, SendError};
use crate::Socket;

pub struct UdpClient {
    socket: Socket,
}

impl UdpClient {
    pub fn new(address: String) -> Result<Self, BindError> {
        let socket = Socket::new(address)?;
        Ok(Self { socket })
    }

    pub fn send(&self, message: String, receiver: String) -> Result<String, SendError> {
        println!("sending message: {:?}", message);
        let buff = message.as_bytes();

        self.socket.send_to(buff, receiver)?;
        Ok(self.response()?)
    }

    fn response(&self) -> Result<String, ReceiveError> {
        let mut buff = [0; 10];
        let (usize, _socket_address) = self.socket.recv_from(&mut buff)?;
        Ok(String::from_utf8(Vec::from(&buff[0..usize]))?)
    }
}
