use crate::error::DomainError;
use crate::{ApartmentData, MongoApartment, MongoClient};
use smart_house_lib::{Apartment, Device, Rosette, Thermometer};

pub struct ApartmentService {
    db_service: MongoApartment,
}

impl ApartmentService {
    pub async fn new(mongo_client: MongoClient) -> Self {
        Self {
            db_service: MongoApartment::new(mongo_client).await,
        }
    }

    pub async fn get_list(&self, house_id: &str) -> Result<Vec<Apartment>, DomainError> {
        let data = self.db_service.get_apartments(house_id).await?;
        let mut houses = vec![];
        for apartment in data {
            let mut apartment_domain = Apartment::new(apartment.name);
            for thermometer in apartment.thermometers {
                match apartment_domain.add_device(Device::Thermometer(Thermometer::new(
                    thermometer.name,
                    thermometer.temperature,
                    thermometer.ip_address,
                ))) {
                    Ok(_) => {}
                    Err(_) => return Err(DomainError::ApartmentError),
                };
            }
            for rosette in apartment.rosettes {
                match apartment_domain.add_device(Device::Rosette(Rosette::new(
                    rosette.name,
                    rosette.ip_address,
                ))) {
                    Ok(_) => {}
                    Err(_) => return Err(DomainError::ApartmentError),
                };
            }

            houses.push(apartment_domain);
        }
        Ok(houses)
    }

    pub async fn create(
        &self,
        house_id: &str,
        data: ApartmentData,
    ) -> Result<Apartment, DomainError> {
        let data = self.db_service.create_apartment(house_id, &data).await?;
        Ok(Apartment::new(data.name))
    }

    pub async fn delete(&self, house_id: &str, apartment_name: &str) -> Result<(), DomainError> {
        let _data = self
            .db_service
            .delete_apartment(house_id, apartment_name)
            .await?;
        Ok(())
    }
}
