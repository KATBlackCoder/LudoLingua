use once_cell::sync::Lazy;
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

// === PRE-COMPILED REGEXES (COMPILED ONCE, USED FOREVER) ===

// RPG Maker regexes
static COLOR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\C\[(\d+)\]").unwrap());
static COLOR_REGEX_LOWER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\c\[(\d+)\]").unwrap());
static NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\N\[(\d+)\]").unwrap());
static NEWLINE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\n\[(\d+)\]").unwrap());

// Wolf RPG regexes
static ICON_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\i\[(\d+)\]").unwrap());
static FONT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\f\[(\d+)\]").unwrap());
static AT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"@(\d+)").unwrap());
static SLOT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\s\[(\d+)\]").unwrap());
static CSELF_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\cself\[(\d+)\]").unwrap());

// Universal regexes
static ARG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[%％]([0-9０-９]+)").unwrap());
static CONDITIONAL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"en\(v\[(\d+)\]>(\d+)\)").unwrap());
static NUM_PREFIX_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^([0-9０-９]{3})[＿_](.+)$").unwrap());

// Whitespace regexes
static FW_SPACE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(　+)").unwrap());
static LEADING_SPACES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^( +)").unwrap());
static TRAILING_SPACES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"( +)$").unwrap());
static MULTI_SPACES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"( {2,})").unwrap());
static TABS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\t+)").unwrap());

// Restoration regexes
static COLOR_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[COLOR_(\d+)\]").unwrap());
static NAME_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[NAME_(\d+)\]").unwrap());
static NEWLINE_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[NEWLINE_(\d+)\]").unwrap());
static ICON_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[ICON_(\d+)\]").unwrap());
static FONT_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[FONT_(\d+)\]").unwrap());
static AT_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[AT_(\d+)\]").unwrap());
static SLOT_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[SLOT_(\d+)\]").unwrap());
static CSELF_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[CSELF_(\d+)\]").unwrap());
static ARG_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[ARG_(\d+)\]").unwrap());
static CONDITIONAL_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[CONDITIONAL_v(\d+)>(\d+)\]").unwrap());
static NUM_PREFIX_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[NUM_PREFIX_(\d{3})\]").unwrap());
static FW_SPACE_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[FWSPC_(\d+)\]").unwrap());
static SPC_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[SPC_(\d+)\]").unwrap());
static TAB_RESTORE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[TAB_(\d+)\]").unwrap());

/// Optimized text formatter with pre-compiled regexes
pub struct OptimizedTextFormatter;

impl OptimizedTextFormatter {
    /// Optimized preparation for translation using pre-compiled regexes with early exit
    pub fn prepare_for_translation(text: &str) -> String {
        // Early exit optimization: Skip processing for plain text without formatting codes
        if !Self::has_formatting_codes(text) {
            return text.to_string();
        }
        
        let mut result = text.to_string();

        // Preserve numeric map/area prefixes
        if let Some(caps) = NUM_PREFIX_REGEX.captures(&result) {
            let prefix_ascii = to_ascii_digits(&caps[1]);
            let tail = caps[2].to_string();
            result = format!("[NUM_PREFIX_{}]{}", prefix_ascii, tail);
        }

        // === RPG MAKER CODES (using pre-compiled regexes) ===
        result = COLOR_REGEX.replace_all(&result, "[COLOR_$1]").to_string();
        result = COLOR_REGEX_LOWER.replace_all(&result, "[COLOR_$1]").to_string();
        result = NAME_REGEX.replace_all(&result, "[NAME_$1]").to_string();
        result = NEWLINE_REGEX.replace_all(&result, "[NEWLINE_$1]").to_string();

        // Other RPG Maker codes (simple string replacements)
        result = result.replace("\\V[", "[VARIABLE_");
        result = result.replace("\\v[", "[variable_");
        result = result.replace("\\S[", "[SWITCH_");
        result = result.replace("\\I[", "[ITEM_");
        result = result.replace("\\W[", "[WEAPON_");
        result = result.replace("\\A[", "[ARMOR_");
        result = result.replace("\\P[", "[ACTOR_");
        result = result.replace("\\G", "[GOLD]");
        result = result.replace("\\$", "[CURRENCY]");

        // === WOLF RPG CODES (using pre-compiled regexes) ===
        result = result.replace("\\E", "[WOLF_END]");
        result = ICON_REGEX.replace_all(&result, "[ICON_$1]").to_string();
        result = FONT_REGEX.replace_all(&result, "[FONT_$1]").to_string();
        result = AT_REGEX.replace_all(&result, "[AT_$1]").to_string();
        result = SLOT_REGEX.replace_all(&result, "[SLOT_$1]").to_string();
        result = CSELF_REGEX.replace_all(&result, "[CSELF_$1]").to_string();

        // Other Wolf RPG codes
        result = result.replace("\\r", "[RUBY_START]");
        result = result.replace('\r', "[CARRIAGE_RETURN]");
        result = result.replace('\n', "[NEWLINE]");

        // Japanese quotation marks
        result = result.replace('「', "\"");
        result = result.replace('」', "\"");

        // === UNIVERSAL CODES (using pre-compiled regexes) ===
        result = ARG_REGEX
            .replace_all(&result, |caps: &Captures| {
                format!("[ARG_{}]", to_ascii_digits(&caps[1]))
            })
            .to_string();

        // Control codes
        result = result.replace("\\.", "[CTRL_DOT]");
        result = result.replace("\\|", "[CTRL_WAIT]");
        result = result.replace("\\^", "[CTRL_INSTANT]");
        result = result.replace("\\!", "[CTRL_INPUT]");
        result = result.replace("\\{", "[CTRL_OPEN_BRACE]");

        // RPG Maker conditional expressions
        result = CONDITIONAL_REGEX
            .replace_all(&result, "[CONDITIONAL_v$1>$2]")
            .to_string();

        // === WHITESPACE ENCODING (using pre-compiled regexes) ===
        result = Self::encode_whitespace_placeholders(&result);

        result
    }

    /// Optimized restoration after translation using pre-compiled regexes with early exit
    pub fn restore_after_translation(text: &str) -> String {
        // Early exit optimization: Skip processing for plain text without placeholders
        if !Self::has_placeholder_codes(text) {
            return text.to_string();
        }
        
        let mut result = text.to_string();

        // === WHITESPACE DECODING (using pre-compiled regexes) ===
        result = Self::decode_whitespace_placeholders(&result);

        // === RPG MAKER CODES (using pre-compiled regexes) ===
        result = COLOR_RESTORE_REGEX.replace_all(&result, "\\C[$1]").to_string();
        result = NAME_RESTORE_REGEX.replace_all(&result, "\\N[$1]").to_string();
        result = NEWLINE_RESTORE_REGEX.replace_all(&result, "\\n[$1]").to_string();

        // Other RPG Maker codes
        result = result.replace("[VARIABLE_", "\\V[");
        result = result.replace("[variable_", "\\v[");
        result = result.replace("[SWITCH_", "\\S[");
        result = result.replace("[ITEM_", "\\I[");
        result = result.replace("[WEAPON_", "\\W[");
        result = result.replace("[ARMOR_", "\\A[");
        result = result.replace("[ACTOR_", "\\P[");
        result = result.replace("[GOLD]", "\\G");
        result = result.replace("[CURRENCY]", "\\$");

        // === WOLF RPG CODES (using pre-compiled regexes) ===
        result = result.replace("[WOLF_END]", "\\E");
        result = ICON_RESTORE_REGEX.replace_all(&result, "\\i[$1]").to_string();
        result = FONT_RESTORE_REGEX.replace_all(&result, "\\f[$1]").to_string();
        result = AT_RESTORE_REGEX.replace_all(&result, "@$1").to_string();
        result = SLOT_RESTORE_REGEX.replace_all(&result, "\\s[$1]").to_string();
        result = CSELF_RESTORE_REGEX.replace_all(&result, "\\cself[$1]").to_string();

        // Other Wolf RPG codes
        result = result.replace("[RUBY_START]", "\\r");
        result = result.replace("[CARRIAGE_RETURN]", "\\r");
        result = result.replace("[NEWLINE]", "\n");

        // === UNIVERSAL CODES (using pre-compiled regexes) ===
        result = ARG_RESTORE_REGEX.replace_all(&result, "%$1").to_string();

        // Restore numeric prefix placeholders
        result = NUM_PREFIX_RESTORE_REGEX
            .replace_all(&result, |caps: &Captures| format!("{}＿", &caps[1]))
            .to_string();

        // Restore control codes
        result = result.replace("[CTRL_DOT]", "\\.");
        result = result.replace("[CTRL_WAIT]", "\\|");
        result = result.replace("[CTRL_INSTANT]", "\\^");
        result = result.replace("[CTRL_INPUT]", "\\!");
        result = result.replace("[CTRL_OPEN_BRACE]", "\\{");

        // Restore RPG Maker conditional expressions
        result = CONDITIONAL_RESTORE_REGEX
            .replace_all(&result, "en(v[$1]>$2)")
            .to_string();

        result
    }

    /// Quick check for formatting codes in text (1μs operation)
    /// Returns true if text contains any formatting codes that need processing
    fn has_formatting_codes(text: &str) -> bool {
        // Check for common formatting patterns without regex compilation
        text.contains('\\') ||           // RPG Maker codes: \C, \N, \V, \I, etc.
        text.contains('@') ||            // Wolf RPG codes: @1, @2, @3
        text.contains('%') ||            // Parameter codes: %1, %2, %3
        text.contains('％') ||           // Parameter codes: ％1, ％2, ％3
        text.contains('[') ||            // Bracketed codes: [COLOR_1], [ITEM_1]
        text.contains(']') ||            // Bracketed codes: [COLOR_1], [ITEM_1]
        text.contains('「') ||           // Japanese quotes: 「」
        text.contains('」') ||           // Japanese quotes: 「」
        text.contains('\r') ||           // Control characters
        text.contains('\n') ||           // Control characters
        text.contains('\t') ||           // Control characters
        text.contains('　')              // Full-width spaces
    }

    /// Quick check for placeholder codes in text (1μs operation)
    /// Returns true if text contains any placeholders that need restoration
    fn has_placeholder_codes(text: &str) -> bool {
        // Check for placeholder patterns without regex compilation
        text.contains('[') ||            // Placeholder codes: [COLOR_1], [ITEM_1], [ARG_1]
        text.contains(']') ||            // Placeholder codes: [COLOR_1], [ITEM_1], [ARG_1]
        text.contains('％') ||           // Parameter codes: %1, ％1, %2, ％2
        text.contains('\\') ||           // Control codes: \., \|, \^, \!
        text.contains('@') ||            // Wolf RPG codes: @1, @2, @3
        text.contains('「') ||           // Japanese quotes: 「」
        text.contains('」') ||           // Japanese quotes: 「」
        text.contains('\r') ||           // Control characters
        text.contains('\n') ||           // Control characters
        text.contains('\t') ||           // Control characters
        text.contains('　')              // Full-width spaces
    }

    /// Encode significant whitespace using pre-compiled regexes
    fn encode_whitespace_placeholders(input: &str) -> String {
        let mut result = input.to_string();

        // Encode full-width spaces
        result = FW_SPACE_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count = caps[1].chars().count();
                format!("[FWSPC_{}]", count)
            })
            .to_string();

        // Encode leading ASCII spaces
        result = LEADING_SPACES_REGEX
            .replace(&result, |caps: &Captures| {
                let count = caps[1].len();
                format!("[SPC_{}]", count)
            })
            .to_string();

        // Encode trailing ASCII spaces
        result = TRAILING_SPACES_REGEX
            .replace(&result, |caps: &Captures| {
                let count = caps[1].len();
                format!("[SPC_{}]", count)
            })
            .to_string();

        // Encode interior runs of ASCII spaces
        result = MULTI_SPACES_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count = caps[1].len();
                format!("[SPC_{}]", count)
            })
            .to_string();

        // Encode tabs
        result = TABS_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count = caps[1].len();
                format!("[TAB_{}]", count)
            })
            .to_string();

        result
    }

    /// Decode whitespace placeholders using pre-compiled regexes
    fn decode_whitespace_placeholders(input: &str) -> String {
        let mut result = input.to_string();

        // Decode full-width spaces
        result = FW_SPACE_RESTORE_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count: usize = caps[1].parse().unwrap_or(0);
                "　".repeat(count)
            })
            .to_string();

        // Decode ASCII spaces
        result = SPC_RESTORE_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count: usize = caps[1].parse().unwrap_or(0);
                " ".repeat(count)
            })
            .to_string();

        // Decode tabs
        result = TAB_RESTORE_REGEX
            .replace_all(&result, |caps: &Captures| {
                let count: usize = caps[1].parse().unwrap_or(0);
                "\t".repeat(count)
            })
            .to_string();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_formatting() {
        let input = "%1は瞑想した！";
        let expected_prepared = "[ARG_1]は瞑想した！";
        let expected_restored = "%1は瞑想した！";
        
        // Test preparation for translation
        let prepared = OptimizedTextFormatter::prepare_for_translation(input);
        assert_eq!(prepared, expected_prepared, "Parameter placeholder should be converted to [ARG_1]");
        
        // Test restoration after translation
        let restored = OptimizedTextFormatter::restore_after_translation(&prepared);
        assert_eq!(restored, expected_restored, "Parameter placeholder should be restored to %1");
    }

    #[test]
    fn test_optimized_item_codes() {
        let input = "「あああぁ……\\I[317]こってりドロドロぉ……\\I[317]";
        let expected_prepared = "\"あああぁ……[ITEM_317]こってりドロドロぉ……[ITEM_317]";
        let expected_restored = "\"あああぁ……\\I[317]こってりドロドロぉ……\\I[317]";
        
        // Test preparation for translation
        let prepared = OptimizedTextFormatter::prepare_for_translation(input);
        assert_eq!(prepared, expected_prepared, "Item codes should be converted to placeholders");
        
        // Test restoration after translation
        let restored = OptimizedTextFormatter::restore_after_translation(&prepared);
        assert_eq!(restored, expected_restored, "Item codes should be restored to original format");
    }

    #[test]
    fn test_early_exit_plain_text() {
        // Test early exit for plain text (no formatting codes)
        let plain_texts = vec![
            "勇者",
            "魔法使い", 
            "薬草",
            "はい",
            "いいえ",
            "こんにちは",
            "Hello World",
            "123",
            "テスト",
        ];
        
        for text in plain_texts {
            let result = OptimizedTextFormatter::prepare_for_translation(text);
            assert_eq!(result, text, "Plain text '{}' should be returned unchanged", text);
        }
    }

    #[test]
    fn test_early_exit_complex_text() {
        // Test that complex text still gets processed
        let complex_texts = vec![
            ("\\C[1]勇者\\C[0]", "[COLOR_1]勇者[COLOR_0]"),
            ("\\N[1]こんにちは", "[NAME_1]こんにちは"),
            ("\\I[317]薬草", "[ITEM_317]薬草"),
            ("@1テスト", "[AT_1]テスト"),
            ("％1は瞑想した！", "[ARG_1]は瞑想した！"),
            ("「勇者」", "\"勇者\""),
        ];
        
        for (input, expected) in complex_texts {
            let result = OptimizedTextFormatter::prepare_for_translation(input);
            assert_eq!(result, expected, "Complex text '{}' should be processed", input);
        }
    }

    #[test]
    fn test_early_exit_restore_plain_text() {
        // Test early exit for plain text during restoration
        let plain_texts = vec![
            "勇者",
            "魔法使い",
            "薬草", 
            "Hello World",
            "123",
        ];
        
        for text in plain_texts {
            let result = OptimizedTextFormatter::restore_after_translation(text);
            assert_eq!(result, text, "Plain text '{}' should be returned unchanged during restoration", text);
        }
    }

    #[test]
    fn test_early_exit_restore_complex_text() {
        // Test that complex placeholders still get restored
        let complex_texts = vec![
            ("[COLOR_1]勇者[COLOR_0]", "\\C[1]勇者\\C[0]"),
            ("[NAME_1]こんにちは", "\\N[1]こんにちは"),
            ("[ITEM_317]薬草", "\\I[317]薬草"),
            ("[AT_1]テスト", "@1テスト"),
            ("[ARG_1]は瞑想した！", "%1は瞑想した！"),
            // Note: Japanese quotes are converted to ASCII quotes during preparation,
            // so they remain as ASCII quotes during restoration (this is correct behavior)
            ("\"勇者\"", "\"勇者\""),
        ];
        
        for (input, expected) in complex_texts {
            let result = OptimizedTextFormatter::restore_after_translation(input);
            assert_eq!(result, expected, "Complex placeholder '{}' should be restored", input);
        }
    }
}
