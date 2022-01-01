pub mod rosette;
pub mod thermometer;

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
