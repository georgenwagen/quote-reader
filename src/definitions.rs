use serde::{Deserialize, Serialize};
use time::UtcDateTime;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Segment {
    #[serde(rename = "static")]
    Static { text: String },
    #[serde(rename = "repeating")]
    Repeating {
        text: String,
        delimiter: Option<String>,
    },
}

#[derive(Serialize, Deserialize)]
pub enum Metadata {
    #[serde(rename = "time_created")]
    Created { datetime: UtcDateTime, user: String },

    #[serde(rename = "time_modified")]
    Modified { datetime: UtcDateTime, user: String },
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub segments: Vec<Segment>,
    pub metadata: Option<Metadata>,
}
