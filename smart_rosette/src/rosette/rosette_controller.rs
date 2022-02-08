use crate::rosette::RosetteService;
use tcp_server::tcp_request::Request;

pub struct RosetteController {
    service: RosetteService,
}

impl RosetteController {
    pub fn new(rosette_service: RosetteService) -> Self {
        Self {
            service: rosette_service,
        }
    }
    pub fn off(&mut self, _req: Request) -> u32 {
        self.service.off()
    }

    pub fn on(&mut self, _req: Request) -> u32 {
        self.service.on()
    }

    pub fn get_power(&self, _req: Request) -> u32 {
        self.service.get_power()
    }
}
