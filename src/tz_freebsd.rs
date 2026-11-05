// `tzsetup` writes the zone name to `/var/db/zoneinfo`. A system whose zone was
// configured another way has only the `/etc/localtime` link, which NetBSD and
// OpenBSD also use.
#[path = "tz_netbsd.rs"]
mod localtime;

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    // see https://gitlab.gnome.org/GNOME/evolution-data-server/-/issues/19
    match std::fs::read_to_string("/var/db/zoneinfo") {
        Ok(mut contents) => {
            // Trim to the correct length without allocating.
            contents.truncate(contents.trim_end().len());
            if !contents.is_empty() {
                return Ok(contents);
            }
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(err.into()),
    }
    localtime::get_timezone_inner()
}
