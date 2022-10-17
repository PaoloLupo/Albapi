use crate::etabs_api;

macro_rules! assert_hr {
    ($e:expr) => {
        let hr = $e;
        if !winapi::shared::winerror::SUCCEEDED(hr) {
        }
    };
}

// struct EtabsAPI {
//     helper: etabs_api::cHelper,
//     object: etabs_api::cOAPI,
// }
//
// impl EtabsAPI {
//
//
//     pub fn initialize_com() -> Self {
//         unsafe {
//             winapi::um::objbase::CoInitialize(std::ptr::null_mut());
//
//             winapi::um::combaseapi::CoCreateInstance(
//                 &<etabs_api::Helper as winapi::Class>::uuidof(),
//                 std::ptr::null_mut(),
//                 winapi::um::combaseapi::CLSCTX_INPROC,
//                 &<etabs_api::cHelper as winapi::Interface>::uuidof(),
//                 &mut etabs_helper,
//             );
//
//             let mut etabs_helper: *mut winapi::ctypes::c_void = std::ptr::null_mut();
//
//             let etabs_helper = &*(etabs_helper as *mut etabs_api::cHelper);
//
//         }
//
//         Self {
//             helper: etabs_helper,
//             object: cOAPI {}
//         }
//
//
//
//
//
//     }
// }
