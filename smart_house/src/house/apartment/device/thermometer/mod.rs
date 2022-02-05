use crate::Rosette;
use udp::UdpClient;

pub struct Thermometer {
    pub name: String,
    description: String,
    temperature: f32,
    ip: String,
}

impl Thermometer {
    pub fn new(name: String, temperature: f32, ip_address: String) -> Self {
        Self {
            name,
            description: "It's a thermometer".to_string(),
            temperature,
            ip: ip_address,
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
    pub fn update_temperature(&mut self) -> f32 {
        let client = UdpClient::new("127.0.0.1:55333".to_string());
        let temp: f32 = client
            .send("Signal".to_string(), self.ip.clone())
            .parse()
            .unwrap();
        self.temperature = temp;
        self.temperature
    }
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
    pub fn get_info(&self) -> String {
        self.description.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _get_current_temperature() {
        let temperature = 21.0;
        let thermometer = Thermometer::new(
            "Thermometer".to_string(),
            temperature,
            "127.0.0.1:8080".to_string(),
        );
        assert_eq!(temperature, thermometer.get_temperature())
    }
}
