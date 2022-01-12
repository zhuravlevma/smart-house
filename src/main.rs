use smart_house::{Apartment, Device, Rosette, Thermometer};
use std::process;

fn main() {
    let mut apartment_parents = Apartment::new("Parents".to_string());
    let rosette_parents1 = Rosette::new("Rosette_Parents1".to_string());
    let rosette_parents2 = Rosette::new("Rosette_Parents2".to_string());
    let thermometer_parents1 = Thermometer::new("Thermometer_Parents1".to_string(), 21.2);
    let thermometer_parents2 = Thermometer::new("Thermometer_Parents2".to_string(), 21.2);
    apartment_parents
        ._add_device(Device::Rosette(rosette_parents1))
        .unwrap_or_else(|err| {
            println!("Adding device return error: {}", err);
            process::exit(1);
        });
    apartment_parents
        ._add_device(Device::Rosette(rosette_parents2))
        .unwrap_or_else(|err| {
            println!("Adding device filed: {}", err);
            process::exit(1);
        });
    apartment_parents
        ._add_device(Device::Thermometer(thermometer_parents1))
        .unwrap_or_else(|err| {
            println!("Adding device filed: {}", err);
            process::exit(1);
        });
    apartment_parents
        ._add_device(Device::Thermometer(thermometer_parents2))
        .unwrap_or_else(|err| {
            println!("Adding device filed: {}", err);
            process::exit(1);
        });

    apartment_parents
        .get_device_by_name("Thermometer_Parents2")
        .unwrap_or_else(|err| {
            println!("Getting device filed: {}", err);
            process::exit(1);
        });
}
