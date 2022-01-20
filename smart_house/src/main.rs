use std::fs;
use smart_house::{Apartment, Device, Rosette, Thermometer};
use tcp::client::Client;
fn main() {
    let mut apartment_parents = Apartment::new("Parents".to_string());
    let rosette_parents1 = Rosette::new("Rosette_Parents1".to_string());
    let rosette_parents2 = Rosette::new("Rosette_Parents2".to_string());
    let thermometer_parents1 = Thermometer::new("Thermometer_Parents1".to_string(), 21.2);
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

    let address = fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let request = format!("fetch|||{}", "Hello".to_string());
    let mut client = Client::connect(address).unwrap();
    let res = client.send_request(request).unwrap();
    println!("{}", res);
}
