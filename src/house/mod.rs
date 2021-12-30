mod apartment;

use crate::result::{_AddDataError, _RemoveDataError};
use apartment::_Apartment;

pub struct _House {
    pub name: String,
    pub apartments: Vec<_Apartment>,
}
impl _House {
    fn _get_apartments(&self) -> Option<&Vec<&_Apartment>> {
        todo!()
    }
    fn _add_apartment(&self, _apartment: _Apartment) -> Result<&_Apartment, _AddDataError> {
        todo!()
    }
    fn _remove_apartment(&self, _apartment_name: String) -> Result<String, _RemoveDataError> {
        todo!()
    }

    fn _create_report(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_apartment_successful() {}

    #[test]
    fn add_apartment_error() {}

    #[test]
    fn get_apartment_successful() {}

    #[test]
    fn get_apartment_error() {}

    #[test]
    fn _remove_apartment_successful() {}

    #[test]
    fn _remove_apartment_error() {}
}
