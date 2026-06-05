use crate::definitions::Message;
use std::fs;

// Read data
pub fn read_msg_file() -> String {
    fs::read_to_string("messages.json").expect("Should've been able to read the file")
}

// Initialize data
pub fn map_messages(json: &str) -> Vec<Message> {
    serde_json::from_str(json).unwrap()
}
