use super::TypeDevice;

pub struct Rosette {
    pub name: String,
    pub t_device: TypeDevice,
    pub description: String,
    pub power: u32,
}

impl Rosette {
    fn _on(&mut self) -> bool {
        self.power = 220;
        true
    }

    fn _off(&mut self) -> bool {
        self.power = 0;
        false
    }

    fn _current_power(&self) -> u32 {
        self.power
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn rosette_on() {}

    #[test]
    fn rosette_off() {}

    #[test]
    fn rosette_get_current_power() {}
}
