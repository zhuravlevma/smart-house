pub struct RosetteService {
    power: u32,
}

impl RosetteService {
    pub fn new() -> Self {
        Self { power: 0 }
    }

    pub fn off(&mut self) -> u32 {
        self.power = 0;
        self.power
    }

    pub fn on(&mut self) -> u32 {
        self.power = 220;
        self.power
    }

    pub fn get_power(&self) -> u32 {
        self.power
    }
}
