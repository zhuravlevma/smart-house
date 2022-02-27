use crate::error::DomainError;
use crate::mongo::rosette::RosetteData;
use crate::{DeviceService, RosetteService};
use actix_web::web::Path;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ApartmentInfo {
    apartment_name: String,
}

#[derive(Deserialize)]
pub struct RosetteInfo {
    apartment_name: String,
    rosette_name: String,
}

#[actix_web::get("/{home_id}/apartment/rosette")]
pub async fn get_rosettes(
    path: Path<String>,
    apartment_info: web::Query<ApartmentInfo>,
    device: web::Data<Arc<DeviceService>>,
) -> Result<HttpResponse, DomainError> {
    let id = &path.into_inner();
    let apartment_name = &apartment_info.apartment_name;
    let rosettes = device.get_rosettes(id, apartment_name).await?;
    Ok(HttpResponse::Ok().json(rosettes))
}

#[actix_web::post("/{home_id}/apartment/rosette")]
pub async fn create_rosette(
    path: Path<String>,
    apartment_info: web::Query<ApartmentInfo>,
    rosette_entity: web::Json<RosetteData>,
    device: web::Data<Arc<DeviceService>>,
) -> Result<HttpResponse, DomainError> {
    let house_id = &path.into_inner();
    let apartment_name = &apartment_info.apartment_name;
    let rosette_entity = rosette_entity.into_inner();
    let rosette = device
        .create_rosette(house_id, apartment_name, rosette_entity)
        .await?;
    Ok(HttpResponse::Ok().json(rosette))
}

#[actix_web::delete("/{home_id}/{apartment_name}/rosette/{rosette_name}")]
pub async fn delete_rosette(
    path: Path<(String, String, String)>,
    device: web::Data<Arc<DeviceService>>,
) -> Result<HttpResponse, DomainError> {
    let (house_id, apartment_name, rosette_name) = &path.into_inner();
    let created = device
        .delete_rosette(house_id, apartment_name, rosette_name)
        .await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::post("/{home_id}/apartment/rosette/on")]
pub async fn rosette_on(
    path: Path<String>,
    rosette_info: web::Query<RosetteInfo>,
    rosette: web::Data<Arc<RosetteService>>,
) -> Result<HttpResponse, DomainError> {
    let house_id = &path.into_inner();
    let apartment_name = &rosette_info.apartment_name;
    let rosette_name = &rosette_info.rosette_name;
    let rosette = rosette.on(house_id, apartment_name, rosette_name).await?;
    Ok(HttpResponse::Ok().json(rosette))
}

#[actix_web::post("/{home_id}/apartment/rosette/off")]
pub async fn rosette_off(
    path: Path<String>,
    rosette_info: web::Query<RosetteInfo>,
    rosette: web::Data<Arc<RosetteService>>,
) -> Result<HttpResponse, DomainError> {
    let house_id = &path.into_inner();
    let apartment_name = &rosette_info.apartment_name;
    let rosette_name = &rosette_info.rosette_name;
    let rosette = rosette.off(house_id, apartment_name, rosette_name).await?;
    Ok(HttpResponse::Ok().json(rosette))
}

#[actix_web::get("/{home_id}/apartment/rosette/power")]
pub async fn rosette_power(
    path: Path<String>,
    rosette_info: web::Query<RosetteInfo>,
    rosette: web::Data<Arc<RosetteService>>,
) -> Result<HttpResponse, DomainError> {
    let house_id = &path.into_inner();
    let apartment_name = &rosette_info.apartment_name;
    let rosette_name = &rosette_info.rosette_name;
    let rosette = rosette
        .get_power(house_id, apartment_name, rosette_name)
        .await?;
    Ok(HttpResponse::Ok().json(rosette))
}
