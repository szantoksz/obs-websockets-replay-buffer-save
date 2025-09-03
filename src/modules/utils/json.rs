use serde_json::{Value, from_str};

pub fn load(file_data: &str) -> Value {
    from_str(file_data).unwrap()
}
