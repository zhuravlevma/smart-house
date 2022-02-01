use crate::handle::Handle;
use config::ConfigServer;
use std::sync::{mpsc, Arc};
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
        let arc_handle = Arc::new(self.handle);
        let thread = thread::spawn(move || loop {
            let (_number_of_bytes, src_addr, _data) = self.connection.receive();
            let (tx, rx) = mpsc::channel();
            let handle = arc_handle.clone();
            thread::spawn(move || tx.send(handle.routing(_data)).unwrap());
            self.connection.response(rx.recv().unwrap(), src_addr);
        });
        thread.join().unwrap();
    }
}
