use crate::server::controller::rosette::PowerController;
use crate::{Request, RequestHandler};

pub struct RosetteHandler {
    pub power_controller: PowerController,
}

impl RequestHandler for RosetteHandler {
    fn routing(&mut self, mut request: Request) -> String {
        let command = request.next_data();
        match command {
            "get_power" => self.power_controller.get_power(request),
            "off" => self.power_controller.off(request),
            "on" => self.power_controller.on(request),
            _ => "Bad command".into(),
        }
    }
}