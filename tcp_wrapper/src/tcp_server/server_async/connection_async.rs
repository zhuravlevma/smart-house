use crate::stream_async::Stream;
use crate::{ReceiveResult, SendResult};
use std::net::SocketAddr;
use tokio::io;
use tokio::net::TcpStream;

pub struct Connection {
    pub(crate) stream: TcpStream,
}

impl Connection {
    pub async fn send_response<Resp: AsRef<str>>(&self, response: Resp) -> SendResult {
        Stream::send_string_async(response, &self.stream).await
    }

    pub async fn recv_request(&self) -> ReceiveResult {
        Stream::recv_string_async(&self.stream).await
    }

    pub async fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
