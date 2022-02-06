use std::error::Error;
use udp_wrapper::UdpServerAsync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = UdpServerAsync::new("127.0.0.1:9001".to_string()).await?;
    server.receive().await;
    Ok(())
}
