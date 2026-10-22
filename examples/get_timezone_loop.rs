use std::thread;
use std::time::Duration;

use iana_time_zone::{GetTimezoneError, get_timezone};

const WAIT: Duration = Duration::from_secs(1);

fn main() -> Result<(), GetTimezoneError> {
    loop {
        println!("{}", get_timezone()?);
        thread::sleep(WAIT);
    }
}
