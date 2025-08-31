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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlossaryQuery {
    pub source_lang: String,
    pub target_lang: String,
    pub categories: Vec<String>,
    pub prompt_types: Vec<String>,
    pub project_scope: Option<String>,
    pub limit: Option<usize>,
    pub only_enabled: bool,
}
