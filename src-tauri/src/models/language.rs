use serde::{Deserialize, Serialize};

/// Represents a language used in translation operations.
///
/// This struct contains basic information about a language, including its ISO code,
/// display name, and native name. It can be used as the source or target language
/// for translation tasks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Language {
    /// ISO 639-1 language code (e.g., "en", "ja", "fr")
    pub id: String,
    /// Display name of the language (e.g., "English", "Japanese", "French")
    pub label: String,
    /// Native name of the language (e.g., "English", "日本語", "Français")
    pub native_name: String,
    /// Text direction: "ltr" or "rtl"
    pub dir: String,
    /// Whether this language should be exposed in the UI/catalog
    pub enabled: bool,
}
