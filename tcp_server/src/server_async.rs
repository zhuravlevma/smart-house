use crate::routing::Routing;
use crate::tcp_request::Request;
use std::error::Error;
use tcp_wrapper::server_async::TcpServer;

pub struct AsyncServer {
    tcp: TcpServer,
}

impl AsyncServer {
    pub async fn new(address: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            tcp: TcpServer::bind(address).await?,
        })
    }
    pub async fn listen(&self, mut routing: Box<dyn Routing>) -> Result<(), Box<dyn Error>> {
        let elem = self.tcp.accept().await?;
        let req_str = elem.recv_request().await?;
        let req = Request::new(&req_str)?;
        elem.send_response(routing.routing(req)).await?;
        Ok(())
    }
}
