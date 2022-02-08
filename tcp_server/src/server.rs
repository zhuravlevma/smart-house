use crate::routing::Routing;
use crate::tcp_request::Request;
use std::error::Error;
use tcp_wrapper::server_std::TcpServer;

pub struct Server {
    tcp: TcpServer,
}
impl Server {
    pub fn new(address: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            tcp: TcpServer::bind(address)?,
        })
    }
    pub fn listen(&self, mut routing: Box<dyn Routing>) -> Result<(), Box<dyn Error>> {
        for elem in self.tcp.incoming() {
            let mut e = elem?;
            let req_str = e.recv_request()?;
            let req = Request::new(&req_str)?;
            let resp = routing.routing(req);
            e.send_response(resp)?;
        }
        Ok(())
    }
}
