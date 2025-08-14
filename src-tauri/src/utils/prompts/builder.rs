use log::{debug, error};
use std::fs;
use std::path::PathBuf;

use crate::core::error::{AppError, AppResult};
use crate::models::engine::EngineInfo;
use crate::models::translation::{PromptType, TextUnit};

/// Shared prompt builder utility for all LLM providers.
///
/// This module provides reusable prompt building functionality that can be used
/// by any LLM provider implementation, ensuring consistency and reducing code duplication.
pub struct PromptBuilder;

impl PromptBuilder {
    /// Render glossary terms (DB) as a block compatible with `vocabularies.txt` sections.
    pub fn render_glossary_terms(terms: &[crate::glossaries::model::GlossaryTerm]) -> String {
        use std::collections::BTreeMap;
        if terms.is_empty() {
            return String::new();
        }
        let mut by_cat: BTreeMap<&str, Vec<&crate::glossaries::model::GlossaryTerm>> =
            BTreeMap::new();
        for t in terms {
            by_cat.entry(&t.category).or_default().push(t);
        }
        let mut s = String::new();
        for (cat, group) in by_cat {
            s.push_str(&format!("### {}\n", cat));
            for t in group {
                s.push_str("Input: ");
                s.push_str(&t.input);
                s.push('\n');
                s.push_str("Output: ");
                s.push_str(&t.output);
                s.push('\n');
                s.push('\n');
            }
        }
        s
    }

    /// Build a translation prompt using a provided set of glossary terms.
    pub async fn build_translation_prompt_with_terms(
        text_unit: &TextUnit,
        engine_info: &EngineInfo,
        terms: &[crate::glossaries::model::GlossaryTerm],
    ) -> String {
        debug!(
            "PromptBuilder: using DB glossary terms ({} terms) for prompt_type {:?}",
            terms.len(),
            text_unit.prompt_type
        );
        let basic_template = match Self::load_prompt_template("prompts/basic.txt") {
            Ok(template) => template,
            Err(_e) => return Self::build_fallback_prompt(text_unit, engine_info),
        };

        // DB-only mode: use DB-rendered terms only (skip file vocabulary)
        let db_vocab_src = Self::render_glossary_terms(terms);
        let db_len = db_vocab_src.len();
        let vocabulary_template =
            Self::filter_vocabulary_sections(&db_vocab_src, text_unit.prompt_type);
        debug!(
            "PromptBuilder: DB-only vocabulary length: db={} bytes, filtered={} bytes",
            db_len,
            vocabulary_template.len()
        );

        // Specific template
        let specific_template = text_unit.prompt_type.template_path();
        let specific_content = Self::load_prompt_template(specific_template).unwrap_or_default();

        // Compose
        let mut template = basic_template;
        template.push_str("\n\n");
        template.push_str(&vocabulary_template);
        template.push_str("\n\n");
        template.push_str(&specific_content);

        let mut final_prompt = Self::replace_template_variables(&template, text_unit, engine_info);
        final_prompt.push_str("\n\n<<<INPUT_START>>>\n");
        final_prompt.push_str(&text_unit.source_text);
        final_prompt.push_str("\n<<<INPUT_END>>>\n");
        final_prompt
    }
    /// Build a translation prompt based on the text unit and engine info.
    ///
    /// This function creates a comprehensive prompt by combining basic, vocabulary, and specific templates,
    /// and replacing template variables with actual content.
    ///
    /// # Arguments
    ///
    /// * `text_unit` - The text unit to translate
    /// * `engine_info` - Information about the game engine and languages
    ///
    /// # Returns
    ///
    /// * `String` - The complete translation prompt ready for LLM processing
    pub async fn build_translation_prompt(
        text_unit: &TextUnit,
        engine_info: &EngineInfo,
    ) -> String {
        debug!(
            "PromptBuilder: no DB glossary provided; using file vocabulary only for prompt_type {:?}",
            text_unit.prompt_type
        );
        // Load basic template
        let basic_template = match Self::load_prompt_template("prompts/basic.txt") {
            Ok(template) => template,
            Err(e) => {
                error!("Failed to load basic template: {}", e);
                return Self::build_fallback_prompt(text_unit, engine_info);
            }
        };

        // Load vocabulary template and filter by prompt type to reduce token usage
        let vocabulary_template = match Self::load_prompt_template("prompts/vocabularies.txt") {
            Ok(template) => Self::filter_vocabulary_sections(&template, text_unit.prompt_type),
            Err(e) => {
                error!("Failed to load vocabulary template: {}", e);
                String::new()
            }
        };

        // Load specific template based on prompt type
        let specific_template = text_unit.prompt_type.template_path();

        let specific_content = match Self::load_prompt_template(specific_template) {
            Ok(content) => content,
            Err(e) => {
                error!(
                    "Failed to load specific template {}: {}",
                    specific_template, e
                );
                String::new()
            }
        };

        // Combine templates: basic + vocabulary + specific
        let mut template = basic_template;
        template.push_str("\n\n");
        template.push_str(&vocabulary_template);
        template.push_str("\n\n");
        template.push_str(&specific_content);

        // Replace variables first
        let prompt_without_text =
            Self::replace_template_variables(&template, text_unit, engine_info);

        // Append raw text with explicit delimiters to prevent bleed
        let mut final_prompt = prompt_without_text;
        final_prompt.push_str("\n\n<<<INPUT_START>>>\n");
        final_prompt.push_str(&text_unit.source_text);
        final_prompt.push_str("\n<<<INPUT_END>>>\n");
        final_prompt
    }

    /// Filter the shared vocabulary to only include sections relevant to the prompt type.
    fn filter_vocabulary_sections(vocab: &str, prompt_type: PromptType) -> String {
        let wanted_sections: &[&str] = match prompt_type {
            PromptType::Dialogue => &[
                "### Characters",
                "### Essential Terms",
                "### Translation Rules",
                "### Locations",
                "### Time & Weather",
                "### Mechanics",
            ],
            PromptType::Character => &["### Characters", "### Essential Terms"],
            PromptType::State | PromptType::Skill => {
                &["### Status Effects", "### Mechanics", "### Essential Terms"]
            }
            PromptType::Equipment => &["### Mechanics", "### Essential Terms"],
            PromptType::System | PromptType::Class | PromptType::Other => {
                &["### Mechanics", "### Essential Terms"]
            }
        };

        let mut output = String::new();
        let mut keep = false;
        for line in vocab.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("### ") {
                keep = wanted_sections.iter().any(|h| *h == trimmed);
            }
            if keep {
                output.push_str(line);
                output.push('\n');
            }
        }
        output
    }

    /// Replace template variables with actual content.
    ///
    /// # Arguments
    ///
    /// * `template` - The template string with variables
    /// * `text_unit` - The text unit being translated
    /// * `engine_info` - Engine information
    ///
    /// # Returns
    ///
    /// * `String` - The template with all variables replaced
    fn replace_template_variables(
        template: &str,
        _text_unit: &TextUnit,
        engine_info: &EngineInfo,
    ) -> String {
        template
            .replace(
                "{source_language}",
                &engine_info.source_language.native_name,
            )
            .replace(
                "{target_language}",
                &engine_info.target_language.native_name,
            )
    }

    /// Load a prompt template from the filesystem.
    ///
    /// # Arguments
    ///
    /// * `template_path` - Path to the template file
    ///
    /// # Returns
    ///
    /// * `AppResult<String>` - The template content or an error
    #[cfg(debug_assertions)]
    fn load_prompt_template(template_path: &str) -> AppResult<String> {
        // Resolve relative template paths (e.g., "prompts/basic.txt") from the
        // crate root to avoid dependency on the current working directory.
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let full_path = base.join(template_path);
        fs::read_to_string(&full_path).map_err(|e| {
            AppError::FileSystem(format!(
                "Failed to read prompt template {}: {}",
                full_path.display(),
                e
            ))
        })
    }

    /// In production builds, embed prompts at compile time to avoid filesystem path issues.
    #[cfg(not(debug_assertions))]
    fn load_prompt_template(template_path: &str) -> AppResult<String> {
        let content: &'static str = match template_path {
            "prompts/basic.txt" => include_str!("../../../prompts/basic.txt"),
            "prompts/vocabularies.txt" => include_str!("../../../prompts/vocabularies.txt"),
            "prompts/character.txt" => include_str!("../../../prompts/character.txt"),
            "prompts/state.txt" => include_str!("../../../prompts/state.txt"),
            "prompts/dialogue.txt" => include_str!("../../../prompts/dialogue.txt"),
            "prompts/equipment.txt" => include_str!("../../../prompts/equipment.txt"),
            "prompts/skill.txt" => include_str!("../../../prompts/skill.txt"),
            "prompts/class.txt" => include_str!("../../../prompts/class.txt"),
            "prompts/system.txt" => include_str!("../../../prompts/system.txt"),
            "prompts/other.txt" => include_str!("../../../prompts/other.txt"),
            _ => {
                return Err(AppError::FileSystem(format!(
                    "Unknown prompt template path: {}",
                    template_path
                )))
            }
        };
        Ok(content.to_string())
    }

    /// Build a fallback prompt when template loading fails.
    ///
    /// This provides a basic but functional prompt when the template files
    /// are unavailable or corrupted.
    ///
    /// # Arguments
    ///
    /// * `text_unit` - The text unit to translate
    /// * `engine_info` - Engine information
    ///
    /// # Returns
    ///
    /// * `String` - A basic fallback prompt
    fn build_fallback_prompt(text_unit: &TextUnit, engine_info: &EngineInfo) -> String {
        let mut prompt = String::from(
            "You are a professional translator specializing in game localization.\n\n",
        );

        prompt.push_str(&format!(
            "Translate the following text from {} to {}:\n\n",
            engine_info.source_language.native_name, engine_info.target_language.native_name
        ));

        // Use explicit delimiters in fallback as well
        prompt.push_str("<<<INPUT_START>>>\n");
        prompt.push_str(&text_unit.source_text);
        prompt.push_str("\n<<<INPUT_END>>>\n");
        prompt
    }
}
