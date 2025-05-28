use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    pub header: Header,
    pub notes: Vec<Note>,
    #[serde(default)]
    pub extra: serde_json::Value
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    pub resolution: i64,
    pub language: Option<String>,
    pub origin: Option<String>,
    #[serde(default)]
    pub extra: serde_json::Value
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub start: i64,
    pub length: i64,
    pub pitch: u8,
    pub label: String,
    #[serde(default)]
    pub extra: serde_json::Value
}