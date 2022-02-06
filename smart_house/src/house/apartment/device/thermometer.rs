use crate::Rosette;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use udp_wrapper::UdpServer;

pub struct Thermometer {
    pub name: String,
    description: String,
    temperature: Arc<Mutex<f32>>,
    ip: String,
    updating: bool,
}

impl Thermometer {
    pub fn new(name: String, temperature: f32, ip_address: String) -> Self {
        Self {
            name,
            description: "It's a thermometer".to_string(),
            temperature: Arc::new(Mutex::new(temperature)),
            ip: ip_address,
            updating: false,
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
    pub fn update_temperature(&mut self) -> Result<(), Box<dyn Error>> {
        if self.updating {
            return Ok(());
        }
        let server = UdpServer::new(self.ip.clone());
        let clone_mutex = self.temperature.clone();
        thread::spawn(move || loop {
            let (_usize, _address, data) = server.receive();
            let temp: f32 = data.parse().unwrap();
            let mut temperature = clone_mutex.lock().unwrap();
            *temperature = temp;
        });
        self.updating = true;
        Ok(())
    }
    pub fn get_temperature(&self) -> f32 {
        let arc_clone = self.temperature.clone();
        let data = arc_clone.lock().unwrap();
        *data
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