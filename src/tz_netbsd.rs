use std::fs::read_link;

const PREFIX: &str = "/usr/share/zoneinfo/";

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    // see https://www.cyberciti.biz/faq/openbsd-time-zone-howto/
    let mut s = read_link("/etc/localtime")?
        .into_os_string()
        .into_string()
        .map_err(|_| crate::GetTimezoneError::FailedParsingString)?;
    if !s.starts_with(PREFIX) {
        return Err(crate::GetTimezoneError::FailedParsingString);
    }

    // Trim to the correct length without allocating.
    s.replace_range(..PREFIX.len(), "");
    Ok(s)
}
