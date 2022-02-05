use std::error::Error;
use config::ConfigServer;
use smart_rosette::server::RosetteServerAsync;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = ConfigServer::new("127.0.0.1".to_string(), "8081".to_string())?;
    let server = RosetteServerAsync::bind(config).await?;
    server.listen().await
}