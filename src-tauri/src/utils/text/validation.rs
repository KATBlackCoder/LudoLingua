/// Universal validation: common logic for all engines
///
/// This struct provides unified validation logic that works for all engines
/// without engine-specific knowledge.
pub struct ContentValidator;

impl ContentValidator {
    /// Determine the initial translation status based on content and translation context
    ///
    /// This method determines if text should be marked as Ignored when translating
    /// from CJK to ASCII and the text is already in ASCII form.
    pub fn get_initial_status(
        content: &str,
        target_language: &str,
    ) -> crate::models::translation::TranslationStatus {
        let content = content.trim();

        // Check if we're translating to English/ASCII
        let is_ascii_target =
            target_language.to_lowercase() == "en" || target_language.to_lowercase() == "english";

        if is_ascii_target {
            // Check if the text is already in ASCII form
            let is_ascii_text = content.chars().all(|c| {
                c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace()
            });

            // If text is already ASCII and we're translating to English, ignore it
            // (no need to translate English to English)
            if is_ascii_text && content.chars().any(|c| c.is_alphabetic()) {
                return crate::models::translation::TranslationStatus::Ignored;
            }
        }

        // Default to NotTranslated for all other cases
        crate::models::translation::TranslationStatus::NotTranslated
    }

    /// Universal validation logic for text content
    ///
    /// This method determines if text should be translated based on common
    /// validation rules that apply to all engines.
    pub fn validate_text(content: &str) -> bool {
        let content = content.trim();

        // Skip empty or whitespace-only content
        if content.is_empty() {
            return false;
        }

        // Detect if the content visually looks like CJK (Han, Kana, Hangul) or uses JP punctuation
        let looks_cjk = content.chars().any(|c| {
            (c >= '\u{4E00}' && c <= '\u{9FFF}') || // CJK Unified Ideographs
            (c >= '\u{3040}' && c <= '\u{309F}') || // Hiragana
            (c >= '\u{30A0}' && c <= '\u{30FF}') || // Katakana
            (c >= '\u{AC00}' && c <= '\u{D7AF}') || // Hangul Syllables
            Self::contains_japanese_punctuation(content)
        });

        // Skip EVXXX event names (technical identifiers)
        if content.starts_with("EV") && content.len() >= 3 {
            let suffix = &content[2..];
            if suffix.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
        }

        // Skip MAPXXX switch names (technical identifiers)
        if content.starts_with("MAP") && content.len() >= 4 {
            let suffix = &content[3..];
            if suffix.chars().all(|c| c.is_ascii_digit()) {
                return false;
            }
        }

        // Skip pure formatting codes (like "\\n[2]" alone)
        if content == "\\n[1]"
            || content == "\\n[2]"
            || content == "\\n[3]"
            || content == "\\n[4]"
            || content == "\\n[5]"
        {
            return false;
        }

        // Skip file names and extensions (images, sounds, etc.)
        if content.contains('.')
            || content.contains('/')
            || (content.contains('\\')
                && !content.contains("\\n[")
                && !content.contains("\\C[")
                && !content.contains("\\N["))
        {
            return false;
        }

        // Skip JavaScript code and expressions
        if content.contains("user.")
            || content.contains("use.")
            || content.contains("&&")
            || content.contains("==")
        {
            return false;
        }

        // Skip technical markers
        if content == "終わり" || content == "==" || content.starts_with("==") {
            return false;
        }

        // Skip sound effect-like short ASCII words only when embedded in CJK-looking content
        if looks_cjk && content.chars().all(|c| c.is_ascii_alphabetic()) && content.len() <= 20 {
            return false;
        }

        // Skip pure ASCII/Latin text only when content overall looks CJK
        if looks_cjk && content.chars().all(Self::is_ascii_or_fullwidth_latin) {
            return false;
        }

        // Skip technical variable names and identifiers
        if content.contains('_')
            && content
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == 'x' || c == 'X')
        {
            return false;
        }

        // Skip numeric-only content
        if content.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        // Skip very short content only in CJK-looking context
        // But allow Japanese/Chinese characters even if short
        if looks_cjk && content.len() <= 3 {
            // If it contains non-ASCII characters or Japanese punctuation, it might be translatable
            if content.chars().any(|c| c.is_alphabetic() && !c.is_ascii())
                || Self::contains_japanese_punctuation(content)
            {
                return true;
            }
            return false;
        }

        // If content contains Japanese characters or other translatable text, allow it
        if content.chars().any(|c| c.is_alphabetic() && !c.is_ascii()) {
            return true;
        }

        // If content contains common Japanese punctuation or quotes, allow it
        if Self::contains_japanese_punctuation(content) {
            return true;
        }

        // Allow reasonable ASCII text that looks like translatable content
        // (alphabetic characters, reasonable length, not just technical identifiers)
        if content.chars().any(|c| c.is_alphabetic())
            && content.len() >= 2
            && content.len() <= 100
            && !content.chars().all(|c| c.is_ascii_digit())
        {
            return true;
        }

        false
    }

    /// Optional warnings for text that passed validation
    ///
    /// This method can provide warnings about text that might need special attention
    /// during translation, such as very long text or unusual formatting.
    pub fn get_warnings(content: &str) -> Vec<String> {
        let mut warnings = Vec::new();
        let content = content.trim();

        // Very long text warning
        if content.len() > 1000 {
            warnings.push("Very long text - may need special handling".to_string());
        }

        // Unusual formatting warning
        if content.contains("\\") && content.len() < 10 {
            warnings.push("Short text with formatting codes - verify translation".to_string());
        }

        // Mixed script warning
        let has_cjk = content.chars().any(|c| {
            (c >= '\u{4E00}' && c <= '\u{9FFF}') || // CJK Unified Ideographs
            (c >= '\u{3040}' && c <= '\u{309F}') || // Hiragana
            (c >= '\u{30A0}' && c <= '\u{30FF}') || // Katakana
            (c >= '\u{AC00}' && c <= '\u{D7AF}') // Hangul Syllables
        });
        let has_ascii = content.chars().any(|c| c.is_ascii_alphanumeric());

        if has_cjk && has_ascii {
            warnings.push("Mixed script content - verify translation direction".to_string());
        }

        warnings
    }

    /// Helper function to check if a character is ASCII or full-width Latin
    fn is_ascii_or_fullwidth_latin(c: char) -> bool {
        c.is_ascii_alphanumeric() ||
        c.is_ascii_punctuation() ||
        c.is_ascii_whitespace() ||
        // Full-width Latin characters (U+FF21-FF5A for letters, U+FF10-FF19 for digits)
        (c >= '\u{FF21}' && c <= '\u{FF5A}') || // Full-width uppercase letters
        (c >= '\u{FF41}' && c <= '\u{FF5A}') || // Full-width lowercase letters  
        (c >= '\u{FF10}' && c <= '\u{FF19}') // Full-width digits
    }

    /// Helper function to check if content contains Japanese punctuation
    fn contains_japanese_punctuation(content: &str) -> bool {
        content.contains('「')
            || content.contains('」')
            || content.contains('、')
            || content.contains('。')
            || content.contains('・')
            || content.contains('…')
    }
}
