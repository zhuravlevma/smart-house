use crate::error::{BindError, ConnectError, ConnectResult};
use crate::{ReceiveResult, SendResult, Stream};
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};

pub struct Server {
    _tcp: TcpListener,
}

impl Server {
    pub fn bind<IpAddrs>(addrs: IpAddrs) -> Result<Server, BindError>
    where
        IpAddrs: ToSocketAddrs,
    {
        let server = TcpListener::bind(addrs)?;
        Ok(Self { _tcp: server })
    }
    fn _try_handshake(mut stream: TcpStream) -> ConnectResult<StpConnection> {
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        if &buf != b"clnt" {
            let msg = format!("received: {:?}", buf);
            return Err(ConnectError::BadHandshake(msg));
        }
        stream.write_all(b"serv")?;
        Ok(StpConnection { stream })
    }
}

/// Represent connection from client.
///
/// Allows to receive requests and send responses.
pub struct StpConnection {
    stream: TcpStream,
}

impl StpConnection {
    /// Send response to client
    pub fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        Stream::send_string(response, &mut self.stream)
    }

    /// Receive requests from client
    pub fn recv_request(&mut self) -> ReceiveResult {
        Stream::receive_string(&mut self.stream)
    }

    /// Address of connected client
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
