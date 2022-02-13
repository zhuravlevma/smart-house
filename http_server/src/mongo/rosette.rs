use crate::mongo::house::HouseData;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct MongoRosette(Client);

#[derive(Clone, Serialize, Deserialize)]
pub struct RosetteData {
    name: String,
}

impl MongoRosette {
    pub async fn new(connection_str: &str) -> Self {
        Self(Client::with_uri_str(connection_str).await.unwrap())
    }

    pub async fn get_rosettes(
        &self,
        house_id: ObjectId,
        apartment_name: &str,
    ) -> Result<Vec<RosetteData>, Box<dyn Error>> {
        let collection = self.0.database("smart_home").collection("house");
        let query = doc! { "_id": house_id };
        let house: Option<HouseData> = collection.find_one(query, None).await?;
        let house = house.unwrap();
        let mut vec = Vec::new();
        house.apartments.into_iter().for_each(|el| {
            if el.name == apartment_name {
                for rosette in el.rosettes {
                    vec.push(rosette)
                }
            }
        });
        Ok(vec)
    }

    // pub async fn create_rosette(&self, apartment_id: ObjectId) -> Result<RosetteData>
}
