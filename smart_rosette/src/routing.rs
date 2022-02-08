use crate::rosette::RosetteController;
use tcp_server::routing::Routing;
use tcp_server::tcp_request::Request;

pub struct RosetteRouting {
    pub(crate) rosette_controller: RosetteController,
}

impl Routing for RosetteRouting {
    fn routing(&mut self, request: Request) -> String {
        match request.get_resource() {
            "rosette_off" => self.rosette_controller.off(request).to_string(),
            "rosette_on" => self.rosette_controller.on(request).to_string(),
            "get_power" => self.rosette_controller.get_power(request).to_string(),
            _ => "false".to_string(),
        }
    }
}
