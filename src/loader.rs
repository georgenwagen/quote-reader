use crate::definitions::Message;
use std::{fs, path::Path};

// Read data
fn read_msg_file(path: &Path) -> String {
    fs::read_to_string(path).expect("Should've been able to read the file")
}

// Vectorize messages
fn map_messages(json: &str) -> Vec<Message> {
    serde_json::from_str(json).unwrap()
}

// Main
pub fn load_messages(path: &Path) -> Vec<Message> {
    let message_file: String = read_msg_file(path);
    map_messages(&message_file)
}
