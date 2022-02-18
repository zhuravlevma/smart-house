use crate::routing::Routing;
use crate::tcp_request::Request;
use log::{debug, info};
use std::error::Error;
use std::net::ToSocketAddrs;
use tcp_wrapper::server_std::TcpServer;

pub struct Server {
    tcp: TcpServer,
}
impl Server {
    pub fn new<Addrs>(address: Addrs) -> Result<Self, Box<dyn Error>>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpServer::bind(address)?;
        info!("Initial tcp server");
        Ok(Self { tcp })
    }
    pub fn listen(&self, mut routing: Box<dyn Routing>) -> Result<(), Box<dyn Error>> {
        for elem in self.tcp.incoming() {
            let mut e = elem?;
            let req_str = e.recv_request()?;
            debug!("Tcp server request string is {}", req_str);
            let req = Request::new(&req_str)?;
            let resp = routing.routing(req);
            debug!("Tcp server response string is {}", resp);
            e.send_response(resp)?;
        }
        Ok(())
    }
}
