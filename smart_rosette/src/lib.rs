use std::error::Error;
use std::thread;
use tcp::server::{Request, RequestHandler, TcpServer, StpConnection};
use tcp::server::Server;

pub struct RosetteState {
    power: u32,
}

impl RosetteState {
    pub fn new(power: u32) -> Self {
        Self {power}
    }
}

struct RosetteHandler {
    data: String
}
impl RequestHandler for RosetteHandler  {
    fn new(data: String) -> Box<Self> {
        Box::new(Self { data })
    }
    fn get_data(&self) -> String {
        self.data.to_string()
    }
    fn fetch(&self, mut request: Request) -> String {
        let data = request.next_data();
        if data.is_empty() {
            return "data empty".into();
        }
        // data.to_string();
        format!("{} - {}", data.to_string(), self.data)
    }
}

pub struct RosetteServer {
    state: RosetteState,
    connection: TcpServer,
}

impl RosetteServer {
    pub fn new(power: u32, address: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            state: RosetteState::new(power),
            connection: TcpServer::bind(address)?
        })
    }
}

impl Server for RosetteServer {
    fn get_connection(&self) -> &TcpServer {
        &self.connection
    }
    fn handle(&self, connection: StpConnection, address: String) {
        let power = self.state.power.to_string();
        thread::spawn(move || {
            let rosette_handler = RosetteHandler::new(power);
            if rosette_handler.handle_connection(connection).is_err() {
                println!("Client disconnected: {}", address);
            }
        });
    }
}