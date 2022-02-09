use crate::error::{BindError, SendError};
use crate::Socket;

pub struct UdpPusher {
    socket: Socket,
}

impl UdpPusher {
    pub fn new(address: String) -> Result<Self, BindError> {
        let socket = Socket::new(address)?;
        Ok(Self { socket })
    }

    pub fn send(&self, message: String, receiver: String) -> Result<usize, SendError> {
        println!("sending message: {:?}", message);
        let buff = message.as_bytes();

        Ok(self.socket.send_to(buff, receiver)?)
    }
}
