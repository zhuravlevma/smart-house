use crate::{AddDataError, RemoveDataError};
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
        let result: Vec<&Apartment> = self.apartments.iter().filter(|apartment| apartment.name.eq(&new_apartment.name)).collect();
        if result.len() > 0 {
            return Err(AddDataError::UniqueConstraint)
        }
        self.apartments.push(new_apartment);
        let length = self.apartments.len();
        Ok(&self.apartments[length - 1])
    }
    pub fn remove_apartment(
        &mut self,
        apartment_name: String,
    ) -> Result<Apartment, RemoveDataError> {
        let position = &self.apartments.iter().enumerate()
            .position(|(_pos, apartment)|  apartment.name.eq(&apartment_name));
        match position {
            Some(position) => {
                Ok(self.apartments.remove(*position))
            }
            None => Err(RemoveDataError::NotFound),
        }
    }

    pub fn create_report(&self) -> String {
        format!("{}, {}", "Test report".to_string(), self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn add_apartment_successful() -> Result<(), Box<dyn Error>> {
        let mut house = House::new("House1".to_string());
        let apartment = Apartment::new("Apartment1".to_string());

        house.add_apartment(apartment)?;
        Ok(())
    }

    #[test]
    fn add_apartment_error() -> Result<(), Box<dyn Error>> {
        let mut house = House::new("House1".to_string());
        let apartment1 = Apartment::new("Apartment1".to_string());
        let apartment2 = Apartment::new("Apartment1".to_string());
        house.add_apartment(apartment1)?;
        assert_eq!(house.add_apartment(apartment2).is_err(), true);
        Ok(())
    }

    #[test]
    fn remove_apartment_successful() -> Result<(), Box<dyn Error>> {
        let mut house = House::new("House1".to_string());
        let apartment1_name = "Apartment1".to_string();
        let apartment1 = Apartment::new(apartment1_name.clone());
        let apartment2 = Apartment::new("Apartment2".to_string());

        house.add_apartment(apartment1)?;
        house.add_apartment(apartment2)?;
        house.remove_apartment(apartment1_name)?;
        Ok(())
    }

    #[test]

    fn remove_apartment_error() -> Result<(), Box<dyn Error>> {
        let mut house = House::new("House1".to_string());
        let search_name = "Apartment3".to_string();
        let apartment1 = Apartment::new("Apartment1".to_string());
        let apartment2 = Apartment::new("Apartment2".to_string());

        house.add_apartment(apartment1)?;
        house.add_apartment(apartment2)?;

        assert_eq!(house.remove_apartment(search_name).is_err(), true);
        Ok(())
    }
}

pub mod apartment;
