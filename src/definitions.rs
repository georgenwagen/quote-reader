use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Segment {
    #[serde(rename = "static")]
    Static { text: String },
    #[serde(rename = "repeating")]
    Repeating { text: String, delimiter: String },
}

#[derive(Deserialize)]
pub struct Message {
    pub segments: Vec<Segment>,
}
