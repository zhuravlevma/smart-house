use crate::error::CustomError;
use crate::mongo::house::HouseData;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, ser};
use mongodb::{Client, Collection};
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

    pub async fn get_rosette(
        &self,
        house_id: ObjectId,
        apartment_name: &str,
        rosette_name: &str,
    ) -> Result<RosetteData, CustomError> {
        let collection = self.0.database("smart_home").collection("house");
        let query = doc! {"_id": &house_id };
        let house: Option<HouseData> = collection.find_one(query, None).await?;
        let house = house.unwrap();
        let apartment = house
            .apartments
            .into_iter()
            .find(|el| el.name == apartment_name);
        match apartment {
            None => Err(CustomError::NotFound(format!(
                "apartment with house_id and name: {} {}",
                house_id, apartment_name
            ))),
            Some(apartment) => {
                let rosette = apartment
                    .rosettes
                    .iter()
                    .find(|rosette| rosette_name == rosette.name);
                match rosette {
                    None => Err(CustomError::NotFound(format!(
                        "rosette with house_id, apartment_name and rosette_name: {} {} {}",
                        house_id, apartment_name, rosette_name,
                    ))),
                    Some(rosette) => Ok(rosette.clone()),
                }
            }
        }
    }

    pub async fn create_rosette(
        &self,
        house_id: ObjectId,
        apartment_name: &str,
        data: &RosetteData,
    ) -> Result<RosetteData, CustomError> {
        let collection: Collection<HouseData> = self.0.database("smart_home").collection("house");
        let query = doc! { "_id": &house_id };
        let house: Option<HouseData> = collection.find_one(query, None).await?;
        let house = house.unwrap();
        let apartment = house
            .apartments
            .into_iter()
            .enumerate()
            .find(|(_index, apartment)| apartment.name == apartment_name);
        match apartment {
            None => Err(CustomError::NotFound(format!(
                "apartment with house_id and name: {} {}",
                house_id, apartment_name
            ))),
            Some((index, _apartment)) => {
                let collection: Collection<HouseData> =
                    self.0.database("smart_home").collection("house");
                let query = doc! { "_id": &house_id };
                let update = doc! { "$push": {format!("apartments.{}.rosettes", index): ser::to_bson(data)? } };
                collection.update_one(query, update, None).await?;
                self.get_rosette(house_id, apartment_name, &data.name).await
            }
        }
    }
}
