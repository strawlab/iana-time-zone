pub(crate) fn get_timezone_inner() -> Result<String,crate::GetTimezoneError> {
    // see https://stackoverflow.com/a/12523283
    use std::io::Read;

    let fname = "/etc/timezone";
    let mut f = std::fs::File::open(&fname)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}
