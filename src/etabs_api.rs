#![allow(non_camel_case_types, non_snake_case, unused, non_upper_case_globals)]

use winapi::{ENUM, RIDL, STRUCT};
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::UINT;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, LPDISPATCH, VARIANT, SAFEARRAY};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl, LPUNKNOWN};

include!(concat!(env!("OUT_DIR"), "/etabs_api.rs"));
