use crate::mongo::house::HouseData;
use mongodb::bson::{doc, ser};
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str::FromStr;
use crate::error::CustomError;

pub struct MongoThermometer(Client);

#[derive(Clone, Serialize, Deserialize)]
pub struct ThermometerData {
    pub(crate) name: String,
    pub temperature: f32,
    pub ip_address: String,
}

impl MongoThermometer {
    pub async fn new(connection_str: &str) -> Self {
        Self(Client::with_uri_str(connection_str).await.unwrap())
    }

    pub async fn get_thermometers(
        &self,
        home_id: &str,
        apartment_name: &str,
    ) -> Result<Vec<ThermometerData>, Box<dyn Error>> {
        let home_id = ObjectId::from_str(home_id)?;
        let collection = self.0.database("smart_home").collection("house");
        let query = doc! { "_id": home_id };
        let house: Option<HouseData> = collection.find_one(query, None).await?;
        let house = house.unwrap();
        let mut thermometers = Vec::new();
        for apartment in house.apartments {
            if apartment_name == apartment.name {
                for thermometer in apartment.thermometers {
                    thermometers.push(thermometer);
                }
            }
        }
        Ok(thermometers)
    }

    pub async fn get_thermometer(
        &self,
        house_id: &str,
        apartment_name: &str,
        thermometer_name: &str,
    ) -> Result<ThermometerData, CustomError> {
        let house_id = ObjectId::from_str(house_id)?;
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
                let thermometer = apartment
                    .thermometers
                    .iter()
                    .find(|thermometer| thermometer_name == thermometer.name);
                match thermometer {
                    None => Err(CustomError::NotFound(format!(
                        "rosette with house_id, apartment_name and thermometer_name: {} {} {}",
                        house_id, apartment_name, thermometer_name,
                    ))),
                    Some(thermometer) => Ok(thermometer.clone()),
                }
            }
        }
    }

    pub async fn create_thermometer(
        &self,
        house_id: &str,
        apartment_name: &str,
        data: &ThermometerData,
    ) -> Result<ThermometerData, CustomError> {
        let house_id_obj = ObjectId::from_str(house_id)?;
        let collection: Collection<HouseData> = self.0.database("smart_home").collection("house");
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
                let collection: Collection<HouseData> =
                    self.0.database("smart_home").collection("house");
                let query = doc! { "_id": house_id };
                let update = doc! { "$push": {format!("apartments.{}.thermometers", index): ser::to_bson(data)? } };
                collection.update_one(query, update, None).await?;
                self.get_thermometer(house_id, apartment_name, &data.name).await
            }
        }
    }
}
