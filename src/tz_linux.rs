pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    // see https://stackoverflow.com/a/12523283
    let mut contents = std::fs::read_to_string("/etc/timezone")?;
    // Trim to the correct length without allocating.
    contents.truncate(contents.trim_end().len());
    Ok(contents)
}
