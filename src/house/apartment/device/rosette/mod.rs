use super::TypeDevice;

pub struct Rosette {
    pub name: String,
    pub t_device: TypeDevice,
    pub description: String,
    pub power: u32,
}

impl Rosette {
    fn _on(&mut self) -> bool {
        self.power = 220;
        true
    }

    fn _off(&mut self) -> bool {
        self.power = 0;
        false
    }

    fn _current_power(&self) -> u32 {
        self.power
    }
}

#[cfg(test)]
mod tests {
    use crate::house::apartment::device::rosette::Rosette;
    use crate::house::apartment::device::TypeDevice;

    #[test]
    fn rosette_on() {
        let mut rosette = Rosette {
            name: "Rosette1".to_string(),
            t_device: TypeDevice::Thermometer,
            description: "".to_string(),
            power: 0,
        };

        rosette._on();

        assert_eq!(rosette.power > 0, true)
    }

    #[test]
    fn rosette_off() {
        let mut rosette = Rosette {
            name: "Rosette1".to_string(),
            t_device: TypeDevice::Thermometer,
            description: "".to_string(),
            power: 0,
        };

        rosette._off();

        assert_eq!(rosette.power == 0, true)
    }

    #[test]
    fn rosette_get_current_power() {
        let mut rosette = Rosette {
            name: "Rosette1".to_string(),
            t_device: TypeDevice::Thermometer,
            description: "".to_string(),
            power: 0,
        };
        rosette._off();
        assert_eq!(rosette.power, rosette._current_power());
        rosette._on();
        assert_eq!(rosette.power, rosette._current_power())
    }
}
