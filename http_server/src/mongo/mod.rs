use crate::error::CustomError;
use crate::HouseData;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection};
use std::error::Error;
use std::str::FromStr;
use mongodb::bson::{doc, Document};

pub mod apartment;
pub mod house;
pub mod rosette;
pub mod thermometer;

#[derive(Clone)]
pub struct MongoClient {
    client: Client,
}

impl MongoClient {
    pub async fn new(connection_str: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            client: Client::with_uri_str(connection_str).await.unwrap(),
        })
    }

    pub fn to_mongoid(&self, str_id: &str) -> Result<ObjectId, CustomError> {
        Ok(ObjectId::from_str(str_id)?)
    }

    pub fn get_collection_house(&self) -> Collection<HouseData> {
        self.client.database("smart_home").collection("house")
    }

    pub fn create_query_find_by_id(&self, id: &str) -> Result<Document, CustomError>  {
        let object_id = self.to_mongoid(id)?;
        Ok(doc! {"_id": object_id })
    }
}
