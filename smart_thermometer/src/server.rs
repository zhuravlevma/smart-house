use crate::handle::Handle;
use config_simple::ConfigServer;
use std::sync::Arc;
use std::thread;
use udp_wrapper::UdpServer;

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
        let arc_handle = Arc::new(self.handle);
        let arc_connection = Arc::new(self.connection);
        let thread = thread::spawn(move || loop {
            let connection = arc_connection.clone();
            let (_number_of_bytes, src_addr, _data) = connection.receive();
            let handle = arc_handle.clone();
            thread::spawn(move || {
                connection.response(handle.routing(_data), src_addr);
            });
        });
        thread.join().unwrap();
    }
}
