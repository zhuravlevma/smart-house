use crate::domain::RosetteService;
use crate::server::controller::rosette::PowerController;
use config::ConfigServer;
use routing::RosetteHandler;
use std::error::Error;
use std::thread;
pub use tcp::server::Server;
use tcp::server::{Connection, RequestHandler, TcpServer};
// use tokio::net::TcpListener;

pub struct RosetteServer {
    connection: TcpServer,
}

impl RosetteServer {
    pub fn new(config: ConfigServer) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            connection: TcpServer::bind(config.url)?,
        })
    }
}
impl Server for RosetteServer {
    fn get_connection(&self) -> &TcpServer {
        &self.connection
    }
    fn handle(&self, connection: Connection, address: String) {
        thread::spawn(move || {
            let mut rosette_handler = RosetteHandler {
                power_controller: PowerController::new(RosetteService::new()),
            };
            if rosette_handler.handle_connection(connection).is_err() {
                println!("Client disconnected: {}", address);
            }
        });
    }
}

// pub struct AsyncRosetteServer {
//
// }

mod controller;
mod routing;
