use smart_house::Rosette;

fn main() {
    let mut rosette_parents =
        Rosette::new("Rosette_Parents1".to_string(), "127.0.0.1:8080".to_string());
    rosette_parents.off();
    assert_eq!(rosette_parents.current_power(), 0);
}
