use rosette::Rosette;
use thermometer::Thermometer;

use serde::{Deserialize, Serialize};

/// Create devices
/// ```
/// use smart_house::{Device, Rosette, Thermometer};
/// let thermometer = Device::Thermometer(Thermometer::new("name".to_string(), 23.0, "127.0.0.1:9091".to_string()));
/// let rosette = Device::Rosette(Rosette::new("name".to_string(), "127.0.0.1:8081".to_string()));
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Device {
    Rosette(Rosette),
    Thermometer(Thermometer),
}

impl PartialEq<Self> for Device {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Device::Rosette(rosette_main) => match other {
                Device::Rosette(rosette) => rosette == rosette_main,
                Device::Thermometer(thermometer) => thermometer == rosette_main,
            },
            Device::Thermometer(thermometer_main) => match other {
                Device::Rosette(rosette) => rosette == thermometer_main,
                Device::Thermometer(thermometer) => thermometer == thermometer_main,
            },
        }
    }
}
impl Eq for Device {}

pub mod rosette;
pub mod thermometer;
