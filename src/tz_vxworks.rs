use std::io::{Error, ErrorKind};
use vxal::timezone::{get_default_timezone, get_filepaths};
use std::fs::read_to_string;

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    read_timezone()
        .or_else(|_| Ok(get_default_timezone().to_string()))
}

fn read_timezone() -> Result<String, crate::GetTimezoneError> {
    for path in get_filepaths() {
        let contents = read_to_string(path);
        if contents.is_ok() {
            let mut contents = contents.unwrap();

            // Trim to the correct length without allocating.
            contents.truncate(contents.trim_end().len());

            // Skip blank file
            if contents.len() < 1 {
                continue;
            }

            return Ok(contents);
        }
    }

    Err(crate::GetTimezoneError::IoError(Error::from(ErrorKind::NotFound)))
}
