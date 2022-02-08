pub use connection_async::Connection;

use crate::error::{BindError, ConnectError, ConnectResult};
use crate::stream::stream_async::Stream;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

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
    pub async fn accept(&self) -> ConnectResult<Connection> {
        let (connection, _) = self.tcp.accept().await?;
        Self::try_handshake(connection).await
    }

    async fn try_handshake(stream: TcpStream) -> ConnectResult<Connection> {
        let mut buf = [0; 4];
        Stream::read_exact_async(&stream, &mut buf).await?;
        if &buf != b"clnt" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        Stream::write_all_async(&stream, b"serv").await?;
        Ok(Connection { stream })
    }
}

pub type BindResult = Result<TcpServer, BindError>;

pub mod connection_async;
