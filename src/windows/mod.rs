mod hstring;
mod inspectable;
mod interfaces;
mod tz_on_calendar;
mod winapi;

use self::hstring::Hstring;
use self::inspectable::Inspectable;
use self::tz_on_calendar::TzOnCalendar;
use self::winapi::co_increment_mta_usage;
use self::winapi::windows_get_string_raw_buffer;

#[allow(clippy::upper_case_acronyms)]
#[repr(C)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
#[must_use]
pub struct HRESULT(i32);

// See <https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/705fb797-2175-4a90-b5a3-3918024b10b8>
const CO_E_NOTINITIALIZED: HRESULT = HRESULT(0x800401F0_u32 as i32);
const E_FAIL: HRESULT = HRESULT(0x80004005_u32 as i32);

const CLASS_NAME: Hstring = hstring::h!("Windows.Globalization.Calendar");

impl From<HRESULT> for crate::GetTimezoneError {
    fn from(orig: HRESULT) -> Self {
        std::io::Error::from_raw_os_error(orig.0).into()
    }
}

impl HRESULT {
    pub fn into_result(self) -> Result<(), HRESULT> {
        // per <https://learn.microsoft.com/en-us/windows/win32/api/winerror/nf-winerror-succeeded>
        if self.0 < 0 {
            return Err(self);
        }

        Ok(())
    }
}

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    // Create new "Windows.Globalization.Calendar" instance.
    let calendar = Inspectable::activate(&CLASS_NAME).or_else(|result| {
        // Some other library could have called CoIncrementMTAUsage() or CoInitializeEx(), so we
        // only call CoIncrementMTAUsage() if RoActivateInstance() tells us that multithreading
        // was not initialized, yet.
        if result != CO_E_NOTINITIALIZED {
            return Err(result);
        }

        // SAFETY: Using the function in `fn main()` is totally fine. If you go really low level
        //         and implement an "fn wWinMain()" somehow, then all bets are off anyway.
        unsafe { co_increment_mta_usage(&mut 0)? };

        Inspectable::activate(&CLASS_NAME)
    })?;

    // Query ITimeZoneOnCalendar of the calendar instance.
    let tz = TzOnCalendar::query(&calendar)?;

    // Get the name of the time zone.
    let name = Hstring::from_tz_on_calendar(&tz)?;
    windows_get_string_raw_buffer(&name)
        .ok()
        .and_then(|s| String::from_utf16(s).ok())
        .ok_or(crate::GetTimezoneError::FailedParsingString)
}
