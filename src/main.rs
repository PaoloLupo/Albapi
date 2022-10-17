mod gui;
mod api;

use winapi::um::oaidl::SAFEARRAY;
use crate::api::etabs_api;
use crate::gui::init::alba_gui_init;


macro_rules! assert_hr {
    ($e:expr) => {
        let hr = $e;
        if !winapi::shared::winerror::SUCCEEDED(hr) {
            panic!("{} failed with hr = 0x{:x}", stringify!($e), hr);
        }
    };
}
//
// macro_rules! ccomptr {
//     ($e:expr) => {
//         let  $e = std::ptr::null_mut();
//         let $e = $e as *mut etabs_api::c$e;
//         let $e = Box::into_raw(Box::new($e));
//     };
// }

/// Funcion responsable de la comunicación con ETABS y el software


fn main() {
    alba_gui_init();

}

unsafe fn bstr(s: &str) -> winapi::shared::wtypes::BSTR {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    winapi::um::oleauto::SysAllocString(v.as_ptr())
}
