use crate::mongo::house::HouseData;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct MongoApartment(Client);

#[derive(Clone, Serialize, Deserialize)]
pub struct ApartmentData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
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
}
