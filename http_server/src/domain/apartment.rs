use crate::{ApartmentData, MongoApartment};
use smart_house::{Apartment, Device, Rosette, Thermometer};
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

            houses.push(apartment_domain);
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
