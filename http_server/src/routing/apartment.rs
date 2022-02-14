use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use actix_web::{HttpResponse, web};
use actix_web::web::Path;
use mongodb::bson::oid::ObjectId;
use crate::{ApartmentData, MongoApartment};

#[actix_web::get("/{home_id}/apartment")]
pub async fn get_apartments(
    path: Path<String>,
    apartments: web::Data<Arc<MongoApartment>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let id = ObjectId::from_str(&path.into_inner())?;
    let apartments = apartments.get_apartments(id).await?;
    Ok(HttpResponse::Ok().json(apartments))
}

#[actix_web::post("/{home_id}/apartment")]
pub async fn create_apartment(
    path: Path<String>,
    apartment_data: web::Json<ApartmentData>,
    apartments: web::Data<Arc<MongoApartment>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let id = ObjectId::from_str(&path.into_inner())?;
    let data = apartment_data.into_inner();
    let apartments = apartments.create_apartment(id, &data).await?;
    Ok(HttpResponse::Ok().json(apartments))
}