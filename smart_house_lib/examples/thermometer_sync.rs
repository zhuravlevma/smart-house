use smart_house_lib::Thermometer;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut thermometer_parents = Thermometer::new(
        "Thermometer_Parents".to_string(),
        21.2,
        "127.0.0.1:8081".to_string(),
    );
    thermometer_parents.update_temperature()?;
    loop {
        sleep(Duration::from_secs(3));
        println!("{}", thermometer_parents.get_temperature());
    }
}
