#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::c_void;
use std::ops::Deref;

use super::hstring::HstringHeader;
use super::{GUID, HRESULT};

pub type IUnknown = *mut *const IUnknown_Vtbl;
pub type IInspectable = *mut *const IInspectable_Vtbl;
pub type ITimeZoneOnCalendar = *mut *const ITimeZoneOnCalendar_Vtbl;

#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface:
        unsafe extern "system" fn(this: IUnknown, iid: &GUID, interface: *mut IUnknown) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: IUnknown) -> u32,
    pub Release: unsafe extern "system" fn(this: IUnknown) -> u32,
}

#[repr(C)]
pub struct IInspectable_Vtbl {
    pub base: IUnknown_Vtbl,
    pub GetIids: unsafe extern "system" fn(
        this: IInspectable,
        count: *mut u32,
        values: *mut *mut GUID,
    ) -> HRESULT,
    pub GetRuntimeClassName:
        unsafe extern "system" fn(this: IInspectable, value: *mut *mut HstringHeader) -> HRESULT,
    pub GetTrustLevel: unsafe extern "system" fn(this: IInspectable, value: *mut i32) -> HRESULT,
}

#[repr(C)]
pub struct ITimeZoneOnCalendar_Vtbl {
    pub base: IInspectable_Vtbl,
    pub GetTimeZone: unsafe extern "system" fn(
        this: ITimeZoneOnCalendar,
        result: *mut *mut HstringHeader,
    ) -> HRESULT,
    pub ChangeTimeZone: unsafe extern "system" fn(
        this: ITimeZoneOnCalendar,
        timezoneid: *mut HstringHeader,
    ) -> HRESULT,
    pub TimeZoneAsFullString: unsafe extern "system" fn(
        this: ITimeZoneOnCalendar,
        result: *mut *mut HstringHeader,
    ) -> HRESULT,
    pub TimeZoneAsString: unsafe extern "system" fn(
        this: ITimeZoneOnCalendar,
        ideallength: i32,
        result: *mut *mut c_void,
    ) -> HRESULT,
}

impl Deref for IInspectable_Vtbl {
    type Target = IUnknown_Vtbl;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl Deref for ITimeZoneOnCalendar_Vtbl {
    type Target = IInspectable_Vtbl;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
