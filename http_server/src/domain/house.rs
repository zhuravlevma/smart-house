use crate::{HouseData, MongoHouse};
use smart_house::{Apartment, Device, House, Rosette, Thermometer};
use std::error::Error;

// test
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
            let mut house_domain = House::new(house.name);
            for apartment in house.apartments {
                let mut apartment_domain = Apartment::new(apartment.name);
                for thermometer in apartment.thermometers {
                    apartment_domain._add_device(Device::Thermometer(Thermometer::new(
                        thermometer.name,
                        thermometer.temperature,
                        thermometer.ip_address,
                    )))?;
                }
                for rosette in apartment.rosettes {
                    apartment_domain._add_device(Device::Rosette(Rosette::new(
                        rosette.name,
                        rosette.ip_address,
                    )))?;
                }
                house_domain.add_apartment(apartment_domain)?;
            }
            houses.push(house_domain);
        }
        Ok(houses)
    }

    pub async fn create(&self, data: HouseData) -> Result<House, Box<dyn Error>> {
        let data = self.db_service.create_house(data).await?;
        Ok(House::new(data.name))
    }

    pub async fn delete(&self, house_id: &str) -> Result<House, Box<dyn Error>> {
        let data = self.db_service.delete_house(house_id).await?;
        Ok(House::new(data.name))
    }
}
