use std::mem::zeroed;

use windows_sys::core::{GUID, HRESULT};
use windows_sys::Win32::System::Com::CoIncrementMTAUsage;
use windows_sys::Win32::System::WinRT::HSTRING_HEADER;

use self::hstring::HString;
use self::instance::Instance;
use self::tz_on_calendar::TzOnCalendar;

macro_rules! wstring {
    ($($letters:tt)+) => {
        [ $($letters as _,)+ ]
    };
}

const WINDOWS_GLOBALIZATION_CALENDAR: &[u16] = &wstring!(
    'W' 'i' 'n' 'd' 'o' 'w' 's' '.'
    'G' 'l' 'o' 'b' 'a' 'l' 'i' 'z' 'a' 't' 'i' 'o' 'n' '.'
    'C' 'a' 'l' 'e' 'n' 'd' 'a' 'r'
    0
);

const TIMEZONE_ON_CALENDAR_GUID: GUID = GUID {
    data1: 0xbb3c25e5,
    data2: 0x46cf,
    data3: 0x4317,
    data4: [0xa3, 0xf5, 0x02, 0x62, 0x1a, 0xd5, 0x44, 0x78],
};

const CO_E_NOTINITIALIZED: HRESULT = -2147221008;

impl From<HRESULT> for crate::GetTimezoneError {
    fn from(orig: HRESULT) -> Self {
        std::io::Error::from_raw_os_error(orig).into()
    }
}

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    // Get HSTRING for "Windows.Globalization.Calendar".
    // SAFETY: An `HSTRING_HEADER` actually does not need initialization when used with
    //         `WindowsCreateStringReference()`, but zeroing it is as good a initial value as any.
    let mut string_header: HSTRING_HEADER = unsafe { zeroed() };
    let class_name = HString::create(WINDOWS_GLOBALIZATION_CALENDAR, &mut string_header)?;

    // Create new "Windows.Globalization.Calendar" instance.
    let calendar = Instance::activate(&class_name).or_else(|result| {
        // Some other library could have called CoIncrementMTAUsage() or CoInitializeEx(), so we
        // only call CoIncrementMTAUsage() if RoActivateInstance() tells us that multithreading
        // was not initialized, yet.

        // No need to check the error. The only conceivable error code this function returns is
        // E_OUTOFMEMORY, and the program is about to get OOM killed anyway in this case.
        // Windows-rs does not check the result, either.

        if result != CO_E_NOTINITIALIZED {
            return Err(result);
        }
        let mut cookie = 0;
        // SAFETY: "Don't call CoIncrementMTAUsage during process shutdown or inside dllmain."
        //         Using the function is `fn main()` is totally fine. If you go really low level
        //         and implement an "fn wWinMain()" somehow, then all bets are off anyway.
        let _ = unsafe { CoIncrementMTAUsage(&mut cookie) };

        Instance::activate(&class_name)
    })?;

    // Query ITimeZoneOnCalendar of the calendar instance.
    let tz = TzOnCalendar::query(&calendar)?;

    // Get the name of the time zone.
    let name = HString::from_tz_on_calendar(&tz)?;

    // Convert to Rust String
    Ok(name.to_string())
}

mod hstring {
    use std::ptr::null_mut;

    use windows_sys::core::{HRESULT, HSTRING};
    use windows_sys::Win32::System::WinRT::WindowsDeleteString;
    use windows_sys::Win32::System::WinRT::{
        WindowsCreateStringReference, WindowsGetStringRawBuffer, HSTRING_HEADER,
    };

    use super::tz_on_calendar::TzOnCalendar;

    pub struct HString<'a> {
        string: HSTRING,
        _header: Option<&'a mut HSTRING_HEADER>,
    }

    impl<'a> HString<'a> {
        // `source` must be null-terminated. Windows tests if the the terminator is missing and
        // returns an error if it is absent.
        pub fn create(source: &'a [u16], header: &'a mut HSTRING_HEADER) -> Result<Self, HRESULT> {
            let mut string = null_mut();
            // SAFETY: `source` is a valid reference. If its contents are not a valid wide string,
            //         then the call will return an error code. We keep a reference to the `source`
            //         and `header`, so they stay valid until the `HSTRING` is released.
            let result = unsafe {
                WindowsCreateStringReference(
                    source.as_ptr(),
                    (source.len().saturating_sub(1)) as u32,
                    header,
                    &mut string,
                )
            };
            if result < 0 || string.is_null() {
                Err(result)
            } else {
                Ok(Self {
                    string,
                    _header: Some(header),
                })
            }
        }

        pub fn from_tz_on_calendar(instance: &'a TzOnCalendar) -> Result<Self, HRESULT> {
            let mut string = null_mut();
            // SAFETY: A `TzOnCalendar` is only ever created with a valid instance.
            let result = unsafe {
                let instance = instance.as_ptr();
                ((**instance).GetTimeZone)(instance, &mut string)
            };
            if result < 0 || string.is_null() {
                Err(result)
            } else {
                Ok(Self {
                    string,
                    _header: None,
                })
            }
        }

        /// SAFETY: You are not allowed to release the returned pointer.
        pub unsafe fn as_ptr(&self) -> HSTRING {
            self.string
        }
    }

    impl ToString for HString<'_> {
        fn to_string(&self) -> String {
            let mut len = 0;
            // SAFETY: An `HString` is only ever created with a valid `HSTRING`.
            // It keeps a reference to `HSTRING_HEADER` if needed.
            let buf = unsafe { WindowsGetStringRawBuffer(self.string, &mut len) };
            if len == 0 || buf.is_null() {
                return String::new();
            }

            // SAFETY: `WindowsGetStringRawBuffer` returns a valid pointer to a wide string.
            let slice = unsafe { std::slice::from_raw_parts(buf, len as usize) };
            String::from_utf16_lossy(slice)
        }
    }

    impl Drop for HString<'_> {
        fn drop(&mut self) {
            // SAFETY: An `HString` is only ever created with a valid `HSTRING`.
            unsafe { WindowsDeleteString(self.string) };
        }
    }
}

mod instance {
    use std::ptr::null_mut;

    use windows_sys::core::HRESULT;
    use windows_sys::Win32::System::WinRT::RoActivateInstance;

    use super::hstring::HString;
    use super::interfaces::IUnknown;

    pub struct Instance(IUnknown);

    impl Instance {
        pub fn activate(class_id: &HString<'_>) -> Result<Self, HRESULT> {
            let mut instance = null_mut();
            // SAFETY: An `HString` is only ever crated with a valid `HSTRING`.
            let result = unsafe { RoActivateInstance(class_id.as_ptr(), &mut instance) };
            if result < 0 || instance.is_null() {
                Err(result)
            } else {
                Ok(Self(instance.cast()))
            }
        }

        /// SAFETY: You are not allowed to release the returned pointer.
        pub unsafe fn as_ptr(&self) -> IUnknown {
            self.0
        }
    }

    impl Drop for Instance {
        fn drop(&mut self) {
            // SAFETY: An `Instance` is only ever created with a valid `IUnknown`.
            unsafe { ((**self.0).Release)(self.0) };
        }
    }
}

mod tz_on_calendar {
    use std::ptr::null_mut;

    use windows_sys::core::HRESULT;

    use super::instance::Instance;
    use super::interfaces::{ITimeZoneOnCalendar, IUnknown};
    use super::TIMEZONE_ON_CALENDAR_GUID;

    pub struct TzOnCalendar(ITimeZoneOnCalendar);

    impl TzOnCalendar {
        pub fn query(source: &Instance) -> Result<Self, HRESULT> {
            let mut tz = null_mut();
            // SAFETY: An `Instance` is only ever created with a valid `IUnknown`.
            let result = unsafe {
                let source = source.as_ptr();
                ((**source).QueryInterface)(source, &TIMEZONE_ON_CALENDAR_GUID, &mut tz)
            };
            if result < 0 || tz.is_null() {
                Err(result)
            } else {
                Ok(Self(tz.cast()))
            }
        }

        /// SAFETY: You are not allowed to release the returned pointer.
        pub unsafe fn as_ptr(&self) -> ITimeZoneOnCalendar {
            self.0
        }
    }

    impl Drop for TzOnCalendar {
        fn drop(&mut self) {
            let v: IUnknown = self.0.cast();
            // SAFETY: `TzOnCalendar` is only ever created with a valid `ITimeZoneOnCalendar`.
            unsafe { ((**v).Release)(v) };
        }
    }
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod interfaces {
    use std::ops::Deref;

    use windows_sys::core::{GUID, HRESULT, HSTRING};

    pub type IUnknown = *mut *const IUnknown_Vtbl;
    pub type IInspectable = *mut *const IInspectable_Vtbl;
    pub type ITimeZoneOnCalendar = *mut *const ITimeZoneOnCalendar_Vtbl;

    #[repr(C)]
    pub struct IUnknown_Vtbl {
        pub QueryInterface: unsafe extern "system" fn(
            this: IUnknown,
            iid: &GUID,
            interface: &mut IUnknown,
        ) -> HRESULT,
        pub AddRef: unsafe extern "system" fn(this: IUnknown) -> u32,
        pub Release: unsafe extern "system" fn(this: IUnknown) -> u32,
    }

    #[repr(C)]
    pub struct IInspectable_Vtbl {
        pub base: IUnknown_Vtbl,
        pub GetIids: unsafe extern "system" fn(
            this: IInspectable,
            count: &mut u32,
            values: &mut &mut GUID,
        ) -> HRESULT,
        pub GetRuntimeClassName:
            unsafe extern "system" fn(this: IInspectable, value: &mut HSTRING) -> HRESULT,
        pub GetTrustLevel:
            unsafe extern "system" fn(this: IInspectable, value: &mut i32) -> HRESULT,
    }

    #[repr(C)]
    pub struct ITimeZoneOnCalendar_Vtbl {
        pub base: IInspectable_Vtbl,
        pub GetTimeZone:
            unsafe extern "system" fn(this: ITimeZoneOnCalendar, result: &mut HSTRING) -> HRESULT,
        pub ChangeTimeZone:
            unsafe extern "system" fn(this: ITimeZoneOnCalendar, timezoneid: HSTRING) -> HRESULT,
        pub TimeZoneAsFullString:
            unsafe extern "system" fn(this: ITimeZoneOnCalendar, result: *mut HSTRING) -> HRESULT,
        pub TimeZoneAsString: unsafe extern "system" fn(
            this: ITimeZoneOnCalendar,
            ideallength: i32,
            result: &mut HSTRING,
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
}
