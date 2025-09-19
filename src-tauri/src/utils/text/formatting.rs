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

/// Universal text formatter for all engines
/// 
/// This struct provides unified text processing that handles ALL placeholder types
/// from RPG Maker, Wolf RPG, and future engines in a single pipeline.
pub struct TextFormatter;

impl TextFormatter {
    /// UNIFIED METHOD: Handles ALL placeholders in one pass
    /// 
    /// This method processes text from any engine and converts all formatting codes,
    /// whitespace patterns, and engine-specific codes into unified placeholders.
    /// 
    /// Supported placeholder types:
    /// - RPG Maker codes: \C[n], \N[n], \V[n], etc.
    /// - Wolf RPG codes: \E, \i[n], \f[n], @n, etc.
    /// - Universal whitespace: leading/trailing spaces, full-width spaces, tabs
    /// - Parameter placeholders: %n, ％n
    /// - Control codes: \., \|, \^, \!
    pub fn prepare_for_translation(text: &str) -> String {
        let mut result = text.to_string();

        // Preserve numeric map/area prefixes like "100＿..." or "１００_..." by tokenizing them
        // to a non-translatable placeholder. We'll restore them after translation.
        if let Some(caps) = Regex::new(r"^([0-9０-９]{3})[＿_](.+)$")
            .unwrap()
            .captures(&result)
        {
            let prefix_ascii = to_ascii_digits(&caps[1]);
            let tail = caps[2].to_string();
            result = format!("[NUM_PREFIX_{}]{}", prefix_ascii, tail);
        }

        // === RPG MAKER CODES ===
        
        // Color codes: \C[n] -> [COLOR_n] and \c[n] -> [COLOR_n]
        let color_regex = Regex::new(r"\\C\[(\d+)\]").unwrap();
        result = color_regex.replace_all(&result, "[COLOR_$1]").to_string();
        let color_regex_lower = Regex::new(r"\\c\[(\d+)\]").unwrap();
        result = color_regex_lower.replace_all(&result, "[COLOR_$1]").to_string();

        // Name codes: \N[n] -> [NAME_n]
        let name_regex = Regex::new(r"\\N\[(\d+)\]").unwrap();
        result = name_regex.replace_all(&result, "[NAME_$1]").to_string();

        // Newline codes: \n[n] -> [NEWLINE_n]
        let newline_regex = Regex::new(r"\\n\[(\d+)\]").unwrap();
        result = newline_regex.replace_all(&result, "[NEWLINE_$1]").to_string();

        // Other RPG Maker codes
        result = result.replace("\\V[", "[VARIABLE_");
        result = result.replace("\\v[", "[variable_");
        result = result.replace("\\S[", "[SWITCH_");
        result = result.replace("\\I[", "[ITEM_");
        result = result.replace("\\W[", "[WEAPON_");
        result = result.replace("\\A[", "[ARMOR_");
        result = result.replace("\\P[", "[ACTOR_");
        result = result.replace("\\G", "[GOLD]");
        result = result.replace("\\$", "[CURRENCY]");

        // === WOLF RPG CODES ===
        
        // Wolf RPG specific codes
        result = result.replace("\\E", "[WOLF_END]");
        
        // Icon codes: \i[n] -> [ICON_n]
        let icon_codes = Regex::new(r"\\i\[(\d+)\]").unwrap();
        result = icon_codes.replace_all(&result, "[ICON_$1]").to_string();
        
        // Font codes: \f[n] -> [FONT_n]
        let font_codes = Regex::new(r"\\f\[(\d+)\]").unwrap();
        result = font_codes.replace_all(&result, "[FONT_$1]").to_string();
        
        // Event markers: @n -> [AT_n]
        let at_codes = Regex::new(r"@(\d+)").unwrap();
        result = at_codes.replace_all(&result, "[AT_$1]").to_string();
        
        // Character slots: \s[n] -> [SLOT_n]
        let slot_codes = Regex::new(r"\\s\[(\d+)\]").unwrap();
        result = slot_codes.replace_all(&result, "[SLOT_$1]").to_string();
        
        // Self color codes: \cself[n] -> [CSELF_n]
        let cself_codes = Regex::new(r"\\cself\[(\d+)\]").unwrap();
        result = cself_codes.replace_all(&result, "[CSELF_$1]").to_string();
        
        // Ruby text marker: \r -> [RUBY_START]
        result = result.replace("\\r", "[RUBY_START]");
        
        // Carriage return: \r -> [CARRIAGE_RETURN]
        result = result.replace('\r', "[CARRIAGE_RETURN]");
        
        // Newline character: \n -> [NEWLINE]
        result = result.replace('\n', "[NEWLINE]");

        // === UNIVERSAL CODES ===
        
        // Parameter placeholders: handle ASCII and full-width forms
        // "%1" or "％１" -> "[ARG_1]"
        let arg_any = Regex::new(r"[%％]([0-9０-９]+)").unwrap();
        result = arg_any
            .replace_all(&result, |caps: &Captures| {
                format!("[ARG_{}]", to_ascii_digits(&caps[1]))
            })
            .to_string();

        // Control codes
        result = result.replace("\\.", "[CTRL_DOT]");
        result = result.replace("\\|", "[CTRL_WAIT]");
        result = result.replace("\\^", "[CTRL_INSTANT]");
        result = result.replace("\\!", "[CTRL_INPUT]");

        // RPG Maker conditional expressions
        let conditional_regex = Regex::new(r"en\(v\[(\d+)\]>(\d+)\)").unwrap();
        result = conditional_regex
            .replace_all(&result, "[CONDITIONAL_v$1>$2]")
            .to_string();

        // === WHITESPACE ENCODING ===
        // Encode significant whitespace so the LLM cannot collapse or trim it
        result = Self::encode_whitespace_placeholders(&result);

        result
    }

    /// UNIFIED METHOD: Restores ALL placeholders in one pass
    /// 
    /// This method restores all placeholders back to their original formatting codes
    /// regardless of which engine they came from.
    pub fn restore_after_translation(text: &str) -> String {
        let mut result = text.to_string();

        // === WHITESPACE DECODING ===
        // Decode previously encoded whitespace placeholders back to actual spaces
        result = Self::decode_whitespace_placeholders(&result);

        // === RPG MAKER CODES ===
        
        // Restore color codes: [COLOR_n] -> \C[n]
        let color_regex = Regex::new(r"\[COLOR_(\d+)\]").unwrap();
        result = color_regex.replace_all(&result, "\\C[$1]").to_string();

        // Restore name codes: [NAME_n] -> \N[n]
        let name_regex = Regex::new(r"\[NAME_(\d+)\]").unwrap();
        result = name_regex.replace_all(&result, "\\N[$1]").to_string();

        // Restore newline codes: [NEWLINE_n] -> \n[n]
        let newline_regex = Regex::new(r"\[NEWLINE_(\d+)\]").unwrap();
        result = newline_regex.replace_all(&result, "\\n[$1]").to_string();

        // Restore other RPG Maker codes
        result = result.replace("[VARIABLE_", "\\V[");
        result = result.replace("[variable_", "\\v[");
        result = result.replace("[SWITCH_", "\\S[");
        result = result.replace("[ITEM_", "\\I[");
        result = result.replace("[WEAPON_", "\\W[");
        result = result.replace("[ARMOR_", "\\A[");
        result = result.replace("[ACTOR_", "\\P[");
        result = result.replace("[GOLD]", "\\G");
        result = result.replace("[CURRENCY]", "\\$");

        // === WOLF RPG CODES ===
        
        // Restore Wolf RPG specific codes
        result = result.replace("[WOLF_END]", "\\E");
        
        // Restore icon codes: [ICON_n] -> \i[n]
        let icon_restore = Regex::new(r"\[ICON_(\d+)\]").unwrap();
        result = icon_restore.replace_all(&result, "\\i[$1]").to_string();
        
        // Restore font codes: [FONT_n] -> \f[n]
        let font_restore = Regex::new(r"\[FONT_(\d+)\]").unwrap();
        result = font_restore.replace_all(&result, "\\f[$1]").to_string();
        
        // Restore @ codes: [AT_n] -> @n
        let at_restore = Regex::new(r"\[AT_(\d+)\]").unwrap();
        result = at_restore.replace_all(&result, "@$1").to_string();
        
        // Restore slot codes: [SLOT_n] -> \s[n]
        let slot_restore = Regex::new(r"\[SLOT_(\d+)\]").unwrap();
        result = slot_restore.replace_all(&result, "\\s[$1]").to_string();
        
        // Restore cself codes: [CSELF_n] -> \cself[n]
        let cself_restore = Regex::new(r"\[CSELF_(\d+)\]").unwrap();
        result = cself_restore.replace_all(&result, "\\cself[$1]").to_string();
        
        // Restore ruby text marker: [RUBY_START] -> \r
        result = result.replace("[RUBY_START]", "\\r");
        
        // Restore carriage return: [CARRIAGE_RETURN] -> \r
        result = result.replace("[CARRIAGE_RETURN]", "\\r");
        
        // Restore newline: [NEWLINE] -> \n
        result = result.replace("[NEWLINE]", "\n");

        // === UNIVERSAL CODES ===
        
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
        result = conditional_regex
            .replace_all(&result, "en(v[$1]>$2)")
            .to_string();

        result
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
}
