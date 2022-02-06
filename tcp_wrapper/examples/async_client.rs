use std::error::Error;
use tcp_wrapper::async_mod::client::TcpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = TcpClient::connect("127.0.0.1:55331").await?;
    let response = client.send_request("Hello, server").await?;
    assert_eq!(response, "Hello, client");
    Ok(())
}
