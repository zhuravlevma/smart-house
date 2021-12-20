use super::_TypeDevice;

pub struct _Rosette {
    name: String,
    t_device: _TypeDevice,
    description: String,
}

impl _Rosette {
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
