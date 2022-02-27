pub use connection_async::Connection;
use log::info;

use crate::error::{BindError, ConnectError, ConnectResult};
use crate::stream::stream_async::Stream;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct TcpServer {
    tcp: TcpListener,
}

/// Tcp async client and server
/// # Example
/// ```
/// use tcp_wrapper::client_async::TcpClient;
/// use tcp_wrapper::server_async::TcpServer;
/// use tcp_wrapper::server_async::Connection;
/// use std::thread;
/// use std::error::Error;
/// use tokio::spawn;
/// use tokio::join;
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     let tcp_server = TcpServer::bind("127.0.0.1:8085").await?;
///     join!(
///         server(tcp_server),
///         client()
///     );
///     Ok(())
/// }
///
/// async fn server(server: TcpServer) -> Result<(), Box<dyn Error>> {
///     let connection = server.accept().await?;
///     let req = connection.recv_request().await?;
///     assert_eq!(req, "Hello, server");
///     connection.send_response("Hello, client").await?;
///     Ok(())
/// }
///
/// async fn client() -> Result<(), Box<dyn Error>> {
///     let mut client = TcpClient::connect("127.0.0.1:8085").await?;
///     let response = client.send_request("Hello, server").await?;
///     assert_eq!(response, "Hello, client");
///     Ok(())
/// }
/// ```
impl TcpServer {
    pub async fn bind<Addrs>(addrs: Addrs) -> BindResult
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs).await?;
        info!("Tcp server async listen port {} ", tcp.local_addr()?);
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
