extern crate simple_log;
use simple_log::LogConfigBuilder;
use std::error::Error;
use std::net::UdpSocket;

fn main() -> Result<(), Box<dyn Error>> {
    let config = LogConfigBuilder::builder().output_console().build();
    simple_log::new(config)?;
    let host = "127.0.0.1:12345";
    let listen_socket = UdpSocket::bind(host)?;

    let mut buf = [0; 10];
    loop {
        let (number_of_bytes, src_addr) = listen_socket.recv_from(&mut buf)?;
        println!("Len: {}", number_of_bytes);
        println!("Data: {}", String::from_utf8(Vec::from(buf))?);
        println!("Address: {}", src_addr);
    }
}
