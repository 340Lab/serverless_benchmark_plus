use std::path::PathBuf;

pub fn app_dir(app: &str) -> PathBuf {
    PathBuf::from("prepare_data").join(app)
}
