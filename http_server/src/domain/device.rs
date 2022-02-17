use crate::mongo::rosette::RosetteData;
use crate::mongo::thermometer::ThermometerData;
use crate::{MongoClient, MongoRosette, MongoThermometer};
use smart_house::{Device, Rosette, Thermometer};
use std::error::Error;

pub struct DeviceService {
    db_thermometer: MongoThermometer,
    db_rosette: MongoRosette,
}

impl DeviceService {
    pub async fn new(
        mongo_client_thermometer: MongoClient,
        mongo_client_rosette: MongoClient,
    ) -> Self {
        Self {
            db_thermometer: MongoThermometer::new(mongo_client_thermometer).await,
            db_rosette: MongoRosette::new(mongo_client_rosette).await,
        }
    }

    pub async fn get_list(
        &self,
        house_id: &str,
        apartment_name: &str,
    ) -> Result<Vec<Device>, Box<dyn Error>> {
        let rosettes = self.get_rosettes(house_id, apartment_name).await?;
        let thermometers = self.get_thermometers(house_id, apartment_name).await?;
        let mut devices = vec![];
        for rosette in rosettes {
            devices.push(rosette);
        }
        for thermometer in thermometers {
            devices.push(thermometer);
        }
        Ok(devices)
    }

    pub async fn get_rosettes(
        &self,
        house_id: &str,
        apartment_name: &str,
    ) -> Result<Vec<Device>, Box<dyn Error>> {
        let rosettes = self
            .db_rosette
            .get_rosettes(house_id, apartment_name)
            .await?;
        let mut devices = vec![];
        for rosette in rosettes {
            devices.push(Device::Rosette(Rosette::new(
                rosette.name,
                rosette.ip_address,
            )));
        }
        Ok(devices)
    }

    pub async fn get_thermometers(
        &self,
        house_id: &str,
        apartment_name: &str,
    ) -> Result<Vec<Device>, Box<dyn Error>> {
        let thermometers = self
            .db_thermometer
            .get_thermometers(house_id, apartment_name)
            .await?;
        let mut devices = vec![];
        for thermometer in thermometers {
            devices.push(Device::Thermometer(Thermometer::new(
                thermometer.name,
                thermometer.temperature,
                thermometer.ip_address,
            )));
        }
        Ok(devices)
    }

    pub async fn create_thermometer(
        &self,
        house_id: &str,
        apartment_name: &str,
        data: ThermometerData,
    ) -> Result<Device, Box<dyn Error>> {
        let data = self
            .db_thermometer
            .create_thermometer(house_id, apartment_name, &data)
            .await?;
        Ok(Device::Thermometer(Thermometer::new(
            data.name,
            data.temperature,
            data.ip_address,
        )))
    }

    pub async fn create_rosette(
        &self,
        house_id: &str,
        apartment_name: &str,
        data: RosetteData,
    ) -> Result<Device, Box<dyn Error>> {
        let data = self
            .db_rosette
            .create_rosette(house_id, apartment_name, &data)
            .await?;
        Ok(Device::Rosette(Rosette::new(data.name, data.ip_address)))
    }

    pub async fn delete_rosette(
        &self,
        house_id: &str,
        apartment_name: &str,
        rosette_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let _data = self
            .db_rosette
            .delete_rosette(house_id, apartment_name, rosette_name)
            .await?;
        Ok(())
    }

    pub async fn delete_thermometer(
        &self,
        house_id: &str,
        apartment_name: &str,
        thermometer_name: &str,
    ) -> Result<(), Box<dyn Error>> {
        let _data = self
            .db_thermometer
            .delete_thermometer(house_id, apartment_name, thermometer_name)
            .await?;
        Ok(())
    }
}
