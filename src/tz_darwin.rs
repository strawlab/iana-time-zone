use objc2_core_foundation::{CFTimeZoneCopySystem, CFTimeZoneGetName};

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    get_timezone().ok_or(crate::GetTimezoneError::OsError)
}

/// Get system time zone, and extract its name.
#[inline]
fn get_timezone() -> Option<String> {
    // SAFETY: No invariants to uphold.
    // `CFRetained` will take care of memory management.
    let tz = unsafe { CFTimeZoneCopySystem() }?;

    // SAFETY: No invariants to uphold (the `CFTimeZone` is valid).
    let name = unsafe { CFTimeZoneGetName(&tz) }?;

    Some(name.to_string())
}
