use crate::mongo::house::HouseData;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct MongoThermometer(Client);

#[derive(Clone, Serialize, Deserialize)]
pub struct ThermometerData {
    name: String,
}

impl MongoThermometer {
    pub async fn new(connection_str: &str) -> Self {
        Self(Client::with_uri_str(connection_str).await.unwrap())
    }

    pub async fn get_thermometers(
        &self,
        apartment_id: ObjectId,
    ) -> Result<Vec<ThermometerData>, Box<dyn Error>> {
        let collection = self.0.database("smart_home").collection("house");
        let query = doc! { "apartments": [{ "_id": apartment_id}] };
        let house: Option<HouseData> = collection.find_one(query, None).await?;
        let house = house.unwrap();
        let mut thermometers = Vec::new();
        for apartment in house.apartments {
            for thermometer in apartment.thermometers {
                thermometers.push(thermometer);
            }
        }
        Ok(thermometers)
    }
}
