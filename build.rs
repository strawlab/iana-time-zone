#[cfg(target_os = "windows")]
fn main() {
    windows::build!(
        windows::globalization::Calendar
    );
}

#[cfg(not(target_os = "windows"))]
fn main() {
}
