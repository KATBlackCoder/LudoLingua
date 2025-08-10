#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlossaryTerm {
    pub id: i64,
    pub category: String,
    pub source_lang: String,
    pub target_lang: String,
    pub input: String,
    pub output: String,
    pub enabled: bool,
}


