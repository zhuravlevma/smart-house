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
    use super::*;
    use crate::house::apartment::device::rosette::Rosette;
    use crate::house::apartment::device::thermometer::Thermometer;
    use crate::house::apartment::device::TypeDevice;

    #[test]
    fn add_device_successful() {
        let mut apartment = Apartment {
            name: "Haha".to_string(),
            devices: vec![],
        };
        let rosette = Rosette {
            name: "".to_string(),
            t_device: TypeDevice::Rosette,
            description: "".to_string(),
            power: 0,
        };

        match apartment._add_device(Device::Rosette(rosette)) {
            Ok(_) => {}
            Err(error) => match error {
                AddDataError::UniqueConstraint => {
                    panic!("AddDataError error: UniqueConstraint")
                }
            },
        };
    }

    #[test]
    fn add_device_unique_error() {
        let mut apartment = Apartment {
            name: "Haha".to_string(),
            devices: vec![],
        };
        let rosette = Rosette {
            name: "Device1".to_string(),
            t_device: TypeDevice::Rosette,
            description: "".to_string(),
            power: 0,
        };
        let thermometer = Thermometer {
            name: "Device1".to_string(),
            t_device: TypeDevice::Thermometer,
            description: "".to_string(),
            temperature: 0.0,
        };

        let _result1 = apartment._add_device(Device::Rosette(rosette));
        match apartment._add_device(Device::Thermometer(thermometer)) {
            Ok(_) => {
                panic!("Add device should get error");
            }
            Err(_) => {}
        };
    }

    #[test]
    fn get_device_by_name_successful() {
        let mut apartment = Apartment {
            name: "Haha".to_string(),
            devices: vec![],
        };
        let device_name = "Device1".to_string();
        let rosette = Rosette {
            name: device_name.clone(),
            t_device: TypeDevice::Rosette,
            description: "".to_string(),
            power: 0,
        };

        match apartment._add_device(Device::Rosette(rosette)) {
            Ok(_) => {}
            Err(error) => match error {
                AddDataError::UniqueConstraint => {
                    panic!("AddDataError Error UniqueConstraint")
                }
            },
        }

        match apartment.get_device_by_name(&device_name) {
            Ok(_) => {}
            Err(error) => match error {
                GetDataError::NotFound => {
                    panic!("GetDataError Error: Not Found")
                }
            },
        };
    }

    #[test]
    fn _get_device_by_name_error() {
        let mut apartment = Apartment {
            name: "Haha".to_string(),
            devices: vec![],
        };
        let device_name = "Device1".to_string();
        let rosette = Rosette {
            name: "Device2".to_string(),
            t_device: TypeDevice::Rosette,
            description: "".to_string(),
            power: 0,
        };

        match apartment._add_device(Device::Rosette(rosette)) {
            Ok(_) => {}
            Err(error) => match error {
                AddDataError::UniqueConstraint => {
                    panic!("AddDataError Error: UniqueConstraint")
                }
            },
        };

        match apartment.get_device_by_name(&device_name) {
            Ok(_) => {
                panic!("Get device by name should get error")
            }
            Err(_) => {}
        }
    }

    #[test]
    fn _remove_device_successful() {
        let mut apartment = Apartment {
            name: "Haha".to_string(),
            devices: vec![],
        };
        let rosette_name = "Rosette1".to_string();
        let rosette = Rosette {
            name: rosette_name.clone(),
            t_device: TypeDevice::Rosette,
            description: "".to_string(),
            power: 0,
        };
        let thermometer = Thermometer {
            name: "Thermometer1".to_string(),
            t_device: TypeDevice::Thermometer,
            description: "".to_string(),
            temperature: 0.0,
        };

        match apartment._add_device(Device::Rosette(rosette)) {
            Ok(_) => {}
            Err(error) => match error {
                AddDataError::UniqueConstraint => {
                    panic!("AddDataError Error: UniqueError");
                }
            },
        }

        match apartment._add_device(Device::Thermometer(thermometer)) {
            Ok(_) => {}
            Err(error) => match error {
                AddDataError::UniqueConstraint => {
                    panic!("AddDataError Error: UniqueError");
                }
            },
        }

        match apartment.remove_device(rosette_name) {
            Ok(_) => {}
            Err(error) => match error {
                RemoveDataError::NotFound => {
                    panic!("RemoveDataError Error: NotFound");
                }
            },
        };

        assert_eq!(apartment._list_devices().len(), 1);
    }

    #[test]
    fn _remove_device_error() {
        let mut apartment = Apartment {
            name: "Haha".to_string(),
            devices: vec![],
        };
        let rosette_name = "Rosette1".to_string();
        let search_name = "Rosette2".to_string();
        let rosette = Rosette {
            name: rosette_name.clone(),
            t_device: TypeDevice::Rosette,
            description: "".to_string(),
            power: 0,
        };

        match apartment._add_device(Device::Rosette(rosette)) {
            Ok(_) => {}
            Err(error) => match error {
                AddDataError::UniqueConstraint => {
                    panic!("AddDataError: UniqueError");
                }
            },
        }

        match apartment.remove_device(search_name) {
            Ok(_) => {
                panic!("Remove device by name should get error")
            }
            Err(_) => {}
        };
    }
}
