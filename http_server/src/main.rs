mod mongo;

use crate::mongo::apartment::MongoApartment;
use crate::mongo::house::MongoHouse;
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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(mongo_houses.clone()))
            .app_data(Data::new(mongo_apartments.clone()))
            .service(get_temperature)
            .service(get_houses)
            .service(get_apartments)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    println!("Hello, world!");
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

#[actix_web::get("/house/{id}/apartment")]
async fn get_apartments(
    path: Path<String>,
    apartments: web::Data<Arc<MongoApartment>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let id = ObjectId::from_str(&path.into_inner())?;
    let apartments = apartments.get_apartments(id).await?;
    Ok(HttpResponse::Ok().json(apartments))
}
