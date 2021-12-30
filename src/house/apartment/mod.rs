mod device;
use crate::result::{_AddDataError, _GetDataError, _RemoveDataError};
use device::_Device;

pub struct _Apartment {
    pub name: String,
    pub devices: Vec<_Device>,
}

impl _Apartment {
    fn _add_device(&self, _device: _Device) -> Result<&_Device, _AddDataError> {
        todo!()
    }
    fn _remove_device(&self, _device_name: String) -> Result<String, _RemoveDataError> {
        todo!()
    }
    fn _list_devices(&self) -> Option<&Vec<&_Device>> {
        todo!()
    }
    fn _get_device_by_name(&self, _device: _Device) -> Result<&_Device, _GetDataError> {
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
