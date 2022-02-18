use crate::routing::Routing;
use crate::tcp_request::Request;
use log::{debug, info};
use std::error::Error;
use tcp_wrapper::server_async::TcpServer;
use tokio::net::ToSocketAddrs;

pub struct AsyncServer {
    tcp: TcpServer,
}

impl AsyncServer {
    pub async fn new<Addrs>(address: Addrs) -> Result<Self, Box<dyn Error>>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpServer::bind(address).await?;
        info!("Initial tcp server async");
        Ok(Self { tcp })
    }
    pub async fn listen(&self, mut routing: Box<dyn Routing>) -> Result<(), Box<dyn Error>> {
        let elem = self.tcp.accept().await?;
        let req_str = elem.recv_request().await?;
        debug!("Tcp server async request string is {}", req_str);
        let req = Request::new(&req_str)?;
        let res = routing.routing(req);
        debug!("Tcp server response string is {}", res);
        elem.send_response(res).await?;
        Ok(())
    }
}
