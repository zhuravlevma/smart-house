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
    fn add_apartment_successful() {
        let mut house = House::new("House1".to_string());
        let apartment = Apartment::new("Apartment1".to_string());

        match house.add_apartment(apartment) {
            Ok(_) => {}
            Err(error) => match error {
                AddDataError::UniqueConstraint => {
                    panic!("{}", AddDataError::UniqueConstraint);
                }
            },
        }
    }

    #[test]
    fn add_apartment_error() {
        let mut house = House::new("House1".to_string());
        let apartment1 = Apartment::new("Apartment1".to_string());
        let apartment2 = Apartment::new("Apartment1".to_string());

        match house.add_apartment(apartment1) {
            Ok(_) => {}
            Err(_) => {
                panic!("unknown error");
            }
        };
        match house.add_apartment(apartment2) {
            Ok(_) => {
                panic!("Add apartment should get error")
            }
            Err(_) => {}
        };
    }

    #[test]
    fn remove_apartment_successful() {
        let mut house = House::new("House1".to_string());
        let apartment1_name = "Apartment1".to_string();
        let apartment1 = Apartment::new(apartment1_name.clone());
        let apartment2 = Apartment::new("Apartment2".to_string());

        match house.add_apartment(apartment1) {
            Ok(_) => {}
            Err(_) => {
                panic!("unknown error");
            }
        };
        match house.add_apartment(apartment2) {
            Ok(_) => {}
            Err(_) => {
                panic!("unknown error");
            }
        };

        match house.remove_apartment(apartment1_name) {
            Ok(_) => {}
            Err(error) => match error {
                RemoveDataError::NotFound => {
                    panic!("RemoveError NotFound. Remove by name should get ok")
                }
            },
        };

        assert_eq!(house.get_apartments().len(), 1);
    }

    #[test]
    fn remove_apartment_error() {
        let mut house = House::new("House1".to_string());
        let search_name = "Apartment3".to_string();
        let apartment1 = Apartment::new("Apartment1".to_string());
        let apartment2 = Apartment::new("Apartment2".to_string());

        match house.add_apartment(apartment1) {
            Ok(_) => {}
            Err(_) => {
                panic!("unknown error");
            }
        };
        match house.add_apartment(apartment2) {
            Ok(_) => {}
            Err(_) => {
                panic!("unknown error");
            }
        };

        match house.remove_apartment(search_name) {
            Ok(_) => {
                panic!("Remove by name should get error")
            }
            Err(_) => {}
        };
    }
}
