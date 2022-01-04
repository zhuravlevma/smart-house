use super::TypeDevice;

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

impl Thermometer {
    pub fn _get_temperature(&self) -> f32 {
        self.temperature
    }
}

#[cfg(test)]
mod tests {
    use crate::house::apartment::device::thermometer::Thermometer;

    #[test]
    fn _get_current_temperature() {
        let temperature = 21.0;
        let thermometer = Thermometer::new("Thermometer".to_string(), temperature);
        assert_eq!(temperature, thermometer._get_temperature())
    }
}
