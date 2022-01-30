use crate::Socket;

pub struct Server {
    socket: Socket,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            socket: Socket::new(address),
        }
    }

    pub fn receive(&self) -> String {
        let mut buf = [0; 10];
        let (number_of_bytes, src_addr) = self.socket.recv_from(&mut buf).unwrap();
        let data = String::from_utf8(Vec::from(buf)).unwrap();
        println!("Len: {}", number_of_bytes);
        println!("Data: {}", data);
        println!("Address: {}", src_addr);
        data
    }
}
