pub mod error;
mod mongo;
mod routing;

use crate::mongo::apartment::{ApartmentData, MongoApartment};
use crate::mongo::house::{HouseData, MongoHouse};
use crate::mongo::rosette::MongoRosette;
use crate::mongo::thermometer::MongoThermometer;
use crate::routing::apartment::{create_apartment, get_apartments};
use crate::routing::house::{create_house, get_houses};
use crate::routing::rosette::{create_rosette, get_rosettes};
use crate::routing::thermometer::get_thermometers;
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
            .service(get_houses)
            .service(get_apartments)
            .service(get_thermometers)
            .service(get_rosettes)
            .service(create_house)
            .service(create_apartment)
            .service(create_rosette)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}
