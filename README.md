# iana-time-zone - get the IANA time zone for the current system

[![Crates.io](https://img.shields.io/crates/v/iana-time-zone.svg)](https://crates.io/crates/iana-time-zone)
[![Documentation](https://docs.rs/iana-time-zone/badge.svg)](https://docs.rs/iana-time-zone/)
[![Crate License](https://img.shields.io/crates/l/iana-time-zone.svg)](https://crates.io/crates/iana-time-zone)
[![build](https://github.com/strawlab/iana-time-zone/workflows/build/badge.svg?branch=master)](https://github.com/strawlab/iana-time-zone/actions?query=branch%3Amaster)

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

## Minimum supported rust version policy

This crate has a minimum supported rust version (MSRV) of 1.59. Updates to the
MSRV are sometimes necessary due to the MSRV of dependencies. MSRV updates will
not be indicated as a breaking change to the semver version.
