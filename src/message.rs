use crate::definitions::{Message, Segment};
use rand::seq::IndexedRandom;

// Pick random message
pub fn choose_message(entries: &[Message]) -> &Message {
    entries.choose(&mut rand::rng()).unwrap()
}

pub fn repeating_string(part: &str, delimiter: Option<&str>) -> String {
    // Generate random number in the range [0, 99]
    let num: i32 = rand::random_range(0..100);

    // Repeated string
    let mut repeated: String = String::from(part);
    let delimiter: &str = delimiter.unwrap_or(" ");

    //loop
    for _ in 0..num {
        repeated += delimiter;
        repeated += part;
    }
    repeated
}

// Make message
pub fn construct_message(message: &Message) -> String {
    let mut result: String = String::new();
    for segment in &message.segments {
        let add: &String = match segment {
            Segment::Static { text } => text,
            Segment::Repeating { text, delimiter } => &repeating_string(text, Some(delimiter)),
        };
        result.push_str(add);
    }
    result
}

pub fn format_message(text: &str) -> String {
    text[0..1].to_uppercase() + &text[1..]
}
