pub mod device;
use crate::result::{AddDataError, GetDataError, RemoveDataError};
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
        for old_device in &self.devices {
            let result_check = match &new_device {
                Device::Thermometer(new_thermometer) => {
                    self.check_device_eq(old_device, &new_thermometer.name)
                }
                Device::Rosette(new_rosette) => self.check_device_eq(old_device, &new_rosette.name),
            };

            if result_check {
                return Err(AddDataError::UniqueConstraint);
            }
        }
        self.devices.push(new_device);
        let last = self.devices.len() - 1;

        Ok(&self.devices[last])
    }
    pub fn remove_device(&mut self, device_name: String) -> Result<Device, RemoveDataError> {
        for (pos, device) in self.devices.iter().enumerate() {
            let result = self.check_device_eq(device, &device_name);
            if result {
                return Ok(self.devices.remove(pos));
            }
        }
        Err(RemoveDataError::NotFound)
    }
    pub fn _list_devices(&self) -> &Vec<Device> {
        &self.devices
    }
    pub fn get_device_by_name(&self, device_name: &str) -> Result<&Device, GetDataError> {
        for device in &self.devices {
            let result = self.check_device_eq(device, device_name);
            if result {
                return Ok(device);
            }
        }
        Err(GetDataError::NotFound)
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
    use crate::house::apartment::device::rosette::Rosette;
    use crate::house::apartment::device::thermometer::Thermometer;
    use crate::house::apartment::device::Device;
    use crate::house::apartment::Apartment;
    use crate::result::{AddDataError, GetDataError, RemoveDataError};

    #[test]
    fn add_device_successful() -> Result<(), AddDataError> {
        let mut apartment = Apartment::new("Haha".to_string());
        let rosette = Rosette::new("".to_string());
        apartment._add_device(Device::Rosette(rosette))?;
        Ok(())
    }

    #[test]
    fn add_device_unique_error() -> Result<(), AddDataError> {
        let mut apartment = Apartment::new("Haha".to_string());
        let rosette = Rosette::new("Device1".to_string());
        let thermometer = Thermometer::new("Device1".to_string(), 0.0);
        apartment._add_device(Device::Rosette(rosette)).unwrap();
        match apartment._add_device(Device::Thermometer(thermometer)) {
            Ok(_) => Err(AddDataError::UniqueConstraint),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn get_device_by_name_successful() -> Result<(), GetDataError> {
        let mut apartment = Apartment::new("Haha".to_string());
        let device_name = "Device1".to_string();
        let rosette = Rosette::new(device_name.clone());
        apartment._add_device(Device::Rosette(rosette)).unwrap();
        apartment.get_device_by_name(&device_name)?;
        Ok(())
    }

    #[test]
    fn get_device_by_name_error() -> Result<(), GetDataError> {
        let mut apartment = Apartment::new("Haha".to_string());
        let device_name = "Device1".to_string();
        let rosette = Rosette::new("Device2".to_string());

        apartment._add_device(Device::Rosette(rosette)).unwrap();

        match apartment.get_device_by_name(&device_name) {
            Ok(_) => Err(GetDataError::NotFound),
            Err(_) => Ok(())
        }
    }

    #[test]
    fn remove_device_successful() -> Result<(), RemoveDataError> {
        let mut apartment = Apartment::new("Haha".to_string());
        let rosette_name = "Rosette1".to_string();
        let rosette = Rosette::new(rosette_name.clone());
        let thermometer = Thermometer::new("Thermometer1".to_string(), 0.0);
        apartment._add_device(Device::Rosette(rosette)).unwrap();
        apartment._add_device(Device::Thermometer(thermometer)).unwrap();
        apartment.remove_device(rosette_name)?;
        Ok(())
    }

    #[test]
    fn remove_device_error() -> Result<(), RemoveDataError> {
        let mut apartment = Apartment::new("Haha".to_string());
        let rosette_name = "Rosette1".to_string();
        let search_name = "Rosette2".to_string();
        let rosette = Rosette::new(rosette_name.clone());
        apartment._add_device(Device::Rosette(rosette)).unwrap();
        match apartment.remove_device(search_name) {
            Ok(_) => {
                Err(RemoveDataError::NotFound)
            }
            Err(_) => Ok(())
        }
    }
}
