use super::TypeDevice;
use crate::Thermometer;

pub struct Rosette {
    pub name: String,
    _t_device: TypeDevice,
    _description: String,
    _power: u32,
}

impl Rosette {
    pub fn new(name: String) -> Self {
        Self {
            name,
            _t_device: TypeDevice::Rosette,
            _description: "It's a rosette".to_string(),
            _power: 0,
        }
    }
}

impl Rosette {
    fn _on(&mut self) -> bool {
        self._power = 220;
        true
    }

    fn _off(&mut self) -> bool {
        self._power = 0;
        false
    }

    fn _current_power(&self) -> u32 {
        self._power
    }
}

impl PartialEq<Self> for Rosette {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
impl PartialEq<Thermometer> for Rosette {
    fn eq(&self, other: &Thermometer) -> bool {
        self.name.eq(&other.name)
    }
}
impl Eq for Rosette {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rosette_on() {
        let mut rosette = Rosette::new("Rosette1".to_string());
        rosette._on();
        assert_eq!(rosette._current_power() > 0, true)
    }

    #[test]
    fn rosette_off() {
        let mut rosette = Rosette::new("Rosette1".to_string());
        rosette._off();
        assert_eq!(rosette._current_power() == 0, true)
    }

    #[test]
    fn rosette_get_current_power() {
        let mut rosette = Rosette::new("Rosette1".to_string());
        rosette._off();
        assert_eq!(0, rosette._current_power());
        rosette._on();
        assert_eq!(220, rosette._current_power())
    }
}
