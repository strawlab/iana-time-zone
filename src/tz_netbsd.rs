use core::convert::TryInto;
use core::str::from_utf8;

use libc::readlink;

use crate::{GetTimezoneError, Timezone};

pub type Error = i32;

const PREFIX: &str = "/usr/share/zoneinfo/";
const PATH: &[u8] = b"/etc/localtime\0";

pub(crate) fn get_timezone_inner() -> Result<Timezone, GetTimezoneError> {
    let mut buf = [0u8; 64];
    let len = unsafe { readlink(PATH.as_ptr().cast(), buf.as_mut_ptr().cast(), buf.len()) };
    let len = match len.try_into() {
        Ok(len) => len,
        Err(_) => return Err(GetTimezoneError::IoError(errno::errno().0)),
    };
    let buf = match from_utf8(&buf[..len]) {
        Ok(buf) => buf,
        Err(_) => return Err(GetTimezoneError::FailedParsingString),
    };
    if !buf.starts_with(PREFIX) {
        return Err(GetTimezoneError::FailedParsingString);
    }
    Timezone::new(&buf[PREFIX.len()..]).ok_or(GetTimezoneError::FailedParsingString)
}
