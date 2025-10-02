/// Raw text unit for file I/O operations
///
/// This struct represents text units as they exist in game files,
/// before any text processing is applied.
#[derive(Debug, Clone)]
pub struct RawTextUnit {
    pub id: String,
    pub source_text: String,
    pub field_type: String,
    pub prompt_type: crate::models::translation::PromptType,
}
