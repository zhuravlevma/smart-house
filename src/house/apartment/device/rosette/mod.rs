use super::TypeDevice;

pub struct Rosette {
    pub name: String,
    pub t_device: TypeDevice,
    pub description: String,
}

impl Rosette {
    fn _on(&self) -> bool {
        todo!()
    }

    fn _off(&self) -> bool {
        todo!()
    }

    fn _current_power(&self) -> i32 {
        todo!()
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
