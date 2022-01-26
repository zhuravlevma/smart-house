use config::Config;
use std::error::Error;
use std::thread;
use tcp::server::Server;
use tcp::server::{Connection, Request, RequestHandler, TcpServer};

pub struct Rosette {
    power: u32,
}

impl Rosette {
    pub fn new(power: u32) -> Self {
        Self { power }
    }
}

struct RosetteHandler {}
impl RequestHandler for RosetteHandler {
    fn routing(&mut self, mut request: Request) -> String {
        let command = request.next_data();
        match command {
            "get_power" => self.get_power(request),
            "off" => self.off(request),
            _ => "Bad command".into(),
        }
    }
}

impl RosetteHandler {
    fn get_power(&mut self, mut request: Request) -> String {
        let data = request.next_data();
        if data.is_empty() {
            return "data empty".into();
        }
        let rosette = Rosette::new(220);
        format!("{} - {}", data.to_string(), rosette.power)
    }
    fn off(&self, mut _request: Request) -> String {
        let rosette = Rosette::new(0);
        format!("{}", rosette.power)
    }
}

pub struct RosetteServer {
    connection: TcpServer,
}

impl RosetteServer {
    pub fn new(address: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            connection: TcpServer::bind(address)?,
        })
    }
}
impl Server for RosetteServer {
    fn get_connection(&self) -> &TcpServer {
        &self.connection
    }
    fn handle(&self, connection: Connection, address: String) {
        thread::spawn(move || {
            let mut rosette_handler = RosetteHandler {};
            if rosette_handler.handle_connection(connection).is_err() {
                println!("Client disconnected: {}", address);
            }
        });
    }
}

mod config;
