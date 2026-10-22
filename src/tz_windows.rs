#[expect(
    dead_code,
    missing_debug_implementations,
    non_snake_case,
    clippy::undocumented_unsafe_blocks,
    reason = "windows-bindgen generates these bindings from Windows metadata"
)]
mod windows_bindings;
use windows_bindings::Windows::Globalization::Calendar;

impl From<windows_core::Error> for crate::GetTimezoneError {
    fn from(orig: windows_core::Error) -> Self {
        crate::GetTimezoneError::IoError(std::io::Error::other(orig))
    }
}

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    let cal = Calendar::new()?;
    let tz_hstring = cal.GetTimeZone()?;
    Ok(tz_hstring.to_string_lossy())
}
