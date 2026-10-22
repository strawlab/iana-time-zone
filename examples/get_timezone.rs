use iana_time_zone::{GetTimezoneError, get_timezone};

fn main() -> Result<(), GetTimezoneError> {
    println!("{}", get_timezone()?);
    Ok(())
}
