pub use errors::{
    AddDataError, AddDataResult, GetDataError, GetDataResult, RemoveDataError, RemoveDataResult,
};
pub use house::apartment::device::rosette::Rosette;
pub use house::apartment::device::thermometer::Thermometer;
pub use house::apartment::device::Device;
pub use house::apartment::Apartment;
pub use house::House;

// #[no_mangle]
// pub extern "C" fn get_my_integer() -> i32 {
//     45
// }

pub mod c_interface;
pub mod errors;
pub mod house;
