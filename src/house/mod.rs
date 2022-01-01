pub mod apartment;

use crate::result::{AddDataError, RemoveDataError};
use apartment::Apartment;

pub struct House {
    pub name: String,
    pub apartments: Vec<Apartment>,
}
impl House {
    pub fn get_apartments(&self) -> &Vec<Apartment> {
        &self.apartments
    }
    pub fn add_apartment(&mut self, new_apartment: Apartment) -> Result<&Apartment, AddDataError> {
        for apartment in &self.apartments {
            if apartment.name.eq(&new_apartment.name) {
                return Err(AddDataError::UniqueConstraint);
            }
        }
        self.apartments.push(new_apartment);
        let length = self.apartments.len();
        Ok(&self.apartments[length - 1])
    }
    pub fn remove_apartment(
        &mut self,
        apartment_name: String,
    ) -> Result<Apartment, RemoveDataError> {
        for (pos, apartment) in self.apartments.iter().enumerate() {
            if apartment.name.eq(&apartment_name) {
                return Ok(self.apartments.remove(pos));
            }
        }
        Err(RemoveDataError::NotFound)
    }

    fn _create_report(&self) -> String {
        "Test report".to_string()
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
