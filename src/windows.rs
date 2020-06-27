use winrt::*;

import!(
    dependencies
        os
    types
        windows::globalization::*
);

impl std::convert::From<winrt::Error> for crate::GetTimezoneError {
    fn from(_orig: winrt::Error) -> Self {
        crate::GetTimezoneError::OsError
    }
}

pub(crate) fn get_timezone_inner() -> std::result::Result<String,crate::GetTimezoneError> {
    use windows::globalization::Calendar;

    let cal = Calendar::new()?;
    let tz_hstring = cal.get_time_zone()?;
    Ok(tz_hstring.to_string())
}
