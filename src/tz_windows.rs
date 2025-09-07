use std::{
    ffi::CString,
    io::{Error, ErrorKind},
};

use windows_bindings::Windows::Win32::{
    Globalization::{ucal_getTimeZoneIDForWindowsID, GetUserDefaultGeoName},
    System::{
        SystemServices::{TIME_ZONE_ID_DAYLIGHT, TIME_ZONE_ID_STANDARD, TIME_ZONE_ID_UNKNOWN},
        Time::{GetDynamicTimeZoneInformation, TIME_ZONE_ID_INVALID},
    },
};

#[path = "windows_bindings.rs"]
#[allow(missing_debug_implementations, clippy::undocumented_unsafe_blocks)]
mod windows_bindings;

/// <https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-getdynamictimezoneinformation>
/// <https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/ns-timezoneapi-dynamic_time_zone_information>
pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    let mut tz = Default::default();
    // SAFETY: Calling an external Windows function, but with a valid pointer
    let ret = unsafe { GetDynamicTimeZoneInformation(&mut tz) };
    dbg!(ret);
    match ret {
        TIME_ZONE_ID_INVALID => {
            return Err(crate::GetTimezoneError::IoError(Error::last_os_error()));
        }
        // TIME_ZONE_ID_UNKNOWN => panic!("ID_UNKNOWN with result {tz:?}"),
        TIME_ZONE_ID_UNKNOWN | TIME_ZONE_ID_STANDARD | TIME_ZONE_ID_DAYLIGHT => {}
        x => {
            return Err(crate::GetTimezoneError::IoError(Error::new(
                ErrorKind::Other,
                format!("Unexpected GetDynamicTimeZoneInformation() return `{x}`"),
            )));
        }
    };
    let windows_tz_id = &tz.TimeZoneKeyName;
    let terminator = windows_tz_id.iter().position(|&c| c == 0).unwrap();
    let windows_tz_id_str = String::from_utf16(&windows_tz_id[..terminator])
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    dbg!(&windows_tz_id_str);

    let mut region = [0; 128];
    // Returns the length including NUL terminator
    // SAFETY: Calling an external Windows function, but with a valid pointer
    let ret = unsafe { GetUserDefaultGeoName(region.as_mut_ptr(), region.len() as _) };
    if ret == 0 {
        return Err(crate::GetTimezoneError::IoError(Error::last_os_error()));
    }
    // Contains an interior NUL already
    let region =
        String::from_utf16(&region[..ret as usize]).map_err(|e| Error::new(ErrorKind::Other, e))?;
    // TODO: The CStr adds nothing besides needing a cast on .as_ptr() below.
    let region = CString::from_vec_with_nul(region.into_bytes()).expect("Unreachable");
    dbg!(&region);

    let mut out = [0u16; 128];
    let mut status = 0;
    // SAFETY: Calling an external Windows function, but with valid pointers
    let ret = unsafe {
        ucal_getTimeZoneIDForWindowsID(
            windows_tz_id.as_ptr(),
            -1,
            region.as_ptr() as _,
            out.as_mut_ptr(),
            out.len() as i32,
            &mut status,
        )
    };
    assert_eq!(status, 0, "UErrorCode: {status:x}");
    let tz_id =
        String::from_utf16(&out[..ret as usize]).map_err(|e| Error::new(ErrorKind::Other, e))?;
    dbg!(&tz_id);
    Ok(tz_id)
}
