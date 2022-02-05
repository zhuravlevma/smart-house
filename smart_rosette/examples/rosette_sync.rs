use config::ConfigServer;
use smart_rosette::RosetteServer;
use std::error::Error;
use tcp::server::Server;

fn main() -> Result<(), Box<dyn Error>> {
    let config = ConfigServer::new("127.0.0.1".to_string(), "8080".to_string())?;
    RosetteServer::new(config)?.listen();
    Ok(())
}
