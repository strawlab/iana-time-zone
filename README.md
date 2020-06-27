# iana-time-zone - get the IANA time zone for the current system

This small utility crate gets get the IANA time zone for the current system.
This is also known the [tz database](https://en.wikipedia.org/wiki/Tz_database),
tzdata, the zoneinfo database and, the Olson database.

Example:

```
extern crate iana_time_zone;
println!("current: {}", iana_time_zone::get_timezone().unwrap());
```

You can test this is working on your platform with:

```
cargo test -- --nocapture
```
