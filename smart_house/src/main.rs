use smart_house::{Apartment, Device, Rosette, Thermometer};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut apartment_parents = Apartment::new("Parents".to_string());
    let mut rosette_parents1 = Rosette::new("Rosette_Parents1".to_string());
    rosette_parents1._on();
    rosette_parents1._off();
    let rosette_parents2 = Rosette::new("Rosette_Parents2".to_string());
    let mut thermometer_parents1 = Thermometer::new("Thermometer_Parents1".to_string(), 21.2);
    thermometer_parents1.update_temperature();
    println!("{}", thermometer_parents1._get_temperature());
    let thermometer_parents2 = Thermometer::new("Thermometer_Parents2".to_string(), 21.2);

    apartment_parents
        ._add_device(Device::Rosette(rosette_parents1))
        .unwrap_or_else(|err| {
            panic!("Adding device return error: {}", err);
        });
    apartment_parents
        ._add_device(Device::Rosette(rosette_parents2))
        .unwrap_or_else(|err| {
            panic!("Adding device filed: {}", err);
        });
    apartment_parents
        ._add_device(Device::Thermometer(thermometer_parents1))
        .unwrap_or_else(|err| {
            panic!("Adding device filed: {}", err);
        });
    apartment_parents
        ._add_device(Device::Thermometer(thermometer_parents2))
        .unwrap_or_else(|err| {
            panic!("Adding device filed: {}", err);
        });

    apartment_parents
        .get_device_by_name("Thermometer_Parents2")
        .unwrap_or_else(|err| {
            panic!("Getting device filed: {}", err);
        });
    Ok(())
}
