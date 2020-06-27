pub(crate) fn get_timezone_inner() -> Result<String,crate::GetTimezoneError> {
    let tz = core_foundation::timezone::CFTimeZone::system();

    // Get string like ""Europe/Berlin (GMT+2) offset 7200 (Daylight)""
    let mut str1 = format!("{:?}", tz);

    // strip leading double quotes
    while str1.starts_with('"') {
        str1 = str1[1..].to_string();
    }

    match str1.split_whitespace().next() {
        Some(s) => Ok(s.to_string()),
        None => Err(crate::GetTimezoneError::FailedParsingString),
    }
}
