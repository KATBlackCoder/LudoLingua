use serde::{Deserialize, Serialize};
use crate::models::language::Language;

/// Represents the status of a text unit's translation process.
///
/// This enum tracks the current state of a translation, from not translated
/// through machine translation to human review, or marked as ignored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TranslationStatus {
    /// Text that has not yet been translated
    NotTranslated,

    /// Text that has been translated by AI but not yet reviewed by a human
    MachineTranslated,

    /// Text that has been translated and reviewed/edited by a human
    HumanReviewed,

    /// Text that has been explicitly marked as not requiring translation
    Ignored,
}

/// Defines the type of prompt template to use for different translation contexts.
///
/// Different text types (names, descriptions, etc.) may require different
/// translation approaches or prompt templates to get optimal results from the AI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PromptType {
    /// Prompt specialized for translating character or location names
    Name,

    /// Prompt specialized for translating longer descriptive text
    Description,

    /// Prompt specialized for translating dialogue
    Dialogue,

    /// Prompt specialized for translating item names and descriptions
    Item,

    /// Prompt specialized for translating skill names and descriptions
    Skill,

    /// Prompt specialized for translating other text types
    Other,
}

/// Represents a single unit of text that can be translated.
///
/// A TextUnit is the core data structure for managing translations. It contains
/// both the original source text and its translation (if any), along with metadata
/// about the translation status and context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextUnit {
    /// Unique identifier for the text field within its file
    pub id: String,

    /// The original text in the source language
    pub source_text: String,

    /// The translated text in the target language (empty if not yet translated)
    pub translated_text: String,

    /// The type of field being translated (e.g., "name", "description", "message")
    /// This helps provide context for the translation
    pub field_type: String,

    /// Current status of the translation process for this text unit
    pub status: TranslationStatus,

    /// The type of prompt template that should be used when translating this text
    pub prompt_type: PromptType,
}

/// Represents the current source language and target language for translation operations.
///
/// A TranslationLanguages struct contains the language pair used for translation,
/// defining what language to translate from and what language to translate to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationLanguages {
    /// The text to translate
    pub text: String,

    /// The current source language (language to translate from)
    pub current_language: Language,

    /// The target language (language to translate to)
    pub target_language: Language,

    /// The type of prompt template that should be used when translating this text
    pub prompt_type: PromptType,
}
