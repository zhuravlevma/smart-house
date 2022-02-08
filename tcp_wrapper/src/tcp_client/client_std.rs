use crate::error::{ConnectError, ConnectResult, RequestResult};
use crate::stream::stream_std::Stream;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn connect<IpAddrs>(addrs: IpAddrs) -> ConnectResult<Self>
    where
        IpAddrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        Self::try_handshake(stream)
    }

    pub fn send_request<R: AsRef<str>>(&mut self, req: R) -> RequestResult {
        Stream::send_string(req, &mut self.stream)?;
        let response = Stream::receive_string(&mut self.stream)?;
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
