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
        let arc_connection = Arc::new(self.connection);
        let thread = thread::spawn(move || loop {
            let connection = arc_connection.clone();
            let (_number_of_bytes, src_addr, _data) = connection.receive();
            let (tx, rx) = mpsc::channel();
            let handle = arc_handle.clone();
            thread::spawn(move || match tx.send(handle.routing(_data)) {
                Ok(_) => {}
                Err(error) => {
                    println!("{}", error)
                }
            });
            let connection = arc_connection.clone();
            thread::spawn(move || {
                loop {
                    let result = rx.try_recv();
                    match result {
                        Ok(message) => {
                            connection.response(message, src_addr);
                        }
                        Err(_error) => {
                            // println!("not found");
                        }
                    }
                }
            });
        });
        thread.join().unwrap();
    }
}
