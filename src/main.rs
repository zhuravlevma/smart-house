use smart_house::house::apartment::device::rosette::Rosette;
use smart_house::house::apartment::device::thermometer::Thermometer;
use smart_house::house::apartment::device::{Device, TypeDevice};
use smart_house::house::apartment::Apartment;

fn main() {
    let rosette1 = Rosette {
        name: "Rosette1".to_string(),
        t_device: TypeDevice::Thermometer,
        description: "It is Rosette1. It's small but very powerful".to_string(),
    };

    let thermometer1 = Thermometer {
        name: "Thermometer1".to_string(),
        t_device: TypeDevice::Thermometer,
        description: "It is Thermometer1. It's simple".to_string(),
    };

    let mut apartment1 = Apartment {
        name: "".to_string(),
        devices: vec![],
    };

    let result_rosette1 = apartment1._add_device(Device::Rosette(rosette1));
    match result_rosette1 {
        Ok(status) => {
            if let Device::Rosette(rosette) = status {
                println!("{}", rosette.name)
            }
        }
        Err(_error) => println!("error"),
    }
    let result_thermometer1 = apartment1._add_device(Device::Thermometer(thermometer1));
    match result_thermometer1 {
        Ok(status) => {
            if let Device::Thermometer(thermometer) = status {
                println!("{}", thermometer.name)
            }
        }
        Err(_error) => println!("error"),
    }

    let _result_get = apartment1.get_device_by_name(&"Thermometer1".to_string());
}
