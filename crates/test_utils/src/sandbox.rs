pub fn create_sandbox<T: AsRef<str>>(fixture: T) -> Sandbox {
    let temp_dir = create_temp_dir();

    temp_dir
        .copy_from(get_fixtures_path(fixture), &["**/*"])
        .unwrap();

    Sandbox {
        // command: None,
        fixture: temp_dir,
    }
}
