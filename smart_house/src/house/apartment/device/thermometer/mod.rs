use super::TypeDevice;
use crate::Rosette;

pub struct Thermometer {
    pub name: String,
    _t_device: TypeDevice,
    _description: String,
    temperature: f32,
}

impl Thermometer {
    pub fn new(name: String, temperature: f32) -> Self {
        Self {
            name,
            _t_device: TypeDevice::Thermometer,
            _description: "It's a thermometer".to_string(),
            temperature,
        }
    }
}

impl PartialEq<Self> for Thermometer {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
impl Eq for Thermometer {}

impl PartialEq<Rosette> for Thermometer {
    fn eq(&self, other: &Rosette) -> bool {
        self.name.eq(&other.name)
    }
}

impl Thermometer {
    pub fn _get_temperature(&self) -> f32 {
        self.temperature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _get_current_temperature() {
        let temperature = 21.0;
        let thermometer = Thermometer::new("Thermometer".to_string(), temperature);
        assert_eq!(temperature, thermometer._get_temperature())
    }
}
