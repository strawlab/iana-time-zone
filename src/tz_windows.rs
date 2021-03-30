mod bindings {
    ::windows::include_bindings!();
}

use bindings::Windows::Globalization::Calendar;

impl std::convert::From<windows::Error> for crate::GetTimezoneError {
    fn from(_orig: windows::Error) -> Self {
        crate::GetTimezoneError::OsError
    }
}

pub(crate) fn get_timezone_inner() -> std::result::Result<String, crate::GetTimezoneError> {
    let cal = Calendar::new()?;
    let tz_hstring = cal.GetTimeZone()?;
    Ok(tz_hstring.to_string())
}
