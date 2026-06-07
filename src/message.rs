use crate::definitions::{Message, Metadata, Segment};
use rand::seq::IndexedRandom;
use time::UtcDateTime;

// Pick random message
pub fn choose_message(entries: &[Message], index: Option<usize>) -> &Message {
    match index {
        Some(i) => entries
            .get(i)
            .unwrap_or_else(|| panic!("Err: index {} out of bounds", i)),
        None => entries.choose(&mut rand::rng()).unwrap(),
    }
}

fn repeating_string(part: &str, delimiter: Option<&str>) -> String {
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
pub fn construct_from_message(message: &Message) -> String {
    let mut result: String = String::new();
    for segment in &message.segments {
        let add: &String = match segment {
            Segment::Static { text } => text,
            Segment::Repeating { text, delimiter } => &repeating_string(text, delimiter.as_deref()),
        };
        result.push_str(add);
    }
    result
}

pub fn format_message(text: &str) -> String {
    text[0..1].to_uppercase() + &text[1..]
}

// Create message
pub fn serialize_new_message(text: &str) -> Message {
    let host = hostname::get().unwrap().to_string_lossy().to_string();
    Message {
        segments: vec![Segment::Static {
            text: text.to_owned(),
        }],
        metadata: Some(Metadata::Created {
            datetime: UtcDateTime::now(),
            user: host,
        }),
    }
}
