use std::error::Error;
use crate::error::{BindError, ConnectError, ConnectResult};
use crate::{ReceiveResult, SendResult, Stream};
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::str::Split;

pub trait Server {
    fn get_connection(&self) -> &TcpServer;
    fn listen(&self) {
        for connection in self.get_connection().incoming() {
            let connection = match connection {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Can't establish connection: {}", e);
                    continue;
                }
            };
            let address = match connection.peer_addr() {
                Ok(address) => address.to_string(),
                Err(_) => "unknown".into(),
            };
            self.handle(connection, address);
        }
    }
    fn handle(&self, connection: StpConnection, address: String);
}

pub struct TcpServer {
    tcp: TcpListener,
}

impl TcpServer {
    pub fn bind<IpAddrs>(addrs: IpAddrs) -> Result<TcpServer, BindError>
    where
        IpAddrs: ToSocketAddrs,
    {
        let server = TcpListener::bind(addrs)?;
        Ok(Self { tcp: server })
    }

    /// Blocking iterator for incoming connections.
    pub fn incoming(&self) -> impl Iterator<Item = ConnectResult<StpConnection>> + '_ {
        self.tcp.incoming().map(|s| match s {
            Ok(s) => Self::try_handshake(s),
            Err(e) => Err(ConnectError::Io(e)),
        })
    }

    fn try_handshake(mut stream: TcpStream) -> ConnectResult<StpConnection> {
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


pub struct Request<'a>(Split<'a, &'a str>);
impl<'a> Request<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s.split("|||"))
    }

    pub fn next_data(&mut self) -> &'a str {
        self.0.next().unwrap_or("")
    }
}

pub trait RequestHandler {
    fn new(data: String) -> Box<Self>;
    fn get_data(&self) -> String;

    fn handle_connection(&self, mut connection: StpConnection) -> Result<(), Box<dyn Error>> {
        let mut handler = Self::new(self.get_data());
        loop {
            let req_str = connection.recv_request()?;
            let req = Request::new(&req_str);
            connection.send_response(handler.handle(req))?;
        }
    }
    fn handle(&mut self, mut request: Request) -> String {
        let command = request.next_data();
        match command {
            "fetch" => self.fetch(request),
            // "create" => self.create_room(request),
            // "append" => self.append(request),
            _ => "Bad command".into(),
        }
    }
    fn fetch(&self, request: Request) -> String;
}