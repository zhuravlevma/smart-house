use rosette::Rosette;
use thermometer::Thermometer;

pub enum TypeDevice {
    Thermometer,
    Rosette,
}

pub enum Device {
    Rosette(Rosette),
    Thermometer(Thermometer),
}

impl PartialEq<Self> for Device {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Device::Rosette(rosette_main) => match other {
                Device::Rosette(rosette) => rosette.name.eq(&rosette_main.name),
                Device::Thermometer(thermometer) => thermometer.name.eq(&rosette_main.name),
            },
            Device::Thermometer(thermometer_main) => match other {
                Device::Rosette(rosette) => rosette.name.eq(&thermometer_main.name),
                Device::Thermometer(thermometer) => thermometer.name.eq(&thermometer_main.name),
            },
        }
    }
}

pub mod rosette;
pub mod thermometer;
