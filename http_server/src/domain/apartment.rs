use crate::{ApartmentData, MongoApartment};
use smart_house::Apartment;
use std::error::Error;

pub struct ApartmentService {
    db_service: MongoApartment,
}

impl ApartmentService {
    pub async fn new(connection_str: &str) -> Self {
        Self {
            db_service: MongoApartment::new(connection_str).await,
        }
    }

    pub async fn get_list(&self, house_id: &str) -> Result<Vec<Apartment>, Box<dyn Error>> {
        let data = self.db_service.get_apartments(house_id).await?;
        let mut houses = vec![];
        for apartment in data {
            houses.push(Apartment::new(apartment.name));
        }
        Ok(houses)
    }

    pub async fn create(
        &self,
        house_id: &str,
        data: ApartmentData,
    ) -> Result<Apartment, Box<dyn Error>> {
        let data = self.db_service.create_apartment(house_id, &data).await?;
        Ok(Apartment::new(data.name))
    }
}
