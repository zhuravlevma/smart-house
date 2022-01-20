mod lib;

use std::error::Error;
use std::{fs};
use tcp::server::Server;
use crate::lib::RosetteServer;

fn main() -> Result<(), Box<dyn Error>> {
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    RosetteServer::new(220, addr)?.listen();
    Ok(())
}