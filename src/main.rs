mod etabs_api;
mod top_bar;

use crate::egui::Context;
use eframe::{egui, App, Frame};
use winapi::um::oaidl::SAFEARRAY;

pub struct Alba {
    boolean: bool,
}

impl Default for Alba {
    fn default() -> Self {
        Self { boolean: false }
    }
}

impl Alba {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for Alba {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.top_bar(ctx, frame);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}

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
fn etabs_api_com_init() {
    unsafe {
        // Initialize COM
        assert_hr!(winapi::um::objbase::CoInitialize(std::ptr::null_mut()));

        // Create an instance of the ETABS object
        let mut etabs_helper: *mut winapi::ctypes::c_void = std::ptr::null_mut();

        assert_hr!(winapi::um::combaseapi::CoCreateInstance(
            &<etabs_api::Helper as winapi::Class>::uuidof(),
            std::ptr::null_mut(),
            winapi::um::combaseapi::CLSCTX_INPROC,
            &<etabs_api::cHelper as winapi::Interface>::uuidof(),
            &mut etabs_helper
        ));

        // Create ETABS Helper
        let etabs_helper = &*(etabs_helper as *mut etabs_api::cHelper);

        // Create an instance of the ETABS object
        let etabs_object = std::ptr::null_mut();
        let etabs_object = etabs_object as *mut etabs_api::cOAPI;
        let etabs_object = Box::into_raw(Box::new(etabs_object));

        // Attach to an existing instance of ETABS
        etabs_helper.GetObject(bstr("CSI.ETABS.API.ETABSObject"), etabs_object);

        // Attach to an existing etabs model
        let etabs_model = std::ptr::null_mut();
        let etabs_model = etabs_model as *mut etabs_api::cSapModel;
        let etabs_model = Box::into_raw(Box::new(etabs_model));

        (&**etabs_object).get_SapModel(etabs_model);

        let mut number = 0.0;
        let ret = 0;
        // (&**etabs_model).InitializeNewModel(etabs_api::eUnits_kip_ft_F, ret as _);

        // let efile = std::ptr::null_mut();
        // let efile = efile as *mut etabs_api::cFile;
        // let efile = Box::into_raw(Box::new(efile));
        // (&**etabs_model).get_File(efile);
        // (&**efile).NewSteelDeck(4, 12.0, 12.0, 4, 4, 24.0, 24.0, ret as _);

        let pierlabel = std::ptr::null_mut();
        let pierlabel = pierlabel as *mut etabs_api::cPierLabel;
        let pierlabel = Box::into_raw(Box::new(pierlabel));

        (&**etabs_model).get_PierLabel(pierlabel);

        let number1 = 0;
        let number2 = 1;
        let safearray: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray1: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray2: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray3: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray4: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray5: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray6: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray7: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray8: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray9: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray10: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray11: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray12: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray13: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray14: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray15: *mut SAFEARRAY = std::ptr::null_mut();
        let safearray16: *mut SAFEARRAY = std::ptr::null_mut();

        (&**pierlabel).GetSectionProperties(
            bstr("MY14"),
            number1 as _,
            safearray1,
            safearray2,
            safearray3,
            safearray4,
            safearray5,
            safearray6,
            safearray7,
            safearray8,
            safearray9,
            safearray10,
            safearray11,
            safearray12,
            safearray13,
            safearray14,
            safearray15,
            number2 as _,
        );

        println!("number2: {}", number2);
        println!("safearray1: {:?}", safearray1);
        println!("safearray2: {:?}", safearray2);
        println!("safearray3: {:?}", safearray3);
        println!("safearray4: {:?}", safearray4);
        println!("safearray5: {:?}", safearray5);

        // *mut i32 */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut SAFEARRAY */, /* *mut i32 */
        // (&**etabs_object).GetOAPIVersionNumber(&mut number as _);
        // println!("Version: {}", number);
        //
        //
        println!("Version: {}", number);
    }
}

fn main() {
    etabs_api_com_init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Alba",
        native_options,
        Box::new(|ctx| Box::new(Alba::new(ctx))),
    )
}

unsafe fn bstr(s: &str) -> winapi::shared::wtypes::BSTR {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    winapi::um::oleauto::SysAllocString(v.as_ptr())
}
