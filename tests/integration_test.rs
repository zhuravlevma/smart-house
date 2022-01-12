use smart_house::house::apartment::device::rosette::Rosette;
use smart_house::house::apartment::device::thermometer::Thermometer;
use smart_house::house::apartment::device::Device;
use smart_house::house::apartment::Apartment;
use std::error::Error;

#[test]
fn create_apartment_with_devices_and_find() -> Result<(), Box<dyn Error>> {
    let mut apartment_parents = Apartment::new("Parents".to_string());
    let rosette_parents1 = Rosette::new("Rosette_Parents1".to_string());
    let rosette_parents2 = Rosette::new("Rosette_Parents2".to_string());
    let thermometer_parents1 = Thermometer::new("Thermometer_Parents1".to_string(), 21.2);
    let thermometer_parents2 = Thermometer::new("Thermometer_Parents2".to_string(), 21.2);
    apartment_parents._add_device(Device::Rosette(rosette_parents1))?;
    apartment_parents._add_device(Device::Rosette(rosette_parents2))?;
    apartment_parents._add_device(Device::Thermometer(thermometer_parents1))?;
    apartment_parents._add_device(Device::Thermometer(thermometer_parents2))?;

    assert_eq!(
        "Thermometer_Parents2",
        match apartment_parents.get_device_by_name("Thermometer_Parents2")? {
            Device::Rosette(_) => panic!("It's thermometer"),
            Device::Thermometer(thermometer) => &thermometer.name,
        }
    );
    assert_eq!(
        "Rosette_Parents1",
        match apartment_parents.get_device_by_name("Rosette_Parents1")? {
            Device::Rosette(rosette) => &rosette.name,
            Device::Thermometer(_) => panic!("It's rosette"),
        }
    );

    Ok(())
}
