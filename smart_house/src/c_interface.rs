// use crate::House;
// use std::os::raw::c_char;
//
// pub(crate) type HouseFn = unsafe extern "C" fn(c_char, c_char) -> House;
//
// /// # Safety
// pub unsafe extern "C" fn get_house(id: c_char, name: c_char) -> House {
//     House::new(id.to_string(), name.to_string())
// }
