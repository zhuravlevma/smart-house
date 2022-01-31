use super::TypeDevice;
use crate::Thermometer;
use std::fs;
use tcp::client::Client;

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
    fn get_ip_address(&self) -> String {
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55332"))
    }
    fn get_connect_to_rosette(&self, address: String) -> Client {
        Client::connect(address).unwrap()
    }
    pub fn _on(&mut self) -> bool {
        let mut client = self.get_connect_to_rosette(self.get_ip_address());
        let res = client.send_request("on|||").unwrap();
        println!("My test res {}", res);
        self._power = 220;
        true
    }

    pub fn _off(&mut self) -> bool {
        let mut client = self.get_connect_to_rosette(self.get_ip_address());
        let res = client.send_request("off|||").unwrap();
        println!("My test res {}", res);
        self._power = 0;
        false
    }

    pub fn _current_power(&self) -> u32 {
        let mut client = self.get_connect_to_rosette(self.get_ip_address());
        let res = client.send_request("get_power|||").unwrap();
        println!("My test res {}", res);
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
        // let mut rosette = Rosette::new("Rosette1".to_string());
        // rosette._on();
        // assert_eq!(rosette._current_power() > 0, true)
    }

    #[test]
    fn rosette_off() {
        // let mut rosette = Rosette::new("Rosette1".to_string());
        // rosette._off();
        // assert_eq!(rosette._current_power() == 0, true)
    }

    #[test]
    fn rosette_get_current_power() {
        // let mut rosette = Rosette::new("Rosette1".to_string());
        // rosette._off();
        // assert_eq!(0, rosette._current_power());
        // rosette._on();
        // assert_eq!(220, rosette._current_power())
    }
}
