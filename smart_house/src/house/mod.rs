use crate::AddDataResult;
use crate::{AddDataError, RemoveDataError, RemoveDataResult};
use apartment::Apartment;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct House {
    id: String,
    name: String,
    apartments: Vec<Apartment>,
}

impl House {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            apartments: vec![],
        }
    }
}

/// Create House
/// ```
/// use smart_house::{House};
/// let house = House::new("test_id".to_string(), "name".to_string());
/// ```
/// Add apartment to house
/// ```
/// use smart_house::{House, Apartment};
/// let mut house = House::new("test_id".to_string(), "name".to_string());
/// house.add_apartment(Apartment::new("name".to_string()));
/// ```
/// Remove apartment from house
/// ```
/// use smart_house::{House, Apartment};
/// let mut house = House::new("test_id".to_string(), "name".to_string());
/// house.add_apartment(Apartment::new("name".to_string()));
/// house.remove_apartment("name".to_string());
/// ```
impl House {
    pub fn get_apartments(&self) -> &[Apartment] {
        info!("Getting apartments");
        &self.apartments
    }
    pub fn add_apartment(&mut self, new_apartment: Apartment) -> AddDataResult<&Apartment> {
        info!(
            "Add apartment with name {} for house {}",
            new_apartment.name, self.name
        );
        match self
            .apartments
            .iter()
            .find(|&apartment| apartment.name.eq(&new_apartment.name))
        {
            None => {
                self.apartments.push(new_apartment);
                let length = self.apartments.len();
                Ok(&self.apartments[length - 1])
            }
            Some(_) => Err(AddDataError::UniqueConstraint),
        }
    }
    pub fn remove_apartment(&mut self, apartment_name: String) -> RemoveDataResult<Apartment> {
        info!(
            "Remove apartment with name {} for house {}",
            apartment_name, self.name
        );
        let position = self
            .apartments
            .iter()
            .position(|apartment| apartment.name.eq(&apartment_name));
        match position {
            Some(position) => Ok(self.apartments.remove(position)),
            None => Err(RemoveDataError::NotFound),
        }
    }

    pub fn create_report(&self) -> String {
        info!("Creating report for house {}", self.name);
        format!("{}, {}", "Test report", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn add_apartment_successful() -> Result<(), Box<dyn Error>> {
        let mut house = House::new("id".to_string(), "House1".to_string());
        let apartment = Apartment::new("Apartment1".to_string());

        house.add_apartment(apartment)?;
        Ok(())
    }

    #[test]
    fn add_apartment_error() -> Result<(), Box<dyn Error>> {
        let mut house = House::new("id".to_string(), "House1".to_string());
        let apartment1 = Apartment::new("Apartment1".to_string());
        let apartment2 = Apartment::new("Apartment1".to_string());
        house.add_apartment(apartment1)?;
        assert_eq!(house.add_apartment(apartment2).is_err(), true);
        Ok(())
    }

    #[test]
    fn remove_apartment_successful() -> Result<(), Box<dyn Error>> {
        let mut house = House::new("id".to_string(), "House1".to_string());
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
        let mut house = House::new("id".to_string(), "House1".to_string());
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
