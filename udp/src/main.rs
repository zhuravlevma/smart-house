#[macro_use]
extern crate simple_log;
use simple_log::LogConfigBuilder;
use std::io;
use io::Result as ioResult;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{UdpSocket};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = LogConfigBuilder::builder()
        .output_console()
        .build();
    simple_log::new(config)?;
    let host = "0.0.0.0:12345";
    debug!("initializing listener udp socket on {}", host);
    let res = create_udp_socket_receiver(&host).await?;
    let res2 = create_udp_socket_sender().await?;
    Ok(())
}

async fn create_udp_socket_receiver(host: &str) -> ioResult<UdpSocket> {
    let socket = UdpSocket::bind(&host).await?;
    Ok(socket)
}

async fn create_udp_socket_sender() -> ioResult<UdpSocket> {
    let local_address = "0.0.0.0:0";
    let socket = UdpSocket::bind(local_address).await?;
    let socket_address: SocketAddr = "8.8.8.8:53"
        .parse::<SocketAddr>()
        .expect("Invalid forwarding address specified");
    socket.connect(&socket_address).await?;
    debug!("initializing listener udp socket on {}", local_address);
    return Ok(socket);
}