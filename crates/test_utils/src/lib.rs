pub fn get_fixtures_root() -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("../../../tests/fixtures/basic");
    root.clean()
}

#[test]
fn get_fixtures_root() {
    assert_eq!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn you_can_assert_eq() {
        assert_eq!();
    }
}
