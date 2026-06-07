pub mod definitions;
mod loader;
mod message;

use crate::definitions::Message;
use crate::loader::load_messages;
use crate::message::{
    choose_message, construct_from_message, format_message, serialize_new_message,
};
use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

#[derive(ValueEnum, Clone)]
enum Mode {
    Quote,
    Add,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // File to read from / write to
    #[arg(default_value = "messages.json")]
    file: PathBuf,

    // Mode
    #[arg(short, long, value_enum)]
    mode: Mode,

    // Choose to read message at specific index
    #[arg(short, long, default_value = None)]
    index: Option<usize>,

    // Text to add for 'Add' mode
    #[arg(long, default_value = None)]
    text: Option<String>,
}

fn main() {
    let args = Args::parse();
    let file_path: &Path = &args.file;

    match &args.mode {
        Mode::Quote => {
            let messages: Vec<Message> = load_messages(file_path);
            let choice: &Message = choose_message(&messages, args.index);
            let message: String = construct_from_message(choice);
            println!("{}", format_message(&message))
        }
        Mode::Add => {
            let message_text: String = args
                .text
                .unwrap_or_else(|| panic!("Use --text to sepcify message to be added!"));
            let mut messages: Vec<Message> = load_messages(file_path);
            let new_message: Message = serialize_new_message(&message_text);
            messages.push(new_message);
            println!(
                "Message: {}\nPushed to index: {}",
                &message_text,
                messages.len() - 1
            );
            let json: String = serde_json::to_string_pretty(&messages).unwrap();
            std::fs::write(file_path, json).unwrap();
        }
    }
}
