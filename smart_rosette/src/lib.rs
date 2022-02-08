use std::error::Error;
use std::str::Split;
pub use server::RosetteServer;
use tcp_wrapper::server_std::TcpServer;
use tcp_wrapper::server_std::connection_std::Connection;
pub mod domain;
pub mod server;

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
    fn handle(&self, connection: Connection, address: String);
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
    fn handle_connection(&mut self, mut connection: Connection) -> Result<(), Box<dyn Error>> {
        loop {
            let req_str = connection.recv_request()?;
            let req = Request::new(&req_str);
            connection.send_response(self.routing(req))?;
        }
    }
    fn routing(&mut self, request: Request) -> String;
}