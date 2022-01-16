use std::net::TcpStream;

pub struct Client {
    stream: TcpStream
}

// type ConnectResult<T> = Result<T, ConnectError>;

impl Client {
    // pub fn connect<IpAddrs>(addrs: IpAddrs) -> ConnectResult<Self>
}