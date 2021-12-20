mod device;
use crate::result::{_AddResult, _GetResult, _RemoveResult};
use device::_Device;

pub struct _Apartment {
    pub name: String,
    pub devices: Vec<_Device>,
}

impl _Apartment {
    fn _add_device(&self, _device: _Device) -> &_AddResult {
        todo!()
    }
    fn _remove_device(&self, _device_name: String) -> _RemoveResult {
        todo!()
    }
    fn _list_devices(&self) {
        todo!()
    }
    fn _get_device_by_name(&self, _device: _Device) -> &_GetResult {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn _add_device_successful() {}

    #[test]
    fn _add_device_error() {}

    #[test]
    fn _get_device_by_name_successful() {}

    #[test]
    fn _get_device_by_name_error() {}

    #[test]
    fn _remove_device_successful() {}

    #[test]
    fn _remove_device_error() {}
}
