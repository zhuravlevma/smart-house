use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use actix_web::{HttpResponse, web};
use actix_web::web::Path;
use mongodb::bson::oid::ObjectId;
use crate::{MongoRosette};
use serde::{Deserialize};
use crate::mongo::rosette::RosetteData;

#[derive(Deserialize)]
pub struct ApartmentInfo {
    apartment_name: String,
}

#[actix_web::get("/{home_id}/apartment/rosette")]
pub async fn get_rosettes(
    path: Path<String>,
    apartment_info: web::Query<ApartmentInfo>,
    rosette: web::Data<Arc<MongoRosette>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let id = path.into_inner();
    let id = ObjectId::from_str(&id)?;
    let apartment_name = &apartment_info.apartment_name;
    let rosettes = rosette.get_rosettes(id, apartment_name).await?;
    Ok(HttpResponse::Ok().json(rosettes))
}

#[actix_web::post("/{home_id}/apartment/rosette")]
pub async fn create_rosette(
    path: Path<String>,
    apartment_info: web::Query<ApartmentInfo>,
    rosette_entity: web::Json<RosetteData>,
    rosette: web::Data<Arc<MongoRosette>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let house_id = path.into_inner();
    let house_id = ObjectId::from_str(&house_id)?;
    let apartment_name = &apartment_info.apartment_name;
    let rosette_entity = rosette_entity.into_inner();
    let rosette = rosette.create_rosette(house_id, apartment_name, &rosette_entity).await?;
    Ok(HttpResponse::Ok().json(rosette))
}