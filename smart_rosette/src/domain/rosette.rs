pub struct Rosette {
    power: u32,
}

impl Rosette {
    pub fn new(power: u32) -> Self {
        Self { power }
    }
}

pub struct RosetteService {}

impl RosetteService {
    pub fn new() -> Self {
        Self {}
    }
    pub(crate) fn get_power(&self, data: String) -> String {
        let rosette = Rosette::new(220); // заглушка, считываем с датчика температуру
        format!("{} - {}", data, rosette.power)
    }
    pub(crate) fn off(&self) -> String {
        let rosette = Rosette::new(0); // заглушка, выключаем разетку
        format!("{}", rosette.power)
    }
}
