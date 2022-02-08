use std::error::Error;
use tcp_server::routing::Routing;
use tcp_server::server::Server;
use tcp_server::tcp_request::Request;

pub struct CatService {}

pub struct CustomRouting {
    cat_controller: CatController,
}

impl Routing for CustomRouting {
    fn routing(&mut self, request: Request) -> String {
        let resource = request.get_resource();

        match resource {
            "rosette_off" => self.cat_controller.off(request).to_string(),
            _ => false.to_string(),
        }
    }
}

impl CustomRouting {
    pub fn new(cat_controller: CatController) -> Self {
        Self { cat_controller }
    }
}

impl CatService {
    pub fn off(&self) -> bool {
        true
    }
}

pub struct CatController {
    cat_service: CatService,
}

impl CatController {
    pub fn off(&self, _req: Request) -> bool {
        self.cat_service.off()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let routing = CustomRouting::new(CatController {
        cat_service: CatService {},
    });
    let server = Server::new("127.0.0.1:8084")?;
    server.listen(Box::new(routing))?;
    Ok(())
}
