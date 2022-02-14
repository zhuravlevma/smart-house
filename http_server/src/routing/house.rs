use std::error::Error;
use std::sync::Arc;
use actix_web::{HttpResponse, web};
use crate::{HouseData, MongoHouse};

#[actix_web::get("/")]
pub async fn get_houses(houses: web::Data<Arc<MongoHouse>>) -> Result<HttpResponse, Box<dyn Error>> {
    let houses = houses.get_houses().await?;
    Ok(HttpResponse::Ok().json(houses))
}

#[actix_web::post("/")]
pub async fn create_house(
    house_data: web::Json<HouseData>,
    houses: web::Data<Arc<MongoHouse>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let house_data = house_data.into_inner();
    let created = houses.create_house(house_data).await?;
    Ok(HttpResponse::Ok().json(created))
}
