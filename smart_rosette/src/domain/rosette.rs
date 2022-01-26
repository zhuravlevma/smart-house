use crate::Rosette;

pub struct RosetteService {}

impl RosetteService {
    pub fn new() -> Self {
        Self {}
    }
    pub(crate) fn get_power(&self, data: String) -> String {
        let rosette = Rosette::new(220);
        format!("{} - {}", data, rosette.power)
    }
    pub(crate) fn off(&self) -> String {
        let rosette = Rosette::new(0);
        format!("{}", rosette.power)
    }
}
