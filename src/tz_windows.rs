use std::mem;

use windows_bindings::Windows::Win32::System::Time::GetDynamicTimeZoneInformation;

#[path = "windows_bindings.rs"]
#[allow(missing_debug_implementations, clippy::undocumented_unsafe_blocks)]
mod windows_bindings;

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    let mut tz = unsafe { mem::zeroed() };
    // let cal = Calendar::new()?;
    // let tz_hstring = cal.GetTimeZone()?;
    // Ok(tz_hstring.to_string())
    let err = unsafe { GetDynamicTimeZoneInformation(&mut tz) };
    if err != 0 {
        return Err(crate::GetTimezoneError::IoError(
            // TODO: Are these HRESULT-compatible?
            std::io::Error::from_raw_os_error(err as i32),
        ));
    }
    let terminator = tz.StandardName.iter().position(|&c| c == 0).unwrap();
    Ok(String::from_utf16(&tz.StandardName[..terminator]).map_err(std::io::Error::other)?)
}
