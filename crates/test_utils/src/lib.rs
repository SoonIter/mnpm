use std::path::PathBuf;
use clean_path::{clean, Clean};


pub fn get_fixtures_root() -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("../../../tests/fixtures/basic");
    root.clean()
}

pub fn get_fixtures_path<T: AsRef<str>>(name: T) -> PathBuf {
    let path = get_fixtures_root().join(name.as_ref());

    if !path.exists() {
        panic!(
            "{}",
            format!("Fixture {} does no exist.", path.to_string_lossy())
        );
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_fix() {

    }
}
