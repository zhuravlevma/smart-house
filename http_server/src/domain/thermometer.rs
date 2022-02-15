use crate::MongoThermometer;
use smart_house::Thermometer;
use std::error::Error;

pub struct ThermometerService {
    db_service: MongoThermometer,
}

impl ThermometerService {
    pub async fn new(connection_str: &str) -> Self {
        Self {
            db_service: MongoThermometer::new(connection_str).await,
        }
    }

    pub async fn get_temperature(
        &self,
        house_id: &str,
        apartment_name: &str,
        thermometer_name: &str,
    ) -> Result<f32, Box<dyn Error>> {
        let data = self
            .db_service
            .get_thermometer(house_id, apartment_name, thermometer_name)
            .await?;
        let mut domain_rosette = Thermometer::new(data.name, data.temperature, data.ip_address);
        domain_rosette.update_temperature_async().await?;
        Ok(domain_rosette.get_temperature())
    }
}
