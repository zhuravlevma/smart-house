use super::TypeDevice;

pub struct Thermometer {
    pub name: String,
    pub t_device: TypeDevice,
    pub description: String,
    pub temperature: f32,
}

impl Thermometer {
    fn _get_temperature(&self) -> f32 {
        self.temperature
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn _get_current_temperature() {}
}
