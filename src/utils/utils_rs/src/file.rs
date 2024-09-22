pub fn kinda_exists(path: &std::path::Path) -> bool {
    if path.exists() {
        return true;
    } else if path.is_symlink() {
        return true;
    }
    false
}

pub fn archive(source: &str) -> Option<std::path::PathBuf> {
    let source = std::fs::canonicalize(source).ok()?;

    if !kinda_exists(&source) {
        return None;
    }

    let source_dir = source.parent().unwrap();
    let source_name = source.file_name().unwrap();
    let target_dir = source_dir.join("__archived__").join(crate::strings::get_timeslug());
    let target = target_dir.join(source_name);

    std::fs::create_dir_all(&target_dir).ok()?;
    std::fs::rename(&source, &target).ok()?;

    Some(target)
}
