use crate::mongo::house::HouseData;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct MongoRosette(Client);

#[derive(Clone, Serialize, Deserialize)]
pub struct RosetteData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
}

impl MongoRosette {
    pub async fn new(connection_str: &str) -> Self {
        Self(Client::with_uri_str(connection_str).await.unwrap())
    }

    pub async fn get_rosettes(
        &self,
        apartment_id: ObjectId,
    ) -> Result<Vec<RosetteData>, Box<dyn Error>> {
        let collection = self.0.database("smart_home").collection("house");
        let query = doc! { "apartments": [{ "_id": apartment_id}] };
        let house: Option<HouseData> = collection.find_one(query, None).await?;
        let house = house.unwrap();
        let mut vec = Vec::new();
        for apartment in house.apartments {
            for rosette in apartment.rosettes {
                vec.push(rosette)
            }
        }
        Ok(vec)
    }
}
