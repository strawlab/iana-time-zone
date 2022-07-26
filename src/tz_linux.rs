use core::str::from_utf8;

use rustix::fs::{cwd, openat, Mode, OFlags};
use rustix::io::{read, Errno};

use crate::{GetTimezoneError, Timezone};

pub(crate) type Error = Errno;

// see https://stackoverflow.com/a/12523283
const PATH: &str = "/etc/timezone";

pub(crate) fn get_timezone_inner() -> Result<Timezone, GetTimezoneError> {
    let fd = openat(
        cwd(),
        PATH,
        OFlags::RDONLY | OFlags::NOCTTY | OFlags::CLOEXEC,
        Mode::empty(),
    )?;

    let mut buf = [0u8; 64];
    let mut pos = 0;
    while pos < buf.len() {
        let amt = read(&fd, &mut buf[pos..])?;
        if amt == 0 {
            break;
        }
        pos += amt;
    }
    drop(fd);

    from_utf8(&buf[..pos])
        .ok()
        .and_then(|s| Timezone::new(s.trim_end()))
        .ok_or(GetTimezoneError::FailedParsingString)
}
