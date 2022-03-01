use smart_house_lib::Thermometer;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut thermometer_parents = Thermometer::new(
        "Thermometer_Parents".to_string(),
        21.2,
        "127.0.0.1:8081".to_string(),
    );

    thermometer_parents.update_temperature_async().await?;

    Ok(())
}
