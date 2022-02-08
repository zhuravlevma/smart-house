use crate::rosette::{RosetteController, RosetteService};
use routing::RosetteRouting;
use std::error::Error;
use tcp_server::server::Server;

fn main() -> Result<(), Box<dyn Error>> {
    let routing = RosetteRouting {
        rosette_controller: RosetteController::new(RosetteService::new()),
    };
    let server = Server::new("127.0.0.1:8084")?;
    server.listen(Box::new(routing))?;
    Ok(())
}

mod rosette;
mod routing;
