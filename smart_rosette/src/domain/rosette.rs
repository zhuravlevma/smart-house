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
    pub(crate) fn get_power(&self) -> u32 {
        let rosette = Rosette::new(220); // заглушка, считываем с датчика температуру
        rosette.power
    }
    pub(crate) fn off(&self) -> String {
        let rosette = Rosette::new(0); // заглушка, выключаем разетку
        format!("{}", rosette.power)
    }

    pub(crate) fn on(&self) -> String {
        let rosette = Rosette::new(220); // заглушка, включаем разетку
        format!("{}", rosette.power)
    }
}

impl Default for RosetteService {
    fn default() -> Self {
        Self::new()
    }
}