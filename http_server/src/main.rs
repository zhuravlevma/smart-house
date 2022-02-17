mod controllers;
mod domain;
pub mod error;
mod mongo;

use crate::controllers::apartment::{create_apartment, delete_apartment, get_apartments};
use crate::controllers::device::get_devices;
use crate::controllers::house::{create_house, delete_house, get_houses};
use crate::controllers::rosette::{
    create_rosette, delete_rosette, get_rosettes, rosette_off, rosette_on, rosette_power,
};
use crate::controllers::thermometer::{
    create_thermometer, delete_thermometer, get_temperature, get_thermometers,
};
use crate::domain::apartment::ApartmentService;
use crate::domain::device::DeviceService;
use crate::domain::house::HouseService;
use crate::domain::rosette::RosetteService;
use crate::domain::thermometer::ThermometerService;
use crate::mongo::apartment::{ApartmentData, MongoApartment};
use crate::mongo::house::{HouseData, MongoHouse};
use crate::mongo::rosette::MongoRosette;
use crate::mongo::thermometer::MongoThermometer;
use crate::mongo::MongoClient;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use log::LevelFilter;
use std::env;
use std::error::Error;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let connection = &env::var("MONGO_CONNECTION")?;
    let mongo_client = MongoClient::new(connection).await?;
    let house_service = Arc::new(HouseService::new(mongo_client.clone()).await);
    let apartment_service = Arc::new(ApartmentService::new(mongo_client.clone()).await);
    let device_service =
        Arc::new(DeviceService::new(mongo_client.clone(), mongo_client.clone()).await);
    let rosette_service = Arc::new(RosetteService::new(mongo_client.clone()).await);
    let thermometer_service = Arc::new(ThermometerService::new(mongo_client.clone()).await);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(house_service.clone()))
            .app_data(Data::new(apartment_service.clone()))
            .app_data(Data::new(device_service.clone()))
            .app_data(Data::new(rosette_service.clone()))
            .app_data(Data::new(thermometer_service.clone()))
            .service(get_houses)
            .service(get_apartments)
            .service(get_thermometers)
            .service(get_rosettes)
            .service(create_house)
            .service(create_apartment)
            .service(create_rosette)
            .service(create_thermometer)
            .service(get_devices)
            .service(rosette_on)
            .service(rosette_off)
            .service(rosette_power)
            .service(get_temperature)
            .service(delete_house)
            .service(delete_rosette)
            .service(delete_thermometer)
            .service(delete_apartment)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
