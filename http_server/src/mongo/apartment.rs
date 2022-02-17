use crate::error::CustomError;
use crate::mongo::house::HouseData;
use crate::mongo::rosette::RosetteData;
use crate::mongo::thermometer::ThermometerData;
use crate::MongoClient;
use mongodb::bson::{doc, ser};
use serde::{Deserialize, Serialize};
use std::error::Error;

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

    pub async fn get_apartments(
        &self,
        house_id: &str,
    ) -> Result<Vec<ApartmentData>, Box<dyn Error>> {
        let house_id = self.client.to_mongoid(house_id)?;
        let collection = self.client.get_collection_house();
        let query = doc! {"_id": &house_id };
        let house: Option<HouseData> = collection.find_one(query, None).await?;
        let house = house.unwrap();
        Ok(house.apartments)
    }

    pub async fn get_apartment(
        &self,
        house_id: &str,
        name: &str,
    ) -> Result<ApartmentData, CustomError> {
        let house_id = self.client.to_mongoid(house_id)?;
        let collection = self.client.get_collection_house();
        let query = doc! {"_id": &house_id };
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
        let house_id_obj = self.client.to_mongoid(house_id)?;
        let collection = self.client.get_collection_house();
        let query = doc! { "_id": &house_id_obj };
        let update = doc! { "$push": {"apartments": ser::to_bson(data)? } };
        collection.update_one(query, update, None).await?;
        self.get_apartment(house_id, &data.name).await
    }
}
