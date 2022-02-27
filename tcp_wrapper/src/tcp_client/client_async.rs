use crate::error::{ConnectError, ConnectResult, RequestResult};
use crate::stream::stream_async::Stream;
use log::{debug, info};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct TcpClient {
    stream: TcpStream,
}

/// Tcp async client and server
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
impl TcpClient {
    pub async fn connect<Addrs>(addrs: Addrs) -> ConnectResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs).await?;
        info!(
            "Tcp async stream connect to address {}",
            stream.local_addr()?
        );
        Self::try_handshake(stream).await
    }

    pub async fn send_request<R: AsRef<str>>(&mut self, req: R) -> RequestResult {
        Stream::send_string_async(req, &self.stream).await?;
        let response = Stream::recv_string_async(&self.stream).await?;
        debug!("Tcp async client resource {}", response);
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
