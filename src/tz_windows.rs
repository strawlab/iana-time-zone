use std::mem;

use windows_bindings::Windows::Win32::System::{
    SystemServices::{TIME_ZONE_ID_DAYLIGHT, TIME_ZONE_ID_STANDARD, TIME_ZONE_ID_UNKNOWN},
    Time::{GetDynamicTimeZoneInformation, TIME_ZONE_ID_INVALID},
};

#[path = "windows_bindings.rs"]
#[allow(missing_debug_implementations, clippy::undocumented_unsafe_blocks)]
mod windows_bindings;

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    let mut tz = unsafe { mem::zeroed() };
    let ret = unsafe { GetDynamicTimeZoneInformation(&mut tz) };
    dbg!(ret);
    let name = match ret {
        TIME_ZONE_ID_INVALID => {
            return Err(crate::GetTimezoneError::IoError(
                std::io::Error::last_os_error(),
            ));
        }
        TIME_ZONE_ID_UNKNOWN => todo!(),
        TIME_ZONE_ID_STANDARD => &tz.StandardName,
        TIME_ZONE_ID_DAYLIGHT => &tz.DaylightName,
        x => return Err(crate::GetTimezoneError::OsError),
    };
    let terminator = name.iter().position(|&c| c == 0).unwrap();
    Ok(String::from_utf16(&name[..terminator]).map_err(std::io::Error::other)?)
}
