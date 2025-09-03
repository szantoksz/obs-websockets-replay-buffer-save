use std::env::current_dir;
use std::path::PathBuf;

pub fn get_exec_dir_path() -> PathBuf {
    current_dir().unwrap()
}
