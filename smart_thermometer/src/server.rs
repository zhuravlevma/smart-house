use config::ConfigServer;
use std::sync::Arc;
use std::thread;
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

    pub fn listen(self) {
        let cmd_arc = Arc::new(self.connection);
        let thread = thread::spawn(move || loop {
            let (_number_of_bytes, src_addr, _data) = cmd_arc.as_ref().receive();
            cmd_arc.response(37.to_string(), src_addr);
        });
        thread.join().unwrap();
    }
}
