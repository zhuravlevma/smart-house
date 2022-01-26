use crate::server::controller::rosette::PowerController;
pub use tcp::server::Server;
use tcp::server::{Request, RequestHandler};

pub struct RosetteHandler {
    pub power_controller: PowerController,
}

impl RequestHandler for RosetteHandler {
    fn routing(&mut self, mut request: Request) -> String {
        let command = request.next_data();
        match command {
            "get_power" => self.power_controller.get_power(request),
            "off" => self.power_controller.off(request),
            _ => "Bad command".into(),
        }
    }
}
