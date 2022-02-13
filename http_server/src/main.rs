mod mongo;

use crate::mongo::house::MongoHouse;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};
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

    let mongo = MongoHouse::new(&env::var("MONGO_CONNECTION")?).await;

    let houses_data = Arc::new(mongo);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(houses_data.clone()))
            .service(get_temperature)
            .service(get_houses)
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
    let boards = houses.get_houses().await?;
    Ok(HttpResponse::Ok().json(boards))
}
