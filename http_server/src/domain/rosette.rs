use crate::mongo::MongoClient;
use crate::MongoRosette;
use smart_house::Rosette;
use std::error::Error;

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
    ) -> Result<bool, Box<dyn Error>> {
        let data = self
            .db_service
            .get_rosette(house_id, apartment_name, rosette_name)
            .await?;
        let mut domain_rosette = Rosette::new(data.name, data.ip_address);
        Ok(domain_rosette.on())
    }

    pub async fn off(
        &self,
        house_id: &str,
        apartment_name: &str,
        rosette_name: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let data = self
            .db_service
            .get_rosette(house_id, apartment_name, rosette_name)
            .await?;
        let mut domain_rosette = Rosette::new(data.name, data.ip_address);
        Ok(domain_rosette.off())
    }

    pub async fn get_power(
        &self,
        house_id: &str,
        apartment_name: &str,
        rosette_name: &str,
    ) -> Result<u32, Box<dyn Error>> {
        let data = self
            .db_service
            .get_rosette(house_id, apartment_name, rosette_name)
            .await?;
        let mut domain_rosette = Rosette::new(data.name, data.ip_address);
        Ok(domain_rosette.current_power())
    }
}
