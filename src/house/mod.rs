pub mod apartment;

use crate::result::{AddDataError, RemoveDataError};
use apartment::Apartment;

pub struct House {
    pub name: String,
    pub apartments: Vec<Apartment>,
}
impl House {
    fn _get_apartments(&self) -> Option<&Vec<&Apartment>> {
        todo!()
    }
    fn _add_apartment(&self, _apartment: Apartment) -> Result<&Apartment, AddDataError> {
        todo!()
    }
    fn _remove_apartment(&self, _apartment_name: String) -> Result<String, RemoveDataError> {
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
