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

    #[cfg(feature = "util")]
    pub fn convert_timing(&mut self, target_resolution: i64) {
        if target_resolution <= 0 {
            return;
        }

        let factor = target_resolution as f64 / self.header.resolution as f64;

        self.header.resolution = target_resolution;

        for note in &mut self.notes {
            note.start = (note.start as f64 * factor) as i64;
            note.length = (note.length as f64 * factor) as i64;
        }
    }

    #[cfg(feature = "util")]
    pub fn relative_to(&mut self, start: Option<i64>) {
        let start = start.unwrap_or(0);
        
        if start < 0 {
            return;
        }

        let notes_start = self.notes.first().map_or(0, |n| n.start);
        
        for note in &mut self.notes {
            note.start -= notes_start - start;
            note.start = note.start.max(0);
        }
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