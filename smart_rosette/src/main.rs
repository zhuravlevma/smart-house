use smart_rosette::RosetteServer;
use smart_rosette::Server;
use smart_rosette::Config;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new("127.0.0.1".to_string(), "55331".to_string())?;
    RosetteServer::new(config)?.listen();
    Ok(())
}
