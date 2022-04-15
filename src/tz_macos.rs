pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    Ok(core_foundation::timezone::CFTimeZone::system()
        .name()
        .to_string())
}
