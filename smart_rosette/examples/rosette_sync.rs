use config_simple::ConfigServer;
use smart_rosette::{RosetteServer, Server};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = ConfigServer::new("127.0.0.1".to_string(), "8080".to_string())?;
    RosetteServer::new(config)?.listen();
    Ok(())
}
