use tokio::net::{TcpStream, ToSocketAddrs};
use crate::async_mod::Stream;
use crate::error::{ConnectError, ConnectResult, RequestResult};

struct TcpClient {
    stream: Stream,
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
        super::send_string_async(req, &self.stream).await?;
        let response = super::recv_string_async(&self.stream).await?;
        Ok(response)
    }

    async fn try_handshake(s: TcpStream) -> ConnectResult<Self> {
        super::write_all_async(&s, b"clnt").await?;
        let mut buf = [0; 4];
        super::read_exact_async(&s, &mut buf).await?;
        if &buf != b"serv" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        Ok(Self { stream: s })
    }
}
