use crate::errors::{
    AddDataError, AddDataResult, GetDataError, GetDataResult, RemoveDataError, RemoveDataResult,
};
use device::Device;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Apartment {
    pub name: String,
    devices: Vec<Device>,
}

impl Apartment {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: vec![],
        }
    }
}

/// Create Apartment
/// ```
/// use smart_house_lib::Apartment;
/// let apartment = Apartment::new("name".to_string());
/// ```
/// Add devices to apartment
/// ```
/// use smart_house_lib::{Apartment, Device, Rosette};
/// let mut apartment = Apartment::new("name".to_string());
/// let device = Device::Rosette(Rosette::new("test".to_string(), "127.0.0.1:8080".to_string()));
/// apartment.add_device(device);
/// ```
/// Remove device from apartment
/// ```
/// use smart_house_lib::{Apartment, Device, Rosette};
/// let mut apartment = Apartment::new("name".to_string());
/// let device = Device::Rosette(Rosette::new("test".to_string(), "127.0.0.1:8080".to_string()));
/// apartment.add_device(device);
/// apartment.remove_device("test".to_string());
/// ```
impl Apartment {
    pub fn add_device(&mut self, new_device: Device) -> AddDataResult<&Device> {
        info!("Adding device for apartment {}", self.name);
        let device = self
            .devices
            .iter()
            .find(|&old_device| old_device == &new_device);
        match device {
            None => {
                self.devices.push(new_device);
                let last = self.devices.len() - 1;

                Ok(&self.devices[last])
            }
            Some(_) => Err(AddDataError::UniqueConstraint),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn remove_device(&mut self, device_name: String) -> RemoveDataResult<Device> {
        info!(
            "Remove device with name {} for apartment {}",
            device_name, self.name
        );
        let position = self
            .devices
            .iter()
            .position(|device| Self::check_device_name(device, &device_name));
        match position {
            None => Err(RemoveDataError::NotFound),
            Some(position) => Ok(self.devices.remove(position)),
        }
    }
    pub fn list_devices(&self) -> &[Device] {
        info!("Get list devices for apartment {}", self.name);
        &self.devices
    }
    pub fn get_device_by_name(&self, device_name: &str) -> GetDataResult<&Device> {
        info!(
            "Getting device with name {} for apartment {}",
            device_name, self.name
        );
        let device = self
            .devices
            .iter()
            .find(|device| Self::check_device_name(device, device_name));
        match device {
            None => Err(GetDataError::NotFound),
            Some(device) => Ok(device),
        }
    }
    fn check_device_name(old_device: &Device, new_name: &str) -> bool {
        match old_device {
            Device::Thermometer(old_thermometer) => new_name.eq(&old_thermometer.name),
            Device::Rosette(old_rosette) => new_name.eq(&old_rosette.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use device::rosette::Rosette;
    use device::thermometer::Thermometer;
    use std::error::Error;

    #[test]
    fn add_device_successful() -> Result<(), Box<dyn Error>> {
        let mut apartment = Apartment::new("Haha".to_string());
        let rosette = Rosette::new("".to_string(), "127.0.0.1:8080".to_string());

        apartment.add_device(Device::Rosette(rosette))?;

        Ok(())
    }

    #[test]
    fn add_device_unique_error() -> Result<(), Box<dyn Error>> {
        let mut apartment = Apartment::new("Haha".to_string());
        let rosette = Rosette::new("Device1".to_string(), "127.0.0.1:8080".to_string());
        let thermometer =
            Thermometer::new("Device1".to_string(), 0.0, "127.0.0.1:8080".to_string());

        apartment.add_device(Device::Rosette(rosette))?;
        assert_eq!(
            apartment
                .add_device(Device::Thermometer(thermometer))
                .is_err(),
            true
        );

        Ok(())
    }

    #[test]
    fn get_device_by_name_successful() -> Result<(), Box<dyn Error>> {
        let mut apartment = Apartment::new("Haha".to_string());
        let device_name = "Device1".to_string();
        let rosette = Rosette::new(device_name.clone(), "127.0.0.1:8080".to_string());
        apartment.add_device(Device::Rosette(rosette))?;

        apartment.get_device_by_name(&device_name)?;

        Ok(())
    }

    #[test]
    fn get_device_by_name_error() -> Result<(), Box<dyn Error>> {
        let mut apartment = Apartment::new("Haha".to_string());
        let device_name = "Device1".to_string();
        let rosette = Rosette::new("Device2".to_string(), "127.0.0.1:8080".to_string());

        apartment.add_device(Device::Rosette(rosette))?;

        assert_eq!(apartment.get_device_by_name(&device_name).is_err(), true);
        Ok(())
    }

    #[test]
    fn remove_device_successful() -> Result<(), Box<dyn Error>> {
        let mut apartment = Apartment::new("Haha".to_string());
        let rosette_name = "Rosette1".to_string();
        let rosette = Rosette::new(rosette_name.clone(), "127.0.0.1:8080".to_string());
        let thermometer = Thermometer::new(
            "Thermometer1".to_string(),
            0.0,
            "127.0.0.1:9000".to_string(),
        );
        apartment.add_device(Device::Rosette(rosette))?;
        apartment.add_device(Device::Thermometer(thermometer))?;

        apartment.remove_device(rosette_name)?;

        Ok(())
    }

    #[test]
    fn remove_device_error() -> Result<(), Box<dyn Error>> {
        let mut apartment = Apartment::new("Haha".to_string());
        let rosette_name = "Rosette1".to_string();
        let search_name = "Rosette2".to_string();
        let rosette = Rosette::new(rosette_name.clone(), "127.0.0.1:8080".to_string());
        apartment.add_device(Device::Rosette(rosette))?;

        assert_eq!(apartment.remove_device(search_name).is_err(), true);

        Ok(())
    }
}

pub mod device;
