use super::TypeDevice;

pub struct Thermometer {
    pub name: String,
    pub t_device: TypeDevice,
    pub description: String,
    pub temperature: f32,
}

impl Thermometer {
    fn _get_temperature(&self) -> f32 {
        self.temperature
    }
}

#[cfg(test)]
mod tests {
    use crate::house::apartment::device::thermometer::Thermometer;
    use crate::house::apartment::device::TypeDevice;

    #[test]
    fn _get_current_temperature() {
        let temperature = 21.0;
        let thermometer = Thermometer {
            name: "Thermometer".to_string(),
            t_device: TypeDevice::Thermometer,
            description: "".to_string(),
            temperature,
        };

        assert_eq!(temperature, thermometer._get_temperature())
    }
}
