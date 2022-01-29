use std::net::UdpSocket;
use std::{fs, io};

fn main() -> io::Result<()> {
    let address =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let sock = UdpSocket::bind(address)?;

    let remote_addr = "127.0.0.1:12345";
    // sock.connect(remote_addr).await?;
    // let mut buf = [0; 1024];
    // let len = sock.recv(&mut buf).await?;

    let len = sock.send_to("Haha".as_bytes(), remote_addr)?;
    println!("{:?} bytes sent", len);
    Ok(())
}
