use std::io;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use crate::error::{BindError, ConnectError, ConnectResult};
use crate::{ReceiveResult, SendResult};
use crate::async_mod::Stream;

pub struct TcpServer {
    tcp: TcpListener,
}

impl TcpServer {
    pub async fn bind<Addrs>(addrs: Addrs) -> BindResult
        where
            Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs).await?;
        Ok(Self { tcp })
    }

    /// Blocking iterator for incoming connections.
    pub async fn accept(&self) -> ConnectResult<StpConnection> {
        let (connection, _) = self.tcp.accept().await?;
        Self::try_handshake(connection).await
    }

    async fn try_handshake(stream: TcpStream) -> ConnectResult<StpConnection> {
        let mut buf = [0; 4];
        Stream::read_exact_async(&stream, &mut buf).await?;
        if &buf != b"clnt" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        Stream::write_all_async(&stream, b"serv").await?;
        Ok(StpConnection { stream })
    }
}

pub type BindResult = Result<TcpServer, BindError>;

pub struct StpConnection {
    stream: TcpStream,
}

impl StpConnection {
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