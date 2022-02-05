use smart_house::Thermometer;

fn main() {
    let mut thermometer_parents = Thermometer::new(
        "Thermometer_Parents".to_string(),
        21.2,
        "127.0.0.1:8081".to_string(),
    );
    thermometer_parents.update_temperature();
    println!("{}", thermometer_parents.get_temperature());
}
