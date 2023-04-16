use std::path::PathBuf;
use dirs::{template_dir, home_dir};

/// Unwraps the default template folder if it exists, otherwise use the fallback format.
pub fn un_path() -> PathBuf {
    match template_dir() {
        None => fallback_dir(),
        Some(dir) => dir
    }
}

fn fallback_dir() -> PathBuf {
    let default_dir = home_dir().expect("Couldn't find the default template folder");
    default_dir.join("Templates")
}
