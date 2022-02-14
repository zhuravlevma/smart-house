use crate::MongoThermometer;
use actix_web::web::Path;
use actix_web::{web, HttpResponse};
use mongodb::bson::oid::ObjectId;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApartmentInfo {
    apartment_name: String,
}

#[actix_web::get("/{home_id}/apartment/thermometer")]
pub async fn get_thermometers(
    path: Path<String>,
    apartment_info: web::Query<ApartmentInfo>,
    thermometer: web::Data<Arc<MongoThermometer>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let home_id = ObjectId::from_str(&path.into_inner())?;
    let apartment_name = &apartment_info.apartment_name;
    let thermometers = thermometer
        .get_thermometers(home_id, apartment_name)
        .await?;
    Ok(HttpResponse::Ok().json(thermometers))
}
