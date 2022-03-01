use crate::errors::ThermometerError;
use crate::Rosette;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use udp_wrapper::{UdpServer, UdpServerAsync};

mod mutex_lock_serde {
    use serde::ser::Serializer;
    use serde::{Deserialize, Deserializer, Serialize};
    use std::sync::{Arc, Mutex};
    pub fn serialize<S, T>(val: &Arc<Mutex<T>>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        T::serialize(&*val.lock().unwrap(), s)
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<Arc<Mutex<T>>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        Ok(Arc::new(Mutex::new(T::deserialize(d)?)))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Thermometer {
    pub name: String,
    description: String,
    #[serde(with = "mutex_lock_serde")]
    temperature: Arc<Mutex<f32>>,
    ip: String,
    updating: bool,
}

/// Create thermometer
/// ```
/// use smart_house_lib::Thermometer;
/// let thermometer = Thermometer::new("name".to_string(), 23.0, "127.0.0.1:9091".to_string());
/// ```
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

impl Thermometer {
    pub fn update_temperature(&mut self) -> Result<JoinHandle<()>, ThermometerError> {
        if self.updating {
            error!("Your thermometer already use simple updating temperature");
            return Err(ThermometerError::Busy(self.ip.clone()));
        }
        let server = UdpServer::new(self.ip.clone())?;
        let clone_mutex = self.temperature.clone();
        let thread: JoinHandle<()> = thread::spawn(move || loop {
            let (_usize, _address, data) = server.receive().unwrap();
            let temp: f32 = data.parse().unwrap();
            let mut temperature = clone_mutex.lock().unwrap();
            *temperature = temp;
        });
        self.updating = true;
        info!(
            "Start simple updating temperature for thermometer {}",
            self.name
        );
        Ok(thread)
    }

    pub async fn update_temperature_async(&mut self) -> Result<(), ThermometerError> {
        if self.updating {
            error!("Your thermometer already use async updating temperature");
            return Err(ThermometerError::Busy(self.ip.clone()));
        }
        let socket = UdpServerAsync::new(self.ip.clone()).await?;

        self.updating = true;
        info!(
            "Start async updating temperature for thermometer {}",
            self.name
        );
        println!("Current temp: {}", self.get_temperature());
        let (_usize, _src_address, data) = socket.receive().await?;
        match data.parse::<f32>() {
            Ok(num) => {
                let arc = self.temperature.clone();
                let mut data = arc.lock().unwrap();
                *data = num;
                Ok(())
            }
            Err(_) => Err(ThermometerError::ParseFloatError(data)),
        }
    }

    pub fn get_temperature(&self) -> f32 {
        info!("Getting temperature for thermometer {}", self.name);
        let arc_clone = self.temperature.clone();
        let data = arc_clone.lock().unwrap();
        *data
    }
    pub fn get_info(&self) -> String {
        info!("Getting info for thermometer {}", self.name);
        self.description.clone()
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_ip(&self) -> &str {
        &self.ip
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
