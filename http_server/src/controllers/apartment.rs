use crate::{ApartmentData, ApartmentService};
use actix_web::web::Path;
use actix_web::{web, HttpResponse};
use std::error::Error;
use std::sync::Arc;

#[actix_web::get("/{home_id}/apartment")]
pub async fn get_apartments(
    path: Path<String>,
    apartments: web::Data<Arc<ApartmentService>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let id = &path.into_inner();
    let apartments = apartments.get_list(id).await?;
    Ok(HttpResponse::Ok().json(apartments))
}

#[actix_web::post("/{home_id}/apartment")]
pub async fn create_apartment(
    path: Path<String>,
    apartment_data: web::Json<ApartmentData>,
    apartments: web::Data<Arc<ApartmentService>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let id = &path.into_inner();
    let data = apartment_data.into_inner();
    let apartments = apartments.create(id, data).await?;
    Ok(HttpResponse::Ok().json(apartments))
}
