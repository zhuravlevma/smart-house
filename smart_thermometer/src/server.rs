use config::ConfigServer;
use udp::UdpServer;

pub struct ThermometerServer {
    connection: UdpServer,
}

impl ThermometerServer {
    pub fn new(config: ConfigServer) -> Self {
        Self {
            connection: UdpServer::new(config.url),
        }
    }

    pub fn listen(&self) {
        loop {
            let message = self.connection.receive();
            println!("{}", message);
        }
    }
}
