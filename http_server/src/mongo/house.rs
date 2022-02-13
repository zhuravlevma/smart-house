use crate::mongo::apartment::ApartmentData;
use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct MongoHouse(Client);

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct HouseData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
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
}
