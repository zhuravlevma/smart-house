use crate::Thermometer;
use log::info;
use serde::Serialize;
use tcp_wrapper::client_std::Client;

#[derive(Serialize)]
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
        info!("Rosette IP {} start on", self.ip);
        let mut client = self.get_connect_to_rosette(self.ip.clone());
        let res = client.send_request("rosette_on|||").unwrap();
        info!("Rosette IP on success: {}", res);
        self.power = 220;
        true
    }

    pub fn off(&mut self) -> bool {
        info!("Rosette IP {} start off", self.ip);
        let mut client = self.get_connect_to_rosette(self.ip.clone());
        let res = client.send_request("rosette_off|||").unwrap();
        info!("Rosette IP off success: {}", res);
        self.power = 0;
        true
    }

    pub fn current_power(&mut self) -> u32 {
        info!("Rosette IP {} start getting power", self.ip);
        let mut client = self.get_connect_to_rosette(self.ip.clone());
        let res = client.send_request("get_power|||").unwrap();
        info!("Rosette IP {} getting power success: {}", self.ip, res);
        let num:u32 = res.parse().unwrap();
        self.power = num;
        self.power
    }

    pub fn get_info(&self) -> String {
        info!("Getting info for rosette {}", self.ip);
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
