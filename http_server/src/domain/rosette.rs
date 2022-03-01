use crate::error::DomainError;
use crate::mongo::MongoClient;
use crate::MongoRosette;
use smart_house_lib::Rosette;

pub struct RosetteService {
    db_service: MongoRosette,
}

impl RosetteService {
    pub async fn new(mongo_client: MongoClient) -> Self {
        Self {
            db_service: MongoRosette::new(mongo_client).await,
        }
    }

    pub async fn on(
        &self,
        house_id: &str,
        apartment_name: &str,
        rosette_name: &str,
    ) -> Result<bool, DomainError> {
        let data = self
            .db_service
            .get_rosette(house_id, apartment_name, rosette_name)
            .await?;
        let mut domain_rosette = Rosette::new(data.name, data.ip_address);
        match domain_rosette.on() {
            Ok(res) => Ok(res),
            Err(_) => Err(DomainError::RosetteError),
        }
    }

    pub async fn off(
        &self,
        house_id: &str,
        apartment_name: &str,
        rosette_name: &str,
    ) -> Result<bool, DomainError> {
        let data = self
            .db_service
            .get_rosette(house_id, apartment_name, rosette_name)
            .await?;
        let mut domain_rosette = Rosette::new(data.name, data.ip_address);
        match domain_rosette.off() {
            Ok(res) => Ok(res),
            Err(_) => Err(DomainError::RosetteError),
        }
    }

    pub async fn get_power(
        &self,
        house_id: &str,
        apartment_name: &str,
        rosette_name: &str,
    ) -> Result<u32, DomainError> {
        let data = self
            .db_service
            .get_rosette(house_id, apartment_name, rosette_name)
            .await?;
        let mut domain_rosette = Rosette::new(data.name, data.ip_address);
        match domain_rosette.current_power() {
            Ok(power) => Ok(power),
            Err(_) => Err(DomainError::RosetteError),
        }
    }
}
