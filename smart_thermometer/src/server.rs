use crate::handle::Handle;
use config::ConfigServer;
use std::thread;
use udp::UdpServer;

pub struct ThermometerServer {
    connection: UdpServer,
    handle: Handle,
}

impl ThermometerServer {
    pub fn new(config: ConfigServer) -> Self {
        Self {
            connection: UdpServer::new(config.url),
            handle: Handle::new(),
        }
    }

    pub fn listen(self) {
        let thread = thread::spawn(move || loop {
            let (_number_of_bytes, src_addr, _data) = self.connection.receive();
            self.connection
                .response(self.handle.routing(_data), src_addr);
        });
        thread.join().unwrap();
    }
}
