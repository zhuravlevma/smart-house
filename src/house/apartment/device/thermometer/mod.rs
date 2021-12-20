use super::_TypeDevice;

pub struct _Thermometer {
    name: String,
    t_device: _TypeDevice,
    description: String,
}

impl _Thermometer {
    fn _get_temperature(&self) -> f32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn _get_current_temperature() {}
}
