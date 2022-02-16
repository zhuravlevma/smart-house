use crate::error::CustomError;
use crate::mongo::apartment::ApartmentData;
use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;

pub struct MongoHouse(Client);

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct HouseData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    pub name: String,
    pub apartments: Vec<ApartmentData>,
}

impl MongoHouse {
    pub async fn new(connection_str: &str) -> Self {
        Self(Client::with_uri_str(connection_str).await.unwrap())
    }

    pub async fn get_houses(&self) -> Result<Vec<HouseData>, Box<dyn Error>> {
        let collection = self.0.database("smart_home").collection("house");
        let query = doc! {};
        let mut houses = collection.find(query, None).await?;
        let mut houses_vec = Vec::new();
        while let Some(house) = houses.next().await {
            houses_vec.push(house?);
        }
        Ok(houses_vec)
    }

    pub async fn create_house(&self, data: HouseData) -> Result<HouseData, Box<dyn Error>> {
        let collection = self.0.database("smart_home").collection("house");
        let inserted = collection.insert_one(data, None).await?;
        let id = inserted.inserted_id;
        let query = doc! { "_id": &id };
        let house: Option<HouseData> = collection.find_one(query, None).await?;
        Ok(house.unwrap())
    }

    pub async fn delete_house(&self, house_id: &str) -> Result<HouseData, CustomError> {
        let house_id = ObjectId::from_str(house_id)?;
        let collection = self.0.database("smart_home").collection("house");
        let query = doc! { "_id": &house_id };
        let board = collection.find_one_and_delete(query, None).await?;
        board.ok_or_else(|| CustomError::NotFound(format!("house with id: {}", house_id)))
    }
}
