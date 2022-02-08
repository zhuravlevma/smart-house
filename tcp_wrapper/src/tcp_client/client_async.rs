use tokio::net::{TcpStream, ToSocketAddrs};
use crate::error::{ConnectError, ConnectResult, RequestResult};
use crate::stream::stream_async::Stream;

pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub async fn connect<Addrs>(addrs: Addrs) -> ConnectResult<Self>
        where
            Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs).await?;
        Self::try_handshake(stream).await
    }

    pub async fn send_request<R: AsRef<str>>(&mut self, req: R) -> RequestResult {
        Stream::send_string_async(req, &self.stream).await?;
        let response = Stream::recv_string_async(&self.stream).await?;
        Ok(response)
    }

    async fn try_handshake(s: TcpStream) -> ConnectResult<Self> {
        Stream::write_all_async(&s, b"clnt").await?;
        let mut buf = [0; 4];
        Stream::read_exact_async(&s, &mut buf).await?;
        if &buf != b"serv" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        Ok(Self { stream: s })
    }
}
