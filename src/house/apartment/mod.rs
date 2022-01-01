pub mod device;
use crate::result::{AddDataError, GetDataError, RemoveDataError};
use device::Device;

pub struct Apartment {
    pub name: String,
    pub devices: Vec<Device>,
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
    #[test]
    fn _add_device_successful() {}

    #[test]
    fn _add_device_error() {}

    #[test]
    fn _get_device_by_name_successful() {}

    #[test]
    fn _get_device_by_name_error() {}

    #[test]
    fn _remove_device_successful() {}

    #[test]
    fn _remove_device_error() {}
}
