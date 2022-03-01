use crate::error::DomainError;
use crate::{HouseData, MongoClient, MongoHouse};
use smart_house_lib::{Apartment, Device, House, Rosette, Thermometer};

pub struct HouseService {
    db_service: MongoHouse,
}

impl HouseService {
    pub async fn new(mongo_client: MongoClient) -> Self {
        Self {
            db_service: MongoHouse::new(mongo_client).await,
        }
    }

    pub async fn get_list(&self) -> Result<Vec<House>, DomainError> {
        let data = self.db_service.get_houses().await?;
        let mut houses = vec![];
        for house in data {
            let mut house_domain = House::new(house.id.unwrap().to_string(), house.name);
            for apartment in house.apartments {
                let mut apartment_domain = Apartment::new(apartment.name);
                for thermometer in apartment.thermometers {
                    match apartment_domain.add_device(Device::Thermometer(Thermometer::new(
                        thermometer.name,
                        thermometer.temperature,
                        thermometer.ip_address,
                    ))) {
                        Ok(_) => {}
                        Err(_) => return Err(DomainError::HouseError),
                    };
                }
                for rosette in apartment.rosettes {
                    match apartment_domain.add_device(Device::Rosette(Rosette::new(
                        rosette.name,
                        rosette.ip_address,
                    ))) {
                        Ok(_) => {}
                        Err(_) => return Err(DomainError::HouseError),
                    };
                }
                match house_domain.add_apartment(apartment_domain) {
                    Ok(_) => {}
                    Err(_) => return Err(DomainError::HouseError),
                };
            }
            houses.push(house_domain);
        }
        Ok(houses)
    }

    pub async fn create(&self, data: HouseData) -> Result<House, DomainError> {
        let data = self.db_service.create_house(data).await?;
        Ok(House::new(data.id.unwrap().to_string(), data.name))
    }

    pub async fn delete(&self, house_id: &str) -> Result<House, DomainError> {
        let data = self.db_service.delete_house(house_id).await?;
        Ok(House::new(data.id.unwrap().to_string(), data.name))
    }
}
