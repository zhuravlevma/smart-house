pub mod apartment;

use crate::result::{AddDataError, RemoveDataError};
use apartment::Apartment;

pub struct House {
    name: String,
    apartments: Vec<Apartment>,
}

impl House {
    pub fn new(name: String) -> Self {
        Self {
            name,
            apartments: vec![],
        }
    }
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

    pub fn create_report(&self) -> String {
        format!("{}, {}", "Test report".to_string(), self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::house::apartment::Apartment;
    use crate::house::House;
    use crate::result::{AddDataError, RemoveDataError};

    #[test]
    fn add_apartment_successful() -> Result<(), AddDataError> {
        let mut house = House::new("House1".to_string());
        let apartment = Apartment::new("Apartment1".to_string());

        house.add_apartment(apartment)?;
        Ok(())
    }

    #[test]
    fn add_apartment_error() -> Result<(), AddDataError> {
        let mut house = House::new("House1".to_string());
        let apartment1 = Apartment::new("Apartment1".to_string());
        let apartment2 = Apartment::new("Apartment1".to_string());
        house.add_apartment(apartment1)?;
        match house.add_apartment(apartment2) {
            Ok(_) => return Err(AddDataError::UniqueConstraint),
            Err(_) => Ok(()),
        }
    }

    #[test]
    fn remove_apartment_successful() -> Result<(), RemoveDataError> {
        let mut house = House::new("House1".to_string());
        let apartment1_name = "Apartment1".to_string();
        let apartment1 = Apartment::new(apartment1_name.clone());
        let apartment2 = Apartment::new("Apartment2".to_string());

        house.add_apartment(apartment1).unwrap();
        house.add_apartment(apartment2).unwrap();
        house.remove_apartment(apartment1_name)?;
        Ok(())
    }

    #[test]

    fn remove_apartment_error() -> Result<(), RemoveDataError> {
        let mut house = House::new("House1".to_string());
        let search_name = "Apartment3".to_string();
        let apartment1 = Apartment::new("Apartment1".to_string());
        let apartment2 = Apartment::new("Apartment2".to_string());

        house.add_apartment(apartment1).unwrap();
        house.add_apartment(apartment2).unwrap();

        match house.remove_apartment(search_name) {
            Ok(_) => Err(RemoveDataError::NotFound),
            Err(_) => Ok(()),
        }
    }
}
