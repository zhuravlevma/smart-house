use crate::{HouseData, MongoHouse};
use smart_house::House;
use std::error::Error;

pub struct HouseService {
    db_service: MongoHouse,
}

impl HouseService {
    pub async fn new(connection_str: &str) -> Self {
        Self {
            db_service: MongoHouse::new(connection_str).await,
        }
    }

    pub async fn get_list(&self) -> Result<Vec<House>, Box<dyn Error>> {
        let data = self.db_service.get_houses().await?;
        let mut houses = vec![];
        for house in data {
            houses.push(House::new(house.name));
        }
        Ok(houses)
    }

    pub async fn create(&self, data: HouseData) -> Result<House, Box<dyn Error>> {
        let data = self.db_service.create_house(data).await?;
        Ok(House::new(data.name))
    }
}
