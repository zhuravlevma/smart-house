use crate::domain::RosetteService;
use crate::server::controller::rosette::PowerController;
use config::ConfigServer;
use routing::RosetteHandler;
use std::error::Error;
use std::thread;
pub use tcp::server::Server;
use tcp::server::{Connection, RequestHandler, TcpServer};
use tcp::async_mod::server::{TcpServer as TcpServerAsync};
use tcp::error::{BindError};

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

pub struct RosetteServerAsync {
    connection: TcpServerAsync
}

impl RosetteServerAsync {
    pub async fn bind(config: ConfigServer) -> Result<Self, BindError> {
        let server = TcpServerAsync::bind(config.url).await?;
        Ok(Self {
            connection: server
        })
    }
    pub async fn listen(&self) -> Result<(), Box<dyn Error>> {
        loop {
            let connection = self.connection.accept().await?;
            let _req = connection.recv_request().await?;
            connection.send_response("Hello, client").await?;
        }
    }
}



mod controller;
mod routing;
