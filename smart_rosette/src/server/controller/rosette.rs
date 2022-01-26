use crate::domain::RosetteService;
use tcp::server::Request;

pub struct PowerController {
    rosette_service: RosetteService,
}
impl PowerController {
    pub fn new(rosette_service: RosetteService) -> Self {
        Self { rosette_service }
    }
    pub(crate) fn get_power(&mut self, mut request: Request) -> String {
        let data = request.next_data();
        if data.is_empty() {
            return "data empty".into();
        }
        self.rosette_service.get_power(data.to_string())
    }
    pub(crate) fn off(&self, mut _request: Request) -> String {
        self.rosette_service.off()
    }
}
