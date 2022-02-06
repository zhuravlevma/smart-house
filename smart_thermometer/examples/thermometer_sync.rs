use config_simple::ConfigServer;
use smart_thermometer::ThermometerServer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = ConfigServer::new("127.0.0.1".to_string(), "8080".to_string())?;
    let server = ThermometerServer::new(config);
    server.listen();

    Ok(())
}
