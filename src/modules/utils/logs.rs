use chrono::Local;

pub fn print_log(text: &str) {
    // get machine time
    let time_now = Local::now();
    let timestamp = time_now.format("%H:%M:%S:%3f");

    println!("[{timestamp}] {text}");
}
