pub(crate) type Error = core::convert::Infallible;

pub(crate) fn get_timezone_inner() -> Result<crate::Timezone, crate::GetTimezoneError> {
    let name = core_foundation::timezone::CFTimeZone::system().name();
    crate::Timezone::try_from_display(name).ok_or(crate::GetTimezoneError::FailedParsingString)
}
