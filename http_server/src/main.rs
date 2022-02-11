use actix_web::{App, HttpResponse, HttpServer};
use log::LevelFilter;
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // dotenv::dotenv()?;
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    HttpServer::new(move || App::new().service(get_temperature))
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
