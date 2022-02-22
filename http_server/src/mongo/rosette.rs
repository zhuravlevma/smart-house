use crate::error::CustomError;
use crate::mongo::house::HouseData;
use crate::mongo::MongoClient;
use mongodb::bson::{doc, ser};
use serde::{Deserialize, Serialize};

pub struct MongoRosette {
    client: MongoClient,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RosetteData {
    pub(crate) name: String,
    pub ip_address: String,
}

impl MongoRosette {
    pub async fn new(mongo_client: MongoClient) -> Self {
        Self {
            client: mongo_client,
        }
    }

    pub async fn get_rosettes(
        &self,
        house_id: &str,
        apartment_name: &str,
    ) -> Result<Vec<RosetteData>, CustomError> {
        let house_id = self.client.to_mongoid(house_id)?;
        let collection = self.client.get_collection_house();
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
        house_id: &str,
        apartment_name: &str,
        rosette_name: &str,
    ) -> Result<RosetteData, CustomError> {
        let house_id = self.client.to_mongoid(house_id)?;
        let collection = self.client.get_collection_house();
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
        house_id: &str,
        apartment_name: &str,
        data: &RosetteData,
    ) -> Result<RosetteData, CustomError> {
        let house_id_obj = self.client.to_mongoid(house_id)?;
        let collection = self.client.get_collection_house();
        let query = doc! { "_id": &house_id_obj };
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
                let collection = self.client.get_collection_house();
                let query = doc! { "_id": &house_id_obj };
                let update = doc! { "$push": {format!("apartments.{}.rosettes", index): ser::to_bson(data)? } };
                collection.update_one(query, update, None).await?;
                self.get_rosette(house_id, apartment_name, &data.name).await
            }
        }
    }

    pub async fn delete_rosette(
        &self,
        house_id: &str,
        apartment_name: &str,
        rosette_name: &str,
    ) -> Result<(), CustomError> {
        let collection = self.client.get_collection_house();
        let query = self.client.create_query_find_by_id(house_id)?;
        let house = collection.find_one(query, None).await?;
        match house {
            None => Ok(()),
            Some(house) => {
                let res = house
                    .apartments
                    .into_iter()
                    .enumerate()
                    .find(|(_idx, apartment)| apartment.name == apartment_name);
                match res {
                    None => Ok(()),
                    Some((idx, _apartment_data)) => {
                        let query = self.client.create_query_find_by_id(house_id)?;
                        let update = doc! { "$pull": {format!("apartments.{}.rosettes", idx): {"name": rosette_name}} };
                        collection.update_one(query, update, None).await?;
                        Ok(())
                    }
                }
            }
        }
    }
}
