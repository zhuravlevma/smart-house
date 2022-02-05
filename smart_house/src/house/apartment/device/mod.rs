use rosette::Rosette;
use thermometer::Thermometer;

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
