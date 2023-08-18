use windows_bindgen::{bindgen, Result};

fn main() -> Result<()> {
    let log = bindgen(["--etc", "bindings.txt"])?;
    println!("{}", log);
    Ok(())
}
