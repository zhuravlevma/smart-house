use crate::error::DomainError;
use crate::{MongoClient, MongoThermometer};
use smart_house_lib::Thermometer;

pub struct ThermometerService {
    db_service: MongoThermometer,
}

impl ThermometerService {
    pub async fn new(mongo_client: MongoClient) -> Self {
        Self {
            db_service: MongoThermometer::new(mongo_client).await,
        }
    }

    pub async fn get_temperature(
        &self,
        house_id: &str,
        apartment_name: &str,
        thermometer_name: &str,
    ) -> Result<f32, DomainError> {
        let data = self
            .db_service
            .get_thermometer(house_id, apartment_name, thermometer_name)
            .await?;
        let mut domain_rosette = Thermometer::new(data.name, data.temperature, data.ip_address);
        match domain_rosette.update_temperature_async().await {
            Ok(_) => Ok(domain_rosette.get_temperature()),
            Err(_) => Err(DomainError::ThermometerError),
        }
    }
}
