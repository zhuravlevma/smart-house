use std::net::{TcpListener, ToSocketAddrs};
use crate::error::BindError;
pub struct Server {
    tcp: TcpListener,
}

impl Server {
    pub fn bind<IpAddrs>(addrs: IpAddrs) -> Result<Server, BindError> where Addrs: ToSocketAddrs {
        let server = TcpListener::bind(addrs)?;
        Ok(Self { tcp: server })
    }
}