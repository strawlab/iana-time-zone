#[cfg(target_os = "windows")]
fn main() {
    windows::build!(Windows::Globalization::Calendar);
}

#[cfg(not(target_os = "windows"))]
fn main() {}
