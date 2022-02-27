use crate::error::{BindError, ConnectError, ConnectResult};
pub use connection_std::Connection;
use log::info;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct TcpServer {
    tcp: TcpListener,
}

/// Tcp client and server
/// # Example
/// ```
/// use tcp_wrapper::client_std::Client;
/// use tcp_wrapper::server_std::TcpServer;
/// use std::error::Error;
/// use std::thread;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let server = TcpServer::bind("127.0.0.1:9092")?;
///     thread::spawn(move || {
///        for elem in server.incoming() {
///             let mut e = elem.unwrap();
///             let req_str = e.recv_request().unwrap();
///             e.send_response("test").unwrap();
///         }
///     });
///     let mut client = Client::connect("127.0.0.1:9092")?;
///     let resp = client.send_request("hello").unwrap();
///     assert_eq!(resp, "test");
///     Ok(())
/// }
/// ```
impl TcpServer {
    pub fn bind<IpAddrs>(addrs: IpAddrs) -> Result<TcpServer, BindError>
    where
        IpAddrs: ToSocketAddrs,
    {
        let server = TcpListener::bind(addrs)?;
        info!("Tcp server async listen port {} ", server.local_addr()?);
        Ok(Self { tcp: server })
    }

    /// Blocking iterator for incoming connections.
    pub fn incoming(&self) -> impl Iterator<Item = ConnectResult<Connection>> + '_ {
        self.tcp.incoming().map(|s| match s {
            Ok(s) => Self::try_handshake(s),
            Err(e) => Err(ConnectError::Io(e)),
        })
    }

    fn try_handshake(mut stream: TcpStream) -> ConnectResult<Connection> {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        if &buf != b"clnt" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        stream.write_all(b"serv")?;
        Ok(Connection { stream })
    }
}

pub mod connection_std;
