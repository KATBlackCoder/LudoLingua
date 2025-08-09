use log::error;
use std::fs;
use std::path::Path;

use crate::core::error::{AppError, AppResult};
use crate::models::engine::EngineInfo;
use crate::models::translation::{PromptType, TextUnit};

/// Shared prompt builder utility for all LLM providers.
///
/// This module provides reusable prompt building functionality that can be used
/// by any LLM provider implementation, ensuring consistency and reducing code duplication.
pub struct PromptBuilder;

impl PromptBuilder {
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
        let prompt_without_text = Self::replace_template_variables(&template, text_unit, engine_info);

        // Append raw text at the very end to avoid any trailing sections overriding or splitting content
        let mut final_prompt = prompt_without_text;
        final_prompt.push_str("\n\n");
        final_prompt.push_str(&text_unit.source_text);
        final_prompt
    }

    /// Filter the shared vocabulary to only include sections relevant to the prompt type.
    fn filter_vocabulary_sections(vocab: &str, prompt_type: PromptType) -> String {
        let wanted_sections: &[&str] = match prompt_type {
            PromptType::Dialogue | PromptType::Character => &[
                "### Characters",
                "### Essential Terms",
            ],
            PromptType::State | PromptType::Skill => &[
                "### Status Effects",
                "### Mechanics",
                "### Essential Terms",
            ],
            PromptType::Equipment => &[
                "### Mechanics",
                "### Essential Terms",
            ],
            PromptType::System | PromptType::Class | PromptType::Other => &[
                "### Mechanics",
                "### Essential Terms",
            ],
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
        text_unit: &TextUnit,
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
            .replace("{context}", "RPG Maker game content")
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
        let path = Path::new(template_path);
        fs::read_to_string(path).map_err(|e| {
            AppError::FileSystem(format!(
                "Failed to read prompt template {}: {}",
                template_path, e
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

        // Keep delimiters in fallback for safety
        prompt.push_str(&format!(
            "<<TEXT_START>>\n{}\n<<TEXT_END>>\n\n",
            text_unit.source_text
        ));
        prompt
    }
}
