use crate::error::{BindError, ConnectError, ConnectResult};
pub use connection_std::Connection;
use log::info;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct TcpServer {
    tcp: TcpListener,
}

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
