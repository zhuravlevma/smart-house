use smart_house_lib::Rosette;

fn main() {
    let mut rosette_parents =
        Rosette::new("Rosette_Parents1".to_string(), "127.0.0.1:8084".to_string());
    rosette_parents.off().unwrap();
    assert_eq!(rosette_parents.current_power().unwrap(), 0);
}
