use crate::DeviceService;
use actix_web::web::Path;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::error::Error;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ApartmentInfo {
    apartment_name: String,
}

#[actix_web::get("/{home_id}/apartment/device")]
pub async fn get_devices(
    path: Path<String>,
    apartment_info: web::Query<ApartmentInfo>,
    device: web::Data<Arc<DeviceService>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let id = &path.into_inner();
    let apartment_name = &apartment_info.apartment_name;
    let rosettes = device.get_list(id, apartment_name).await?;
    Ok(HttpResponse::Ok().json(rosettes))
}
