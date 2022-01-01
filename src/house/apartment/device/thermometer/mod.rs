use super::TypeDevice;

pub struct Thermometer {
    pub name: String,
    pub t_device: TypeDevice,
    pub description: String,
}

impl Thermometer {
    fn _get_temperature(&self) -> f32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn _get_current_temperature() {}
}
