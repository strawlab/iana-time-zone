use core::{mem, ptr, slice};

use interfaces::ITimeZoneOnCalendar;
use once_cell::sync::OnceCell;
use winapi::ctypes::wchar_t;
use winapi::shared::winerror::{CO_E_NOTINITIALIZED, FAILED, HRESULT};
use winapi::um::combaseapi::CoIncrementMTAUsage;
use winapi::um::unknwnbase::IUnknown;
use winapi::winrt::activation::IActivationFactory;
use winapi::winrt::hstring::{HSTRING, HSTRING_HEADER};
use winapi::winrt::inspectable::IInspectable;
use winapi::winrt::roapi::RoGetActivationFactory;
use winapi::winrt::winstring::{
    WindowsCreateStringReference, WindowsDeleteString, WindowsGetStringRawBuffer,
};
use winapi::Interface;

use crate::{GetTimezoneError, Timezone};

pub(crate) type Error = HRESULT;

macro_rules! wstring {
    ($($letters:literal)* $(,)?) => {
        [ $($letters as wchar_t,)* ]
    };
}

const WINDOWS_GLOBALIZATION_CALENDAR: &[wchar_t] = &wstring!(
    'W' 'i' 'n' 'd' 'o' 'w' 's' '.'
    'G' 'l' 'o' 'b' 'a' 'l' 'i' 'z' 'a' 't' 'i' 'o' 'n' '.'
    'C' 'a' 'l' 'e' 'n' 'd' 'a' 'r'
    0
);

static mut FACTORY: OnceCell<Result<Unknown<IActivationFactory>, HRESULT>> = OnceCell::new();

#[repr(transparent)]
struct HString(HSTRING);

#[repr(transparent)]
struct Unknown<T>(*mut T);

#[inline]
pub(crate) fn get_timezone_inner() -> Result<Timezone, GetTimezoneError> {
    unsafe { get_timezone() }
}

unsafe fn get_timezone() -> Result<Timezone, GetTimezoneError> {
    // This function crates a Windows.Globalization.Calendar, gets its ITimeZoneOnCalendar, and
    // then the name of the timezone.

    // We memorize the calendar constructor instead of an instance, because the user could
    // change their timezone during the execution of the program. Caching the constructor makes
    // the stress-test example run about 3% faster.
    let factory = match FACTORY.get_or_init(|| initialize_factory()) {
        Ok(factory) => factory,
        Err(hr) => return Err(GetTimezoneError::IoError(*hr)),
    };

    let mut calendar: Unknown<IInspectable> = mem::zeroed();
    let hr = (*factory.0).ActivateInstance(mem::transmute(&mut calendar));
    if FAILED(hr) {
        return Err(GetTimezoneError::IoError(hr));
    }

    let mut tz_on_caledar: Unknown<ITimeZoneOnCalendar> = mem::zeroed();
    let hr = (*calendar.0).QueryInterface(
        &ITimeZoneOnCalendar::uuidof(),
        mem::transmute(&mut tz_on_caledar),
    );
    drop(calendar);
    if FAILED(hr) {
        return Err(GetTimezoneError::IoError(hr));
    }

    let mut timezone: HString = mem::zeroed();
    let hr = (*tz_on_caledar.0).GetTimeZone(mem::transmute(&mut timezone));
    drop(tz_on_caledar);
    if FAILED(hr) {
        return Err(GetTimezoneError::IoError(hr));
    }

    let mut len = 0;
    let buf = WindowsGetStringRawBuffer(timezone.0, &mut len);
    let buf = slice::from_raw_parts::<wchar_t>(&*buf, len as usize);
    Timezone::try_from_utf16(buf).ok_or(GetTimezoneError::FailedParsingString)
}

unsafe fn initialize_factory() -> Result<Unknown<IActivationFactory>, HRESULT> {
    // Some other liberary could have called CoIncrementMTAUsage() or CoInitializeEx(), so we only
    // call CoIncrementMTAUsage() if RoGetActivationFactory() tells us that multithreading was not
    // initialized, yet.

    let mut h_class_name: HString = mem::zeroed();
    let mut string_header: HSTRING_HEADER = mem::zeroed();
    let hr = WindowsCreateStringReference(
        WINDOWS_GLOBALIZATION_CALENDAR.as_ptr(),
        (WINDOWS_GLOBALIZATION_CALENDAR.len() - 1) as _,
        &mut string_header as *mut _,
        mem::transmute(&mut h_class_name),
    );
    if FAILED(hr) {
        return Err(hr);
    }

    let mut factory: Unknown<IActivationFactory> = mem::zeroed();
    let hr = RoGetActivationFactory(
        h_class_name.0,
        &IActivationFactory::uuidof(),
        mem::transmute(&mut factory),
    );
    if !FAILED(hr) {
        return Ok(factory);
    } else if hr != CO_E_NOTINITIALIZED {
        return Err(hr);
    }

    // No need to check the error. The only conceivable error code this function returns is
    // E_OUTOFMEMORY, and the program is about to get OOM killed anyway in this case. Windows-rs
    // does not check the result, either.
    let mut cookie = mem::zeroed();
    let _ = CoIncrementMTAUsage(&mut cookie);

    let mut factory: Unknown<IActivationFactory> = mem::zeroed();
    let hr = RoGetActivationFactory(
        h_class_name.0,
        &IActivationFactory::uuidof(),
        mem::transmute(&mut factory),
    );
    match !FAILED(hr) {
        true => Ok(factory),
        false => Err(hr),
    }
}

impl Drop for HString {
    fn drop(&mut self) {
        let string = mem::replace(&mut self.0, ptr::null_mut());
        if !string.is_null() {
            unsafe { WindowsDeleteString(string) };
        }
    }
}

impl<T> Drop for Unknown<T> {
    fn drop(&mut self) {
        let instance = mem::replace(&mut self.0, ptr::null_mut());
        if !instance.is_null() {
            unsafe { (*(instance as *mut IUnknown)).Release() };
        }
    }
}

#[allow(non_snake_case, non_camel_case_types)]
mod interfaces {
    use winapi::shared::minwindef::DWORD;
    use winapi::shared::winerror::HRESULT;
    use winapi::winrt::hstring::HSTRING;
    use winapi::winrt::inspectable::{IInspectable, IInspectableVtbl};
    use winapi::RIDL;

    RIDL! {
        #[uuid(0xbb3c25e5, 0x46cf, 0x4317, 0xa3, 0xf5, 0x02, 0x62, 0x1a, 0xd5, 0x44, 0x78)]
        interface ITimeZoneOnCalendar(ITimeZoneOnCalendar_Vtbl): IInspectable(IInspectableVtbl) {
            fn GetTimeZone(result: &mut HSTRING,) -> HRESULT,
            fn ChangeTimeZone(timezoneid: HSTRING,) -> HRESULT,
            fn TimeZoneAsFullString(result: &mut HSTRING,) -> HRESULT,
            fn TimeZoneAsString(ideallength: &DWORD, result: &mut HSTRING,) -> HRESULT,
        }
    }
}
