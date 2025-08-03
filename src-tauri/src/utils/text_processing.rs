use regex::Regex;

/// Replace RPG Maker formatting codes with placeholder names for translation
/// 
/// # Arguments
/// * `text` - The text containing RPG Maker formatting codes
/// 
/// # Returns
/// * `String` - Text with formatting codes replaced by placeholders
pub fn replace_formatting_codes_for_translation(text: &str) -> String {
    let mut result = text.to_string();
    
    // Replace color codes: \C[n] -> [COLOR_n]
    let color_regex = Regex::new(r"\\C\[(\d+)\]").unwrap();
    result = color_regex.replace_all(&result, "[COLOR_$1]").to_string();
    
    // Replace name codes: \N[n] -> [NAME_n]
    let name_regex = Regex::new(r"\\N\[(\d+)\]").unwrap();
    result = name_regex.replace_all(&result, "[NAME_$1]").to_string();
    
    // Replace newline codes: \n[n] -> [NEWLINE_n]
    let newline_regex = Regex::new(r"\\n\[(\d+)\]").unwrap();
    result = newline_regex.replace_all(&result, "[NEWLINE_$1]").to_string();
    
    // Replace other common RPG Maker codes
    result = result.replace("\\V[", "[VARIABLE_");
    result = result.replace("\\S[", "[SWITCH_");
    result = result.replace("\\I[", "[ITEM_");
    result = result.replace("\\W[", "[WEAPON_");
    result = result.replace("\\A[", "[ARMOR_");
    result = result.replace("\\P[", "[ACTOR_");
    result = result.replace("\\G", "[GOLD]");
    result = result.replace("\\$", "[CURRENCY]");
    
    // Replace RPG Maker conditional expressions
    let conditional_regex = Regex::new(r"en\(v\[(\d+)\]>(\d+)\)").unwrap();
    result = conditional_regex.replace_all(&result, "[CONDITIONAL_v$1>$2]").to_string();
    
    result
}

/// Restore RPG Maker formatting codes from placeholder names after translation
/// 
/// # Arguments
/// * `text` - The translated text with placeholder names
/// 
/// # Returns
/// * `String` - Text with placeholders replaced by original formatting codes
pub fn restore_formatting_codes_after_translation(text: &str) -> String {
    let mut result = text.to_string();
    
    // Restore color codes: [COLOR_n] -> \C[n]
    let color_regex = Regex::new(r"\[COLOR_(\d+)\]").unwrap();
    result = color_regex.replace_all(&result, "\\C[$1]").to_string();
    
    // Restore name codes: [NAME_n] -> \N[n]
    let name_regex = Regex::new(r"\[NAME_(\d+)\]").unwrap();
    result = name_regex.replace_all(&result, "\\N[$1]").to_string();
    
    // Restore newline codes: [NEWLINE_n] -> \n[n]
    let newline_regex = Regex::new(r"\[NEWLINE_(\d+)\]").unwrap();
    result = newline_regex.replace_all(&result, "\\n[$1]").to_string();
    
    // Restore other common RPG Maker codes
    result = result.replace("[VARIABLE_", "\\V[");
    result = result.replace("[SWITCH_", "\\S[");
    result = result.replace("[ITEM_", "\\I[");
    result = result.replace("[WEAPON_", "\\W[");
    result = result.replace("[ARMOR_", "\\A[");
    result = result.replace("[ACTOR_", "\\P[");
    result = result.replace("[GOLD]", "\\G");
    result = result.replace("[CURRENCY]", "\\$");
    
    // Restore RPG Maker conditional expressions
    let conditional_regex = Regex::new(r"\[CONDITIONAL_v(\d+)>(\d+)\]").unwrap();
    result = conditional_regex.replace_all(&result, "en(v[$1]>$2)").to_string();
    
    result
}

/// Helper function to check if a character is ASCII or full-width Latin
fn is_ascii_or_fullwidth_latin(c: char) -> bool {
    c.is_ascii_alphanumeric() || 
    c.is_ascii_punctuation() || 
    c.is_ascii_whitespace() ||
    // Full-width Latin characters (U+FF21-FF5A for letters, U+FF10-FF19 for digits)
    (c >= '\u{FF21}' && c <= '\u{FF5A}') || // Full-width uppercase letters
    (c >= '\u{FF41}' && c <= '\u{FF5A}') || // Full-width lowercase letters  
    (c >= '\u{FF10}' && c <= '\u{FF19}')    // Full-width digits
}

/// Helper function to check if content contains Japanese punctuation
fn contains_japanese_punctuation(content: &str) -> bool {
    content.contains('「') || content.contains('」') || content.contains('、') || 
    content.contains('。') || content.contains('・') || content.contains('…')
}

/// Checks if content is technical and shouldn't be translated
///
/// # Arguments
/// * `content` - The content to check
///
/// # Returns
/// * `bool` - True if the content is technical and should be skipped
pub fn is_technical_content(content: &str) -> bool {
    let content = content.trim();
    
    // Skip empty or whitespace-only content
    if content.is_empty() {
        return true;
    }
    
    // Skip EVXXX event names (technical identifiers)
    if content.starts_with("EV") && content.len() >= 3 {
        let suffix = &content[2..];
        if suffix.chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
    }
    
    // Skip MAPXXX switch names (technical identifiers)
    if content.starts_with("MAP") && content.len() >= 4 {
        let suffix = &content[3..];
        if suffix.chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
    }
    
    // Skip pure formatting codes (like "\\n[2]" alone)
    if content == "\\n[1]" || content == "\\n[2]" || content == "\\n[3]" || content == "\\n[4]" || content == "\\n[5]" {
        return true;
    }
    
    // Skip file names and extensions (images, sounds, etc.)
    if content.contains('.') || content.contains('/') || 
       (content.contains('\\') && !content.contains("\\n[") && !content.contains("\\C[") && !content.contains("\\N[")) {
        return true;
    }
    
    // Skip JavaScript code and expressions
    if content.contains("user.") || content.contains("use.") || content.contains("&&") || content.contains("==") {
        return true;
    }
    
    // Skip technical markers
    if content == "終わり" || content == "==" || content.starts_with("==") {
        return true;
    }
    
    // Skip sound effect names (usually English words)
    if content.chars().all(|c| c.is_ascii_alphabetic()) && content.len() <= 20 {
        return true;
    }
    
    // Skip pure ASCII/Latin text (including full-width)
    if content.chars().all(is_ascii_or_fullwidth_latin) {
        return true;
    }
    
    // Skip technical variable names and identifiers
    if content.contains('_') && content.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '_' || c == 'x' || c == 'X'
    }) {
        return true;
    }
    
    // Skip numeric-only content
    if content.chars().all(|c| c.is_ascii_digit()) {
        return true;
    }
    
    // Skip very short content that's likely technical
    // But allow Japanese/Chinese characters even if short
    if content.len() <= 3 {
        // If it contains non-ASCII characters or Japanese punctuation, it might be translatable
        if content.chars().any(|c| c.is_alphabetic() && !c.is_ascii()) ||
           contains_japanese_punctuation(content) {
            return false;
        }
        return true;
    }
    
    // If content contains Japanese characters or other translatable text, allow it
    if content.chars().any(|c| c.is_alphabetic() && !c.is_ascii()) {
        return false;
    }
    
    // If content contains common Japanese punctuation or quotes, allow it
    if contains_japanese_punctuation(content) {
        return false;
    }
    
    false
} 