pub use config::Config;
pub use server::RosetteServer;
pub use tcp::server::Server;
pub struct Rosette {
    power: u32,
}

impl Rosette {
    pub fn new(power: u32) -> Self {
        Self { power }
    }
}

mod config;
mod domain;
mod server;
