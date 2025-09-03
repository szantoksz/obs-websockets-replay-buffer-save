use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub fn create(path: &PathBuf) -> File {
    File::create(path).unwrap()
}

pub fn has_data(path: &PathBuf) -> bool {
    let filesize = fs::metadata(path).unwrap().len();
    if filesize == 0 { false } else { true }
}

pub fn write_with_path(path: &PathBuf, data: &str) {
    let mut file = OpenOptions::new().write(true).open(path).unwrap();
    writeln!(file, "{}", data).unwrap();
}

pub fn write_with_file(mut file: &File, data: &str) {
    writeln!(file, "{}", data).unwrap();
}

pub fn read(path: &PathBuf) -> String {
    // trim_end so it doesnt leave an extra newline character
    fs::read_to_string(path).unwrap().trim_end().to_string()
}
