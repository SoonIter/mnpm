fn get_dep_symlink_path(name: &String, version: &Version) -> PathBuf {
    let folder_name = if name.starts_with("@") {
        name.replace("/", "+")
    } else {
        name.to_string()
    };

    Path::new("..")
        .join("..")
        .join(format!("{}@{}", folder_name, version))
        .join(DEPS_FOLDER)
        .join(name)
}

pub async fn symlink_dep(
    dep_name: &String,
    dep_version: &Version,
    dest_name: &String,
    dest_version: &Version,
) -> anyhow::Result<()> {
    let original = get_dep_symlink_path(&dep_name, dep_version);

    let link = get_local_store_package_path(dest_name, dest_version);
    let mut parent = link
        .parent()
        .expect("failed to get package folder")
        .to_path_buf();
    if dest_name.starts_with("@") {
        parent = parent
            .parent()
            .expect("failed to get package folder")
            .to_path_buf();
    }

    parent = parent.join(&dep_name);

    let name = dep_name.clone();

    task::spawn_blocking(
        move || match std::os::unix::fs::symlink(&original, &parent) {
            Err(error) if error.kind() == ErrorKind::AlreadyExists => Ok(()),
            Err(error) => {
                // TODO: check what pnpm does with @floating-ui/react-dom
                // I bet they convert the / to something else like _

                println!("{:?}, {:?}, {:?}, {:?}", error, original, parent, name);

                return Err(error.into());
            }
            Ok(_) => Ok(()),
        },
    )
    .await?
}
