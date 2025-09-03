use std::io;
use std::process;

pub fn get_default_config() -> String {
    // trim_end so it doesnt leave an extra newline character
    include_str!("../../assets/default_config.json")
        .trim_end()
        .to_string()
}

pub fn quit() {
    process::exit(0);
}

pub fn safe_quit() {
    println!("Press Enter to quit...");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).unwrap();
    drop(_input);
    quit();
}
