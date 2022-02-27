use crate::error::{ConnectError, ConnectResult, RequestResult};
use crate::stream::stream_std::Stream;
use log::{debug, info};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    stream: TcpStream,
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
impl Client {
    pub fn connect<IpAddrs>(addrs: IpAddrs) -> ConnectResult<Self>
    where
        IpAddrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        info!("Tcp stream connect to address {}", stream.local_addr()?);
        Self::try_handshake(stream)
    }

    pub fn send_request<R: AsRef<str>>(&mut self, req: R) -> RequestResult {
        Stream::send_string(req, &mut self.stream)?;
        let response = Stream::receive_string(&mut self.stream)?;
        debug!("Tcp async client resource {}", response);
        Ok(response)
    }

    fn try_handshake(mut stream: TcpStream) -> ConnectResult<Self> {
        stream.write_all(b"clnt")?;
        let mut buffer = [0; 4];
        stream.read_exact(&mut buffer)?;
        if &buffer != b"serv" {
            return Err(ConnectError::BadHandshake(format!(
                "received: {:?}",
                buffer
            )));
        }
        Ok(Self { stream })
    }
}
