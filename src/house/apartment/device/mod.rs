mod rosette;
mod thermometer;

use rosette::_Rosette;
use thermometer::_Thermometer;

pub enum _TypeDevice {
    Thermometer,
    Rosette,
}

pub enum _Device {
    Rosette(_Rosette),
    Thermometer(_Thermometer),
}
