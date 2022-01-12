use crate::errors::{AddDataError, GetDataError, RemoveDataError};
use device::Device;

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

impl Apartment {
    pub fn _add_device(&mut self, new_device: Device) -> Result<&Device, AddDataError> {
        let device = self.devices.iter().find(|old_device| {
             match &new_device {
                Device::Thermometer(new_thermometer) => {
                    self.check_device_eq(old_device, &new_thermometer.name)
                }
                Device::Rosette(new_rosette) => self.check_device_eq(old_device, &new_rosette.name),
            }
        });
        match device {
            None => {
                self.devices.push(new_device);
                let last = self.devices.len() - 1;

                Ok(&self.devices[last])
            },
            Some(_) => {
                Err(AddDataError::UniqueConstraint)
            }
        }
    }
    pub fn remove_device(&mut self, device_name: String) -> Result<Device, RemoveDataError> {
        let position = self.devices.iter()
            .position(|device| self.check_device_eq(device, &device_name));
        match position {
            None => Err(RemoveDataError::NotFound),
            Some(position) => Ok(self.devices.remove(position))
        }
    }
    pub fn _list_devices(&self) -> &Vec<Device> {
        &self.devices
    }
    pub fn get_device_by_name(&self, device_name: &str) -> Result<&Device, GetDataError> {
        let device = self.devices.iter().find(|device| self.check_device_eq(device, device_name));
        match device {
            None => Err(GetDataError::NotFound),
            Some(device) => Ok(device)
        }
    }
    fn check_device_eq(&self, old_device: &Device, new_name: &str) -> bool {
        match old_device {
            Device::Thermometer(old_thermometer) => {
                self.name_is_eq(new_name, &old_thermometer.name)
            }
            Device::Rosette(old_rosette) => self.name_is_eq(new_name, &old_rosette.name),
        }
    }
    fn name_is_eq(&self, str1: &str, str2: &str) -> bool {
        str1.eq(str2)
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
        let rosette = Rosette::new("".to_string());

        apartment._add_device(Device::Rosette(rosette))?;

        Ok(())
    }

    #[test]
    fn add_device_unique_error() -> Result<(), Box<dyn Error>> {
        let mut apartment = Apartment::new("Haha".to_string());
        let rosette = Rosette::new("Device1".to_string());
        let thermometer = Thermometer::new("Device1".to_string(), 0.0);

        apartment._add_device(Device::Rosette(rosette))?;
        assert_eq!(
            apartment
                ._add_device(Device::Thermometer(thermometer))
                .is_err(),
            true
        );

        Ok(())
    }

    #[test]
    fn get_device_by_name_successful() -> Result<(), Box<dyn Error>> {
        let mut apartment = Apartment::new("Haha".to_string());
        let device_name = "Device1".to_string();
        let rosette = Rosette::new(device_name.clone());
        apartment._add_device(Device::Rosette(rosette))?;

        apartment.get_device_by_name(&device_name)?;

        Ok(())
    }

    #[test]
    fn get_device_by_name_error() -> Result<(), Box<dyn Error>> {
        let mut apartment = Apartment::new("Haha".to_string());
        let device_name = "Device1".to_string();
        let rosette = Rosette::new("Device2".to_string());

        apartment._add_device(Device::Rosette(rosette))?;

        assert_eq!(apartment.get_device_by_name(&device_name).is_err(), true);
        Ok(())
    }

    #[test]
    fn remove_device_successful() -> Result<(), Box<dyn Error>> {
        let mut apartment = Apartment::new("Haha".to_string());
        let rosette_name = "Rosette1".to_string();
        let rosette = Rosette::new(rosette_name.clone());
        let thermometer = Thermometer::new("Thermometer1".to_string(), 0.0);
        apartment._add_device(Device::Rosette(rosette))?;
        apartment._add_device(Device::Thermometer(thermometer))?;

        apartment.remove_device(rosette_name)?;

        Ok(())
    }

    #[test]
    fn remove_device_error() -> Result<(), Box<dyn Error>> {
        let mut apartment = Apartment::new("Haha".to_string());
        let rosette_name = "Rosette1".to_string();
        let search_name = "Rosette2".to_string();
        let rosette = Rosette::new(rosette_name.clone());
        apartment._add_device(Device::Rosette(rosette))?;

        assert_eq!(apartment.remove_device(search_name).is_err(), true);

        Ok(())
    }
}

pub mod device;
