use crate::error::DomainError;
use crate::{HouseData, HouseService};
use actix_web::web::Path;
use actix_web::{web, HttpResponse};
use std::sync::Arc;

#[actix_web::get("/")]
pub async fn get_houses(houses: web::Data<Arc<HouseService>>) -> Result<HttpResponse, DomainError> {
    let houses = houses.get_list().await?;
    Ok(HttpResponse::Ok().json(houses))
}

#[actix_web::post("/")]
pub async fn create_house(
    house_data: web::Json<HouseData>,
    houses: web::Data<Arc<HouseService>>,
) -> Result<HttpResponse, DomainError> {
    let house_data = house_data.into_inner();
    let created = houses.create(house_data).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::delete("/{id}")]
pub async fn delete_house(
    path: Path<String>,
    houses: web::Data<Arc<HouseService>>,
) -> Result<HttpResponse, DomainError> {
    let house_id = &path.into_inner();
    let created = houses.delete(house_id).await?;
    Ok(HttpResponse::Ok().json(created))
}
