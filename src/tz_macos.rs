pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    let tz = core_foundation::timezone::CFTimeZone::system();

    // Get string like ""Europe/Berlin (GMT+2) offset 7200 (Daylight)""
    let mut str1 = format!("{:?}", tz);

    // We split the string first, so removing the leading quotes has to move fewer characters.
    str1.truncate(match str1.split_ascii_whitespace().next() {
        Some(s) => s.len(),
        None => return Err(crate::GetTimezoneError::FailedParsingString),
    });

    let trim_amount = str1.len() - str1.trim_start_matches('"').len();
    if trim_amount == 0 {
        return Err(crate::GetTimezoneError::FailedParsingString);
    }

    str1.replace_range(..trim_amount, "");
    Ok(str1)
}
