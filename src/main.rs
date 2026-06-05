pub mod definitions;
mod loader;
mod message;

use crate::definitions::Message;
use crate::loader::{map_messages, read_msg_file};
use crate::message::{choose_message, construct_message, format_message};

fn main() {
    let file: String = read_msg_file();
    let messages: Vec<Message> = map_messages(&file);
    let choice: &Message = choose_message(&messages);
    let message: String = construct_message(choice);
    println!("{}", format_message(&message))
}
