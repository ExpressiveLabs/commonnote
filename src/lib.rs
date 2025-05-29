use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier {
    pub identifier: String
}
impl Identifier {
    pub fn validate(data: &str) -> Result<(), String> {
        let ident: Self = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse commonnote identifier: {}", e))?;

        if ident.identifier.is_empty() {
            return Err("Data identifier is empty.".into());
        }

        if !ident.identifier.eq("commonnote") {
            return Err(format!("Invalid identifier: {}", ident.identifier).into());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    pub identifier: String,
    pub header: Header,
    pub notes: Vec<Note>,
    #[serde(default)]
    pub extra: serde_json::Value
}
impl Data {
    pub fn from_string(data: &str) -> Result<Self, String> {
        Identifier::validate(data)?; // Ensure that this is commonnote data

        let data: Self = serde_json::from_str(data)
            .map_err(|e| format!("Failed to parse commonnote data: {}", e))?;

        if data.notes.is_empty() {
            return Err("Data contains no notes.".into());
        }

        Ok(data)
    }
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