use smart_house::{Apartment, Device, House};

pub async fn get_houses() -> Vec<House> {
    let url = String::from("http://127.0.0.1:8080/");
    let res = reqwest::get(&url).await.unwrap();
    let tmp: Vec<House> = res.json().await.unwrap();
    tmp
}

pub async fn get_apartments(id: String) -> (String, Vec<Apartment>) {
    let house_id = id.clone();
    let url = format!("http://127.0.0.1:8080/{}/apartment", { id });
    let res = reqwest::get(&url).await.unwrap();
    let tmp: Vec<Apartment> = res.json().await.unwrap();
    (house_id, tmp)
}

pub async fn get_devices(id: String, apartment_name: String) -> (String, String, Vec<Device>) {
    let house_id = id.clone();
    let apartment = apartment_name.clone();
    let url = format!(
        "http://127.0.0.1:8080/{}/apartment/device?apartment_name={}",
        id, apartment_name
    );
    let res = reqwest::get(&url).await.unwrap();
    let tmp: Vec<Device> = res.json().await.unwrap();
    (house_id, apartment, tmp)
}