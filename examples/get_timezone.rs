use iana_time_zone::{GetTimezoneError, Timezone};

fn main() -> Result<(), GetTimezoneError> {
    println!("{}", Timezone::system()?);
    Ok(())
}
