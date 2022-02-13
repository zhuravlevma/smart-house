use crate::error::CustomError;
use crate::mongo::house::HouseData;
use crate::mongo::rosette::RosetteData;
use crate::mongo::thermometer::ThermometerData;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, ser};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct MongoApartment(Client);

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct ApartmentData {
    pub(crate) name: String,
    pub rosettes: Vec<RosetteData>,
    pub thermometers: Vec<ThermometerData>,
}

impl MongoApartment {
    pub async fn new(connection_str: &str) -> Self {
        Self(Client::with_uri_str(connection_str).await.unwrap())
    }

    pub async fn get_apartments(
        &self,
        house_id: ObjectId,
    ) -> Result<Vec<ApartmentData>, Box<dyn Error>> {
        let collection = self.0.database("smart_home").collection("house");
        let query = doc! {"_id": &house_id };
        let house: Option<HouseData> = collection.find_one(query, None).await?;
        let house = house.unwrap();
        Ok(house.apartments)
    }

    pub async fn get_apartment(
        &self,
        house_id: ObjectId,
        name: &str,
    ) -> Result<ApartmentData, CustomError> {
        let collection = self.0.database("smart_home").collection("house");
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
        house_id: ObjectId,
        data: &ApartmentData,
    ) -> Result<ApartmentData, CustomError> {
        let collection: Collection<HouseData> = self.0.database("smart_home").collection("house");
        let query = doc! { "_id": &house_id };
        let update = doc! { "$push": {"apartments": ser::to_bson(data)? } };
        collection.update_one(query, update, None).await?;
        self.get_apartment(house_id, &data.name).await
    }
}
