pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    // see https://stackoverflow.com/a/12523283
    use std::io::Read;

    let fname = "/etc/timezone";
    let mut f = std::fs::File::open(&fname)?;
    let mut contents = String::with_capacity(32);
    f.read_to_string(&mut contents)?;
    contents.truncate(contents.trim_end().len());
    Ok(contents)
}
