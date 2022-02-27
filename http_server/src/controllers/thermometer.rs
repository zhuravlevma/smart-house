use crate::{DeviceService, ThermometerService};
use actix_web::web::Path;
use actix_web::{web, HttpResponse};
use std::sync::Arc;

use crate::error::DomainError;
use crate::mongo::thermometer::ThermometerData;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApartmentInfo {
    apartment_name: String,
}

#[derive(Deserialize)]
pub struct ThermometerInfo {
    apartment_name: String,
    thermometer_name: String,
}

#[actix_web::get("/{home_id}/apartment/thermometer")]
pub async fn get_thermometers(
    path: Path<String>,
    apartment_info: web::Query<ApartmentInfo>,
    device: web::Data<Arc<DeviceService>>,
) -> Result<HttpResponse, DomainError> {
    let home_id = &path.into_inner();
    let apartment_name = &apartment_info.apartment_name;
    let thermometers = device.get_thermometers(home_id, apartment_name).await?;
    Ok(HttpResponse::Ok().json(thermometers))
}

#[actix_web::post("/{home_id}/apartment/thermometer")]
pub async fn create_thermometer(
    path: Path<String>,
    apartment_info: web::Query<ApartmentInfo>,
    thermometer_entity: web::Json<ThermometerData>,
    device: web::Data<Arc<DeviceService>>,
) -> Result<HttpResponse, DomainError> {
    let house_id = &path.into_inner();
    let apartment_name = &apartment_info.apartment_name;
    let thermometer_entity = thermometer_entity.into_inner();
    let thermometer = device
        .create_thermometer(house_id, apartment_name, thermometer_entity)
        .await?;
    Ok(HttpResponse::Ok().json(thermometer))
}

#[actix_web::delete("/{home_id}/{apartment_name}/thermometer/{thermometer_name}")]
pub async fn delete_thermometer(
    path: Path<(String, String, String)>,
    device: web::Data<Arc<DeviceService>>,
) -> Result<HttpResponse, DomainError> {
    let (house_id, apartment_name, thermometer_name) = &path.into_inner();
    let created = device
        .delete_thermometer(house_id, apartment_name, thermometer_name)
        .await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::get("/{home_id}/apartment/thermometer/temp")]
pub async fn get_temperature(
    path: Path<String>,
    apartment_info: web::Query<ThermometerInfo>,
    thermometer: web::Data<Arc<ThermometerService>>,
) -> Result<HttpResponse, DomainError> {
    let house_id = &path.into_inner();
    let apartment_name = &apartment_info.apartment_name;
    let thermometer_name = &apartment_info.thermometer_name;
    let status = thermometer
        .get_temperature(house_id, apartment_name, thermometer_name)
        .await?;
    Ok(HttpResponse::Ok().json(status))
}
