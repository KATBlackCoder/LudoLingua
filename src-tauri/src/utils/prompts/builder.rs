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
    /// This function creates a comprehensive prompt by combining basic and specific templates,
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

        // Load specific template based on prompt type
        let specific_template = Self::get_specific_template_path(text_unit.prompt_type);

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

        // Combine templates
        let mut template = basic_template;
        template.push_str("\n\n");
        template.push_str(&specific_content);

        // Replace template variables
        Self::replace_template_variables(&template, text_unit, engine_info)
    }

    /// Get the path to the specific template based on prompt type.
    ///
    /// # Arguments
    ///
    /// * `prompt_type` - The type of prompt to get the template for
    ///
    /// # Returns
    ///
    /// * `&'static str` - The path to the specific template file
    fn get_specific_template_path(prompt_type: PromptType) -> &'static str {
        match prompt_type {
            PromptType::Character => "prompts/character.txt",
            PromptType::Description => "prompts/description.txt",
            PromptType::Dialogue => "prompts/dialogue.txt",
            PromptType::Item => "prompts/item.txt",
            PromptType::Skill => "prompts/skill.txt",
            PromptType::Other => "prompts/other.txt",
        }
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
            .replace("{text}", &text_unit.source_text)
            .replace("{field_type}", &text_unit.field_type)
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
    fn load_prompt_template(template_path: &str) -> AppResult<String> {
        let path = Path::new(template_path);
        fs::read_to_string(path).map_err(|e| {
            AppError::FileSystem(format!(
                "Failed to read prompt template {}: {}",
                template_path, e
            ))
        })
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

        prompt.push_str(&format!(
            "Text to translate: {}\n\nTranslation:",
            text_unit.source_text
        ));
        prompt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::language::Language;

    #[test]
    fn test_get_specific_template_path() {
        assert_eq!(
            PromptBuilder::get_specific_template_path(PromptType::Character),
            "prompts/character.txt"
        );
        assert_eq!(
            PromptBuilder::get_specific_template_path(PromptType::Description),
            "prompts/description.txt"
        );
        assert_eq!(
            PromptBuilder::get_specific_template_path(PromptType::Dialogue),
            "prompts/dialogue.txt"
        );
    }

    #[test]
    fn test_replace_template_variables() {
        let template = "Translate from {source_language} to {target_language}: {text}";
        let text_unit = TextUnit {
            id: "test".to_string(),
            source_text: "Hello world".to_string(),
            translated_text: String::new(),
            field_type: "test".to_string(),
            status: crate::models::translation::TranslationStatus::NotTranslated,
            prompt_type: PromptType::Other,
        };
        let engine_info = EngineInfo {
            name: "Test".to_string(),
            path: std::path::PathBuf::new(),
            engine_type: crate::models::engine::EngineType::RpgMakerMv,
            source_language: Language {
                id: "en".to_string(),
                label: "English".to_string(),
                native_name: "English".to_string(),
            },
            target_language: Language {
                id: "fr".to_string(),
                label: "French".to_string(),
                native_name: "Français".to_string(),
            },
            version: None,
            detection_criteria: crate::models::engine::EngineCriteria {
                required_files: vec![],
                required_folders: vec![],
                optional_files: None,
            },
        };

        let result = PromptBuilder::replace_template_variables(template, &text_unit, &engine_info);

        assert_eq!(result, "Translate from English to Français: Hello world");
    }
}
