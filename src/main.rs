use smart_house::errors::GetDataError;
use smart_house::Apartment;
use smart_house::Device;
use smart_house::Rosette;
use smart_house::Thermometer;

fn main() {
    let rosette1 = Rosette::new("Rosette1".to_string());

    let thermometer1 = Thermometer::new("Thermometer1".to_string(), 24.4);

    let mut apartment1 = Apartment::new("Apartment1".to_string());

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

    let name_device = "Thermometer1".to_string();

    let _result_get = apartment1.get_device_by_name(&"Thermometer1".to_string());
    find_by_name(&apartment1, name_device).unwrap();
}

fn find_by_name(apartment: &Apartment, s: String) -> Result<&Device, GetDataError> {
    let test = apartment.get_device_by_name(&s)?;
    Ok(test)
}
