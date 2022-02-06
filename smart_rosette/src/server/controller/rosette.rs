use crate::domain::RosetteService;
use tcp_wrapper::server::Request;

pub struct PowerController {
    rosette_service: RosetteService,
}
impl PowerController {
    pub fn new(rosette_service: RosetteService) -> Self {
        Self { rosette_service }
    }
    pub(crate) fn get_power(&mut self, _request: Request) -> String {
        self.rosette_service.get_power().to_string()
    }
    pub(crate) fn off(&self, _request: Request) -> String {
        self.rosette_service.off()
    }

    pub(crate) fn on(&self, _request: Request) -> String {
        self.rosette_service.on()
    }
}
