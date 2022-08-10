use std::fs::{read_link, read_to_string};

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    etc_localtime().or_else(|_| etc_timezone())
}

fn etc_timezone() -> Result<String, crate::GetTimezoneError> {
    // see https://stackoverflow.com/a/12523283
    let mut contents = read_to_string("/etc/timezone")?;
    // Trim to the correct length without allocating.
    contents.truncate(contents.trim_end().len());
    Ok(contents)
}

fn etc_localtime() -> Result<String, crate::GetTimezoneError> {
    // https://www.man7.org/linux/man-pages/man5/localtime.5.html
    // https://www.cyberciti.biz/faq/openbsd-time-zone-howto/
    const PREFIX: &str = "/usr/share/zoneinfo/";

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
