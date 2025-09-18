use crate::models::translation::{PromptType, TextUnit, TranslationStatus};
use serde::{Deserialize, Serialize};

/// Database representation of a text unit for translation
/// This is the persistent version of TextUnit with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextUnitRecord {
    pub id: Option<i64>, // None for new records, Some(id) for existing
    pub project_path: String,
    pub file_path: String,
    pub field_type: String,
    pub source_text: String,
    pub translated_text: Option<String>,
    pub status: String,      // Stored as string in DB, converted to enum
    pub prompt_type: String, // Stored as string in DB, converted to enum
    pub source_lang: String,
    pub target_lang: String,
    pub manifest_hash: Option<String>, // Links to .ludolingua.json
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl TextUnitRecord {
    /// Create a new TextUnitRecord from a TextUnit with project context
    pub fn from_text_unit(
        text_unit: &TextUnit,
        project_path: &str,
        file_path: &str,
        manifest_hash: Option<&str>,
    ) -> Self {
        Self {
            id: None, // New record
            project_path: project_path.to_string(),
            file_path: file_path.to_string(),
            field_type: text_unit.field_type.clone(),
            source_text: text_unit.source_text.clone(),
            translated_text: if text_unit.translated_text.is_empty() {
                None
            } else {
                Some(text_unit.translated_text.clone())
            },
            status: Self::status_to_string(text_unit.status),
            prompt_type: Self::prompt_type_to_string(text_unit.prompt_type),
            source_lang: "ja".to_string(), // TODO: Get from engine info
            target_lang: "en".to_string(), // TODO: Get from engine info
            manifest_hash: manifest_hash.map(|s| s.to_string()),
            created_at: None,
            updated_at: None,
        }
    }

    /// Convert TextUnitRecord back to TextUnit for frontend use
    pub fn to_text_unit(&self) -> TextUnit {
        TextUnit {
            id: self.id.map(|id| id.to_string()).unwrap_or_else(|| {
                format!(
                    "{}_{}_{}",
                    self.project_path, self.file_path, self.field_type
                )
            }),
            source_text: self.source_text.clone(),
            translated_text: self.translated_text.clone().unwrap_or_default(),
            field_type: self.field_type.clone(),
            status: Self::string_to_status(&self.status),
            prompt_type: Self::string_to_prompt_type(&self.prompt_type),
        }
    }

    /// Update existing record with new translation data
    pub fn update_from_text_unit(&mut self, text_unit: &TextUnit) {
        self.translated_text = if text_unit.translated_text.is_empty() {
            None
        } else {
            Some(text_unit.translated_text.clone())
        };
        self.status = Self::status_to_string(text_unit.status);
        self.prompt_type = Self::prompt_type_to_string(text_unit.prompt_type);
        // updated_at will be set by database trigger
    }

    fn status_to_string(status: TranslationStatus) -> String {
        match status {
            TranslationStatus::NotTranslated => "NotTranslated".to_string(),
            TranslationStatus::MachineTranslated => "MachineTranslated".to_string(),
            TranslationStatus::HumanReviewed => "HumanReviewed".to_string(),
            TranslationStatus::Ignored => "Ignored".to_string(),
        }
    }

    fn string_to_status(s: &str) -> TranslationStatus {
        match s {
            "MachineTranslated" => TranslationStatus::MachineTranslated,
            "HumanReviewed" => TranslationStatus::HumanReviewed,
            "Ignored" => TranslationStatus::Ignored,
            _ => TranslationStatus::NotTranslated,
        }
    }

    fn prompt_type_to_string(prompt_type: PromptType) -> String {
        match prompt_type {
            PromptType::Character => "Character".to_string(),
            PromptType::State => "State".to_string(),
            PromptType::System => "System".to_string(),
            PromptType::Dialogue => "Dialogue".to_string(),
            PromptType::Equipment => "Equipment".to_string(),
            PromptType::Skill => "Skill".to_string(),
            PromptType::Class => "Class".to_string(),
            PromptType::Other => "Other".to_string(),
        }
    }

    fn string_to_prompt_type(s: &str) -> PromptType {
        match s {
            "Character" => PromptType::Character,
            "State" => PromptType::State,
            "System" => PromptType::System,
            "Dialogue" => PromptType::Dialogue,
            "Equipment" => PromptType::Equipment,
            "Skill" => PromptType::Skill,
            "Class" => PromptType::Class,
            _ => PromptType::Other,
        }
    }
}

/// Query parameters for finding text units
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextUnitQuery {
    pub project_path: Option<String>,
    pub file_path: Option<String>,
    pub status: Option<String>,
    pub manifest_hash: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Result of bulk operations
#[derive(Debug, Clone)]
pub struct BulkOperationResult {
    pub inserted: i64,
    pub updated: i64,
    pub errors: Vec<String>,
}
