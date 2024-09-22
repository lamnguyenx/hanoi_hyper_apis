pub fn get_timeslug() -> String {
    let slug_fmt = "%Y.%m.%d__%Hh%Mm%Ss.%f";
    return chrono::Utc::now().format(&slug_fmt).to_string();
}

pub fn get_timedir_hourly() -> std::path::PathBuf {
    let path_fmt = format!("%Y{0}%m{0}%d{0}%H", std::path::MAIN_SEPARATOR);
    let path_str = chrono::Utc::now().format(&path_fmt).to_string();
    return std::path::PathBuf::from(path_str);
}
