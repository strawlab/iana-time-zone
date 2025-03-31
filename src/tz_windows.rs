use std::mem;

use windows_bindings::Windows::Win32::System::{
    SystemServices::{TIME_ZONE_ID_DAYLIGHT, TIME_ZONE_ID_STANDARD, TIME_ZONE_ID_UNKNOWN},
    Time::{GetDynamicTimeZoneInformation, TIME_ZONE_ID_INVALID},
};

#[path = "windows_bindings.rs"]
#[allow(missing_debug_implementations, clippy::undocumented_unsafe_blocks)]
mod windows_bindings;

/// <https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/nf-timezoneapi-getdynamictimezoneinformation>
/// <https://learn.microsoft.com/en-us/windows/win32/api/timezoneapi/ns-timezoneapi-dynamic_time_zone_information>
pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    let mut tz = unsafe { mem::zeroed() };
    let ret = unsafe { GetDynamicTimeZoneInformation(&mut tz) };
    dbg!(ret);
    match ret {
        TIME_ZONE_ID_INVALID => {
            return Err(crate::GetTimezoneError::IoError(
                std::io::Error::last_os_error(),
            ));
        }
        // XXX: I think this is
        TIME_ZONE_ID_UNKNOWN => todo!(),
        TIME_ZONE_ID_STANDARD | TIME_ZONE_ID_DAYLIGHT => {}
        x => {
            return Err(crate::GetTimezoneError::IoError(std::io::Error::other(
                format!("Unexpected GetDynamicTimeZoneInformation() return `{x}`"),
            )));
        }
    };
    let windows_tz_id = &tz.TimeZoneKeyName;
    let terminator = windows_tz_id.iter().position(|&c| c == 0).unwrap();
    let windows_tz_id_str =
        String::from_utf16(&windows_tz_id[..terminator]).map_err(std::io::Error::other)?;
    dbg!(&windows_tz_id_str);

    type UChar = u16;
    #[repr(transparent)]
    struct UErrorCode(i32);

    // TODO: Link against windowsapp? Use rust_icu crate?
    #[link(name = "icu")]
    unsafe extern "C" {
        pub fn ucal_getTimeZoneIDForWindowsID(
            winid: *const UChar,
            len: i32,
            region: *const ::std::os::raw::c_char,
            id: *mut UChar,
            idCapacity: i32,
            status: *mut UErrorCode,
        ) -> i32;
    }

    let mut out = [0u16; 60];

    let mut status = UErrorCode(0);
    let ret = unsafe {
        ucal_getTimeZoneIDForWindowsID(
            windows_tz_id.as_ptr(),
            -1,
            // windows_tz_id_str.len() as _,
            std::ptr::null(),
            out.as_mut_ptr(),
            out.len() as i32,
            &mut status,
        )
    };
    assert_eq!(status.0, 0);
    let tz_id = String::from_utf16(&out[..ret as usize]).map_err(std::io::Error::other)?;
    dbg!(&tz_id);
    Ok(tz_id)
}
