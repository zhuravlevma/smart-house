use config::ConfigServer;
use smart_rosette::RosetteServer;
use smart_rosette::Server;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = ConfigServer::new("127.0.0.1".to_string(), "55332".to_string())?;
    RosetteServer::new(config)?.listen();
    Ok(())
}
