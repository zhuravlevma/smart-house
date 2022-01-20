use std::error::Error;
use std::fs;
use tcp::server::Server;

fn main() -> Result<(), Box<dyn Error>> {
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let _server = Server::bind(addr)?;
    println!("Hello, world!1212");
    Ok(())
}
