use std::time::SystemTime;
use time_format::{from_system_time_ms, strftime_ms_local};

pub fn print_log(text: &str) {
    let time_format_ms = from_system_time_ms(SystemTime::now()).unwrap();
    let timestamp = strftime_ms_local("%H:%M:%S:{ms}", time_format_ms).unwrap();

    println!("[{timestamp}] {text}");
}
