use crate::stream_std::Stream;
use crate::{ReceiveResult, SendResult};
use std::io;
use std::net::{SocketAddr, TcpStream};

pub struct Connection {
    pub(crate) stream: TcpStream,
}

impl Connection {
    pub fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        Stream::send_string(response, &mut self.stream)
    }
    pub fn recv_request(&mut self) -> ReceiveResult {
        Stream::receive_string(&mut self.stream)
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
