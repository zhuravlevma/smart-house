use crate::Thermometer;
use tcp::client::Client;

pub struct Rosette {
    pub name: String,
    description: String,
    power: u32,
    ip: String,
}

impl Rosette {
    pub fn new(name: String, ip_address: String) -> Self {
        Self {
            name,
            description: "It's a rosette".to_string(),
            power: 0,
            ip: ip_address,
        }
    }
}

impl Rosette {
    fn get_connect_to_rosette(&self, address: String) -> Client {
        Client::connect(address).unwrap()
    }

    pub fn on(&mut self) -> bool {
        let mut client = self.get_connect_to_rosette(self.ip.clone());
        let res = client.send_request("on|||").unwrap();
        println!("My test res {}", res);
        self.power = 220;
        true
    }

    pub fn off(&mut self) -> bool {
        let mut client = self.get_connect_to_rosette(self.ip.clone());
        let res = client.send_request("off|||").unwrap();
        println!("My test res {}", res);
        self.power = 0;
        false
    }

    pub fn current_power(&self) -> u32 {
        let mut client = self.get_connect_to_rosette(self.ip.clone());
        let res = client.send_request("get_power|||").unwrap();
        println!("My test res {}", res);
        self.power
    }

    pub fn get_info(&self) -> String {
        self.description.clone()
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
    // use super::*;

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
