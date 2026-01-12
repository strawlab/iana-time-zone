use crate::GetTimezoneError;
use emscripten_functions::emscripten::run_script_string;

pub(crate) fn get_timezone_inner() -> Result<String, GetTimezoneError> {
    run_script_string("Intl.DateTimeFormat().resolvedOptions().timeZone")
        .ok_or_else(|| GetTimezoneError::OsError)
}
