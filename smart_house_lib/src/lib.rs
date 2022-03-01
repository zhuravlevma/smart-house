pub use errors::{
    AddDataError, AddDataResult, GetDataError, GetDataResult, RemoveDataError, RemoveDataResult,
};
pub use house::apartment::device::rosette::Rosette;
pub use house::apartment::device::thermometer::Thermometer;
pub use house::apartment::device::Device;
pub use house::apartment::Apartment;
pub use house::House;

// use c_interface::HouseFn;
// use crate::c_interface::get_house;
//
// #[allow(unused)]
// #[repr(C)]
// pub struct FunctionsBlock {
//     size: usize,
//     house: HouseFn
// }
//
// impl Default for FunctionsBlock {
//     fn default() -> Self {
//         Self {
//             size: std::mem::size_of::<Self>(),
//             house: get_house,
//         }
//     }
// }
//
// #[no_mangle]
// pub extern "C" fn functions() -> FunctionsBlock {
//     FunctionsBlock::default()
// }

pub mod c_interface;
pub mod errors;
pub mod house;
