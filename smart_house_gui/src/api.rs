use smart_house_lib::{Apartment, Device, House};

pub async fn get_houses() -> Vec<House> {
    let url = String::from("http://127.0.0.1:8080/");
    let res = reqwest::get(&url).await.unwrap();
    let tmp: Vec<House> = res.json().await.unwrap();
    tmp
}

pub async fn delete_house(house_id: String) -> String {
    let url = format!("http://127.0.0.1:8080/{}", &house_id);
    let _res = reqwest::Client::new().delete(&url).send().await.unwrap();
    // let tmp = res.json().await.unwrap();
    house_id
}

pub async fn get_apartments(id: String) -> (String, Vec<Apartment>) {
    let house_id = id.clone();
    let url = format!("http://127.0.0.1:8080/{}/apartment", { id });
    let res = reqwest::get(&url).await.unwrap();
    let tmp: Vec<Apartment> = res.json().await.unwrap();
    (house_id, tmp)
}

pub async fn delete_apartment(house_id: String, apartment_name: String) -> (String, String) {
    let url = format!("http://127.0.0.1:8080/{}/{}", &house_id, &apartment_name);
    let _res = reqwest::Client::new().delete(&url).send().await.unwrap();
    (house_id, apartment_name)
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

pub async fn delete_rosette(
    house_id: String,
    apartment_name: String,
    rosette_name: String,
) -> (String, String, String) {
    let url = format!(
        "http://127.0.0.1:8080/{}/{}/rosette/{}",
        &house_id, &apartment_name, &rosette_name
    );
    let _res = reqwest::Client::new().delete(&url).send().await.unwrap();
    (house_id, apartment_name, rosette_name)
}

pub async fn delete_thermometer(
    house_id: String,
    apartment_name: String,
    thermometer_name: String,
) -> (String, String, String) {
    let url = format!(
        "http://127.0.0.1:8080/{}/{}/thermometer/{}",
        &house_id, &apartment_name, &thermometer_name
    );
    let _res = reqwest::Client::new().delete(&url).send().await.unwrap();
    (house_id, apartment_name, thermometer_name)
}

pub async fn rosette_off(
    id: String,
    apartment_name: String,
    rosette_name: String,
) -> (String, String, String, bool) {
    let house_id = id.clone();
    let apartment = apartment_name.clone();
    let rosette = rosette_name.clone();
    let url = format!(
        "http://127.0.0.1:8080/{}/apartment/rosette/off?apartment_name={}&rosette_name={}",
        id, apartment_name, rosette_name
    );
    let res = reqwest::Client::new().post(url).send().await.unwrap();
    let tmp = res.json().await.unwrap();
    (house_id, apartment, rosette, tmp)
}

pub async fn rosette_on(
    id: String,
    apartment_name: String,
    rosette_name: String,
) -> (String, String, String, bool) {
    let house_id = id.clone();
    let apartment = apartment_name.clone();
    let rosette = rosette_name.clone();
    let url = format!(
        "http://127.0.0.1:8080/{}/apartment/rosette/on?apartment_name={}&rosette_name={}",
        id, apartment_name, rosette_name
    );
    let res = reqwest::Client::new().post(url).send().await.unwrap();
    let tmp = res.json().await.unwrap();
    (house_id, apartment, rosette, tmp)
}

pub async fn rosette_sync(
    id: String,
    apartment_name: String,
    rosette_name: String,
) -> (String, String, String, u32) {
    let house_id = id.clone();
    let apartment = apartment_name.clone();
    let rosette = rosette_name.clone();
    let url = format!(
        "http://127.0.0.1:8080/{}/apartment/rosette/power?apartment_name={}&rosette_name={}",
        id, apartment_name, rosette_name
    );
    let res = reqwest::get(&url).await.unwrap();
    let tmp = res.json().await.unwrap();
    (house_id, apartment, rosette, tmp)
}

pub async fn thermometer_sync(
    id: String,
    apartment_name: String,
    thermometer_name: String,
) -> (String, String, String, f32) {
    let house_id = id.clone();
    let apartment = apartment_name.clone();
    let thermometer = thermometer_name.clone();
    let url = format!(
        "http://127.0.0.1:8080/{}/apartment/thermometer/temp?apartment_name={}&thermometer_name={}",
        id, apartment_name, thermometer_name
    );
    let res = reqwest::get(&url).await.unwrap();
    let tmp = res.json::<f32>().await.unwrap();
    (house_id, apartment, thermometer, tmp)
}
