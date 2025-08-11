use regex::{Captures, Regex};

/// Convert full-width digits to ASCII digits within a string
fn to_ascii_digits(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '０'..='９' => char::from_u32('0' as u32 + (c as u32 - '０' as u32)).unwrap(),
            d => d,
        })
        .collect()
}

/// Replace RPG Maker formatting codes with placeholder names for translation
/// 
/// # Arguments
/// * `text` - The text containing RPG Maker formatting codes
/// 
/// # Returns
/// * `String` - Text with formatting codes replaced by placeholders
pub fn replace_formatting_codes_for_translation(text: &str) -> String {
    let mut result = text.to_string();

    // Preserve numeric map/area prefixes like "100＿..." or "１００_..." by tokenizing them
    // to a non-translatable placeholder. We'll restore them after translation.
    if let Some(caps) = Regex::new(r"^([0-9０-９]{3})[＿_](.+)$").unwrap().captures(&result) {
        let prefix_ascii = to_ascii_digits(&caps[1]);
        let tail = caps[2].to_string();
        result = format!("[NUM_PREFIX_{}]{}", prefix_ascii, tail);
    }
    
    // Replace color codes: \C[n] -> [COLOR_n]
    let color_regex = Regex::new(r"\\C\[(\d+)\]").unwrap();
    result = color_regex.replace_all(&result, "[COLOR_$1]").to_string();
    // Lowercase color code: \c[n] -> [COLOR_n]
    let color_regex_lower = Regex::new(r"\\c\[(\d+)\]").unwrap();
    result = color_regex_lower.replace_all(&result, "[COLOR_$1]").to_string();
    
    // Replace name codes: \N[n] -> [NAME_n]
    let name_regex = Regex::new(r"\\N\[(\d+)\]").unwrap();
    result = name_regex.replace_all(&result, "[NAME_$1]").to_string();
    
    // Replace newline codes: \n[n] -> [NEWLINE_n]
    let newline_regex = Regex::new(r"\\n\[(\d+)\]").unwrap();
    result = newline_regex.replace_all(&result, "[NEWLINE_$1]").to_string();
    
    // Replace other common RPG Maker codes
    result = result.replace("\\V[", "[VARIABLE_");
    result = result.replace("\\v[", "[variable_");
    result = result.replace("\\S[", "[SWITCH_");
    result = result.replace("\\I[", "[ITEM_");
    result = result.replace("\\W[", "[WEAPON_");
    result = result.replace("\\A[", "[ARMOR_");
    result = result.replace("\\P[", "[ACTOR_");
    result = result.replace("\\G", "[GOLD]");
    result = result.replace("\\$", "[CURRENCY]");

    // Map parameter placeholders: handle ASCII and full-width forms
    // "%1" or "％１" -> "[ARG_1]"
    let arg_any = Regex::new(r"[%％]([0-9０-９]+)").unwrap();
    result = arg_any
        .replace_all(&result, |caps: &Captures| format!("[ARG_{}]", to_ascii_digits(&caps[1])))
        .to_string();

    // Normalize control codes
    result = result.replace("\\.", "[CTRL_DOT]");
    result = result.replace("\\|", "[CTRL_WAIT]");
    result = result.replace("\\^", "[CTRL_INSTANT]");
    result = result.replace("\\!", "[CTRL_INPUT]");
    
    // Replace RPG Maker conditional expressions
    let conditional_regex = Regex::new(r"en\(v\[(\d+)\]>(\d+)\)").unwrap();
    result = conditional_regex.replace_all(&result, "[CONDITIONAL_v$1>$2]").to_string();
    
    // Encode significant whitespace so the LLM cannot collapse or trim it
    result = encode_whitespace_placeholders(&result);

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
    result = result.replace("[variable_", "\\v[");
    result = result.replace("[SWITCH_", "\\S[");
    result = result.replace("[ITEM_", "\\I[");
    result = result.replace("[WEAPON_", "\\W[");
    result = result.replace("[ARMOR_", "\\A[");
    result = result.replace("[ACTOR_", "\\P[");
    result = result.replace("[GOLD]", "\\G");
    result = result.replace("[CURRENCY]", "\\$");

    // Restore parameter placeholders: [ARG_n] -> %n
    let arg_regex = Regex::new(r"\[ARG_(\d+)\]").unwrap();
    result = arg_regex.replace_all(&result, "%$1").to_string();

    // Restore numeric prefix placeholders back to their original form: [NUM_PREFIX_200]Tail -> 200＿Tail
    let num_prefix = Regex::new(r"\[NUM_PREFIX_(\d{3})\]").unwrap();
    result = num_prefix
        .replace_all(&result, |caps: &Captures| format!("{}＿", &caps[1]))
        .to_string();

    // Restore control codes
    result = result.replace("[CTRL_DOT]", "\\.");
    result = result.replace("[CTRL_WAIT]", "\\|");
    result = result.replace("[CTRL_INSTANT]", "\\^");
    result = result.replace("[CTRL_INPUT]", "\\!");
    
    // Restore RPG Maker conditional expressions
    let conditional_regex = Regex::new(r"\[CONDITIONAL_v(\d+)>(\d+)\]").unwrap();
    result = conditional_regex.replace_all(&result, "en(v[$1]>$2)").to_string();
    
    // Decode previously encoded whitespace placeholders back to actual spaces
    result = decode_whitespace_placeholders(&result);

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
    // Detect if the content visually looks like CJK (Han, Kana, Hangul) or uses JP punctuation
    let looks_cjk = content.chars().any(|c|
        (c >= '\u{4E00}' && c <= '\u{9FFF}') || // CJK Unified Ideographs
        (c >= '\u{3040}' && c <= '\u{309F}') || // Hiragana
        (c >= '\u{30A0}' && c <= '\u{30FF}') || // Katakana
        (c >= '\u{AC00}' && c <= '\u{D7AF}')    // Hangul Syllables
    ) || contains_japanese_punctuation(content);
    
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
    
    // Skip sound effect-like short ASCII words only when embedded in CJK-looking content
    if looks_cjk && content.chars().all(|c| c.is_ascii_alphabetic()) && content.len() <= 20 {
        return true;
    }
    
    // Skip pure ASCII/Latin text only when content overall looks CJK
    if looks_cjk && content.chars().all(is_ascii_or_fullwidth_latin) {
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
    
    // Skip very short content only in CJK-looking context
    // But allow Japanese/Chinese characters even if short
    if looks_cjk && content.len() <= 3 {
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

/// Encode significant whitespace (leading/trailing, runs of ASCII spaces, full-width spaces, tabs)
/// into bracketed placeholders to preserve them through translation.
///
/// Rules:
/// - Full-width spaces (U+3000 '　') are always encoded as [FWSPC_n]
/// - Leading/trailing ASCII spaces are encoded as [SPC_n]
/// - Interior runs of ASCII spaces of length >= 2 are encoded as [SPC_n]
/// - Tabs runs are encoded as [TAB_n]
fn encode_whitespace_placeholders(input: &str) -> String {
    let mut result = input.to_string();

    // Encode any run of full-width spaces anywhere
    let fw_space_regex = Regex::new(r"(　+)").unwrap();
    result = fw_space_regex
        .replace_all(&result, |caps: &Captures| {
            let count = caps[1].chars().count();
            format!("[FWSPC_{}]", count)
        })
        .to_string();

    // Encode leading ASCII spaces (one or more)
    let leading_spaces = Regex::new(r"^( +)").unwrap();
    result = leading_spaces
        .replace(&result, |caps: &Captures| {
            let count = caps[1].len();
            format!("[SPC_{}]", count)
        })
        .to_string();

    // Encode trailing ASCII spaces (one or more)
    let trailing_spaces = Regex::new(r"( +)$").unwrap();
    result = trailing_spaces
        .replace(&result, |caps: &Captures| {
            let count = caps[1].len();
            format!("[SPC_{}]", count)
        })
        .to_string();

    // Encode interior runs of ASCII spaces (length >= 2)
    let multi_spaces = Regex::new(r"( {2,})").unwrap();
    result = multi_spaces
        .replace_all(&result, |caps: &Captures| {
            let count = caps[1].len();
            format!("[SPC_{}]", count)
        })
        .to_string();

    // Encode tabs
    let tabs = Regex::new(r"(\t+)").unwrap();
    result = tabs
        .replace_all(&result, |caps: &Captures| {
            let count = caps[1].len();
            format!("[TAB_{}]", count)
        })
        .to_string();

    result
}

/// Decode whitespace placeholders back to their original characters
fn decode_whitespace_placeholders(input: &str) -> String {
    let mut result = input.to_string();

    // Decode full-width spaces
    let fw_space_regex = Regex::new(r"\[FWSPC_(\d+)\]").unwrap();
    result = fw_space_regex
        .replace_all(&result, |caps: &Captures| {
            let count: usize = caps[1].parse().unwrap_or(0);
            "　".repeat(count)
        })
        .to_string();

    // Decode ASCII spaces
    let spc_regex = Regex::new(r"\[SPC_(\d+)\]").unwrap();
    result = spc_regex
        .replace_all(&result, |caps: &Captures| {
            let count: usize = caps[1].parse().unwrap_or(0);
            " ".repeat(count)
        })
        .to_string();

    // Decode tabs
    let tab_regex = Regex::new(r"\[TAB_(\d+)\]").unwrap();
    result = tab_regex
        .replace_all(&result, |caps: &Captures| {
            let count: usize = caps[1].parse().unwrap_or(0);
            "\t".repeat(count)
        })
        .to_string();

    result
}