use smart_house::{Apartment, House};

pub async fn get_houses() -> Vec<House> {
    let url = String::from("http://127.0.0.1:8080/");
    let res = reqwest::get(&url).await.unwrap();
    let tmp: Vec<House> = res.json().await.unwrap();
    tmp
}

pub async fn get_apartments(id: String) -> Vec<Apartment> {
    let url = format!("http://127.0.0.1:8080/{}/apartment", { id });
    let res = reqwest::get(&url).await.unwrap();
    let tmp: Vec<Apartment> = res.json().await.unwrap();
    tmp
}