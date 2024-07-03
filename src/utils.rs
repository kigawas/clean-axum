use std::process::Command;

pub fn touch(file_name: &str) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &format!("copy nul {}", file_name)])
            .output()
            .expect("failed to execute touch");
    } else {
        Command::new("touch")
            .arg(file_name)
            .output()
            .expect("failed to execute touch");
    }
}

pub fn create_dev_db(db_url: &str) {
    let prefix = "sqlite://";
    if let Some(file_name) = db_url.strip_prefix(prefix) {
        touch(file_name);
    }
}
