use crate::error::CustomError;
use crate::mongo::house::HouseData;
use crate::mongo::rosette::RosetteData;
use crate::mongo::thermometer::ThermometerData;
use crate::MongoClient;
use mongodb::bson::{doc, ser};
use serde::{Deserialize, Serialize};

pub struct MongoApartment {
    client: MongoClient,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApartmentData {
    pub(crate) name: String,
    pub rosettes: Vec<RosetteData>,
    pub thermometers: Vec<ThermometerData>,
}

impl MongoApartment {
    pub async fn new(mongo_client: MongoClient) -> Self {
        Self {
            client: mongo_client,
        }
    }

    pub async fn get_apartments(&self, house_id: &str) -> Result<Vec<ApartmentData>, CustomError> {
        let collection = self.client.get_collection_house();
        let query = self.client.create_query_find_by_id(house_id)?;
        let house: Option<HouseData> = collection.find_one(query, None).await?;
        let house = house.unwrap();
        Ok(house.apartments)
    }

    pub async fn get_apartment(
        &self,
        house_id: &str,
        name: &str,
    ) -> Result<ApartmentData, CustomError> {
        let collection = self.client.get_collection_house();
        let query = self.client.create_query_find_by_id(house_id)?;
        let house: Option<HouseData> = collection.find_one(query, None).await?;
        let house = house.unwrap();
        let res = house.apartments.into_iter().find(|el| el.name == name);
        match res {
            None => Err(CustomError::NotFound(format!(
                "board with id: {}",
                house_id
            ))),
            Some(apartment) => Ok(apartment),
        }
    }

    pub async fn create_apartment(
        &self,
        house_id: &str,
        data: &ApartmentData,
    ) -> Result<ApartmentData, CustomError> {
        let collection = self.client.get_collection_house();
        let query = self.client.create_query_find_by_id(house_id)?;
        let update = doc! { "$push": {"apartments": ser::to_bson(data)? } };
        collection.update_one(query, update, None).await?;
        self.get_apartment(house_id, &data.name).await
    }

    pub async fn delete_apartment(
        &self,
        house_id: &str,
        apartment_name: &str,
    ) -> Result<(), CustomError> {
        let collection = self.client.get_collection_house();
        let query = self.client.create_query_find_by_id(house_id)?;
        let house = collection.find_one(query, None).await?;
        match house {
            None => Ok(()),
            Some(_house) => {
                let query = self.client.create_query_find_by_id(house_id)?;
                let update = doc! { "$pull": {"apartments": {"name": apartment_name} } };
                collection.update_one(query, update, None).await?;
                Ok(())
            }
        }
    }
}
