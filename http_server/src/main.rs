pub mod error;
mod mongo;

use crate::mongo::apartment::{ApartmentData, MongoApartment};
use crate::mongo::house::{HouseData, MongoHouse};
use crate::mongo::rosette::MongoRosette;
use crate::mongo::thermometer::MongoThermometer;
use actix_web::web::{Data, Path};
use actix_web::{web, App, HttpResponse, HttpServer};
use log::LevelFilter;
use mongodb::bson::oid::ObjectId;
use std::env;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let connection = &env::var("MONGO_CONNECTION")?;
    let mongo_houses = Arc::new(MongoHouse::new(connection).await);
    let mongo_apartments = Arc::new(MongoApartment::new(connection).await);
    let mongo_rosette = Arc::new(MongoRosette::new(connection).await);
    let mongo_thermometer = Arc::new(MongoThermometer::new(connection).await);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(mongo_houses.clone()))
            .app_data(Data::new(mongo_apartments.clone()))
            .app_data(Data::new(mongo_rosette.clone()))
            .app_data(Data::new(mongo_thermometer.clone()))
            .service(get_temperature)
            .service(get_houses)
            .service(get_apartments)
            .service(get_thermometers)
            .service(get_rosettes)
            .service(create_house)
            .service(create_apartment)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}

#[actix_web::get("/temp")]
async fn get_temperature() -> Result<HttpResponse, Box<dyn Error>> {
    Ok(HttpResponse::Ok().json("Test result"))
}

#[actix_web::get("/house")]
async fn get_houses(houses: web::Data<Arc<MongoHouse>>) -> Result<HttpResponse, Box<dyn Error>> {
    let houses = houses.get_houses().await?;
    Ok(HttpResponse::Ok().json(houses))
}

#[actix_web::post("/house")]
async fn create_house(
    house_data: web::Json<HouseData>,
    houses: web::Data<Arc<MongoHouse>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let house_data = house_data.into_inner();
    let created = houses.create_house(house_data).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::get("/house/{id}/apartment")]
async fn get_apartments(
    path: Path<String>,
    apartments: web::Data<Arc<MongoApartment>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let id = ObjectId::from_str(&path.into_inner())?;
    let apartments = apartments.get_apartments(id).await?;
    Ok(HttpResponse::Ok().json(apartments))
}

#[actix_web::post("/house/{id}/apartment")]
async fn create_apartment(
    path: Path<String>,
    apartment_data: web::Json<ApartmentData>,
    apartments: web::Data<Arc<MongoApartment>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let id = ObjectId::from_str(&path.into_inner())?;
    let data = apartment_data.into_inner();
    let apartments = apartments.create_apartment(id, &data).await?;
    Ok(HttpResponse::Ok().json(apartments))
}

#[actix_web::get("/house/{id}/apartment/{name}/rosette")]
async fn get_rosettes(
    path: Path<(String, String)>,
    rosette: web::Data<Arc<MongoRosette>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let (id, name) = path.into_inner();
    let id = ObjectId::from_str(&id)?;
    let rosettes = rosette.get_rosettes(id, &name).await?;
    Ok(HttpResponse::Ok().json(rosettes))
}

#[actix_web::get("/apartment/${id}/thermometers")]
async fn get_thermometers(
    path: Path<String>,
    thermometer: web::Data<Arc<MongoThermometer>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let id = ObjectId::from_str(&path.into_inner())?;
    let thermometers = thermometer.get_thermometers(id).await?;
    Ok(HttpResponse::Ok().json(thermometers))
}
