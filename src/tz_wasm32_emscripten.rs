use libc::{localtime_r, time_t, tm, time};
use std::ffi::CStr;
use crate::GetTimezoneError;

pub(crate) fn get_timezone_inner() -> Result<String, GetTimezoneError> {
    unsafe {
        let mut now: time_t = 0;
        let mut local_tm: tm = std::mem::zeroed();
        
        if time(&mut now as *mut time_t) == -1 {
            return Err(GetTimezoneError::OsError);
        };
        
        if localtime_r(&now as *const time_t, &mut local_tm as *mut tm).is_null() {
            return Err(GetTimezoneError::OsError);
        }
        
        if local_tm.tm_zone.is_null() {
            return Err(GetTimezoneError::OsError);
        }
            
        let tz_str = CStr::from_ptr(local_tm.tm_zone).to_str()
            .map_err(|_| GetTimezoneError::FailedParsingString)?;
        Ok(tz_str.to_owned())
    }
}
