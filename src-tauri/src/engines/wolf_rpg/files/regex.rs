use regex::{Captures, Regex};

/// Heuristic filter: return true if the string likely contains user-facing text we should translate.
/// This is Wolf RPG–tuned and much faster than the generic JSON walker.
pub fn is_translatable_wolf_text(input: &str) -> bool {
    let s = input.trim();
    if s.is_empty() || s.len() < 2 {
        return false;
    }

    // Fast early rejection of common technical patterns
    if is_pure_digits(s) || looks_like_code_fast(s) || looks_like_path_or_file_fast(s) {
        return false;
    }

    // Fast acceptance: strings with CJK or Japanese punctuation (most Wolf RPG content)
    if contains_cjk_or_jp_punct_fast(s) {
        return true;
    }

    // Fast rejection of technical IDs
    if is_technical_id_fast(s) {
        return false;
    }

    // Accept reasonable length text with alphabetic characters and spaces
    s.len() >= 3 && s.len() <= 500 && s.chars().any(|c| c.is_alphabetic()) && (s.contains(' ') || s.chars().any(|c| !c.is_ascii()))
}

fn is_pure_digits(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

fn is_technical_id_fast(s: &str) -> bool {
    // Fast check for common technical patterns
    if s.len() > 20 || s.len() < 2 {
        return false;
    }
    
    // EV###, MAP###, etc.
    if s.len() >= 3 && (s.starts_with("EV") || s.starts_with("MAP")) {
        return s[2..].chars().all(|c| c.is_ascii_digit());
    }
    
    // Check for underscore-heavy technical identifiers
    s.contains('_') && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

fn looks_like_path_or_file_fast(s: &str) -> bool {
    // Fast path/file detection
    if s.len() > 100 {
        return true; // Very long strings are probably paths
    }
    
    // Check for obvious file patterns
    s.contains('/') || s.contains('\\') || 
    s.ends_with(".png") || s.ends_with(".jpg") || s.ends_with(".ogg") || 
    s.ends_with(".wav") || s.ends_with(".mp3") || s.ends_with(".dat")
}

fn looks_like_code_fast(s: &str) -> bool {
    // Fast code detection using byte search
    s.contains("&&") || s.contains("||") || s.contains("==") || 
    s.contains("!=") || s.contains("user.") || s.contains("use.")
}

fn contains_cjk_or_jp_punct_fast(s: &str) -> bool {
    // Fast CJK/Japanese detection - check first few chars for performance
    for c in s.chars().take(10) {
        if (c >= '\u{4E00}' && c <= '\u{9FFF}') || // CJK Unified Ideographs
           (c >= '\u{3040}' && c <= '\u{309F}') || // Hiragana
           (c >= '\u{30A0}' && c <= '\u{30FF}') || // Katakana
           (c >= '\u{AC00}' && c <= '\u{D7AF}') || // Hangul
           c == '「' || c == '」' || c == '、' || c == '。' || c == '・' || c == '…' {
            return true;
        }
    }
    false
}

/// Replace Wolf-specific control codes and preserve significant whitespace/formatting with placeholders.
/// Based on actual Wolf RPG codes: \\E, \n, \\r (ruby), \r (carriage return)
pub fn wolf_replace_placeholders_for_translation(text: &str) -> String {
    let mut result = text.to_string();

    // IMPORTANT: Process control codes BEFORE whitespace encoding to avoid conflicts
    
    // Wolf RPG specific codes found in the actual data
    
    // \\E - End formatting/reset (found in Picture commands)
    result = result.replace("\\E", "[WOLF_END]");
    
    // \\c[n] - Color codes (Wolf RPG uses double backslash format)
    // After JSON parsing, \\c[n] becomes \c[n] in memory, so we match single backslash
    let color_codes = Regex::new(r"\\c\[(\d+)\]").unwrap();
    result = color_codes.replace_all(&result, "[COLOR_$1]").to_string();
    
    // \\i[n] - Icon codes (Wolf RPG database files)
    let icon_codes = Regex::new(r"\\i\[(\d+)\]").unwrap();
    result = icon_codes.replace_all(&result, "[ICON_$1]").to_string();
    
    // \\f[n] - Font/Face codes (Wolf RPG database files)
    let font_codes = Regex::new(r"\\f\[(\d+)\]").unwrap();
    result = font_codes.replace_all(&result, "[FONT_$1]").to_string();
    
    // @n - Event/command markers (found in Wolf RPG text)
    let at_codes = Regex::new(r"@(\d+)").unwrap();
    result = at_codes.replace_all(&result, "[AT_$1]").to_string();
    
    // \\s[n] - Character/slot references (preserves exact formatting including leading zeros)
    let slot_codes = Regex::new(r"\\s\[(\d+)\]").unwrap();
    result = slot_codes.replace_all(&result, "[SLOT_$1]").to_string();
    
    // \\cself[n] - Self-referencing color codes
    let cself_codes = Regex::new(r"\\cself\[(\d+)\]").unwrap();
    result = cself_codes.replace_all(&result, "[CSELF_$1]").to_string();
    
    // \\r - Ruby text marker (the [kanji,reading] part is translatable text, not placeholder)
    // This just marks where ruby text starts, content inside [] should be translated
    result = result.replace("\\\\r", "[RUBY_START]");
    
    // Handle both actual carriage return characters (\r) and literal \r strings  
    // Actual carriage returns (from parsed JSON)
    result = result.replace('\r', "[CARRIAGE_RETURN]");
    // Literal \r strings (different from \\r ruby marker)
    result = result.replace("\\r", "[CARRIAGE_RETURN]");
    
    // Handle actual newline characters (from parsed JSON)
    // Wolf RPG only has actual newlines, not literal "\n" strings
    result = result.replace('\n', "[NEWLINE]");

    // Arg placeholders like %1 / ％１ → [ARG_1]
    let arg_any = Regex::new(r"[%％]([0-9０-９]+)").unwrap();
    result = arg_any
        .replace_all(&result, |caps: &Captures| {
            format!("[ARG_{}]", to_ascii_digits(&caps[1]))
        })
        .to_string();

    // Control sequences
    result = result.replace("\\.", "[CTRL_DOT]");
    result = result.replace("\\|", "[CTRL_WAIT]");
    result = result.replace("\\^", "[CTRL_INSTANT]");
    result = result.replace("\\!", "[CTRL_INPUT]");

    // Preserve significant whitespace (AFTER control codes to avoid conflicts)
    result = encode_whitespace(&result);
    result
}

/// Restore placeholders back to Wolf codes after translation.
pub fn wolf_restore_placeholders_after_translation(text: &str) -> String {
    let mut result = text.to_string();
    
    // Restore whitespace FIRST (reverse order of encoding)
    result = decode_whitespace(&result);
    
    // Restore Wolf RPG specific codes
    result = result.replace("[WOLF_END]", "\\E");
    
    // Restore color codes (use double backslash format for Wolf RPG)
    let color_restore = Regex::new(r"\[COLOR_(\d+)\]").unwrap();
    result = color_restore.replace_all(&result, "\\c[$1]").to_string();
    
    // Restore icon codes
    let icon_restore = Regex::new(r"\[ICON_(\d+)\]").unwrap();
    result = icon_restore.replace_all(&result, "\\\\i[$1]").to_string();
    
    // Restore font codes
    let font_restore = Regex::new(r"\[FONT_(\d+)\]").unwrap();
    result = font_restore.replace_all(&result, "\\\\f[$1]").to_string();
    
    // Restore @ codes
    let at_restore = Regex::new(r"\[AT_(\d+)\]").unwrap();
    result = at_restore.replace_all(&result, "@$1").to_string();
    
    // Restore slot codes
    let slot_restore = Regex::new(r"\[SLOT_(\d+)\]").unwrap();
    result = slot_restore.replace_all(&result, "\\\\s[$1]").to_string();
    
    // Restore cself codes
    let cself_restore = Regex::new(r"\[CSELF_(\d+)\]").unwrap();
    result = cself_restore.replace_all(&result, "\\\\cself[$1]").to_string();
    
    // Restore ruby text marker
    result = result.replace("[RUBY_START]", "\\\\r");
    
    // Restore carriage return
    result = result.replace("[CARRIAGE_RETURN]", "\\r");
    
    // Restore actual newlines (not literal strings)
    result = result.replace("[NEWLINE]", "\n");

    // Restore arg placeholders
    let arg = Regex::new(r"\[ARG_(\d+)\]").unwrap();
    result = arg.replace_all(&result, "%$1").to_string();

    // Restore control codes
    result = result.replace("[CTRL_DOT]", "\\.");
    result = result.replace("[CTRL_WAIT]", "\\|");
    result = result.replace("[CTRL_INSTANT]", "\\^");
    result = result.replace("[CTRL_INPUT]", "\\!");

    result
}

fn to_ascii_digits(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '０'..='９' => char::from_u32('0' as u32 + (c as u32 - '０' as u32)).unwrap(),
            d => d,
        })
        .collect()
}

fn encode_whitespace(input: &str) -> String {
    let mut result = input.to_string();
    // Full-width spaces → [FWSPC_n]
    let fw = Regex::new(r"(　+)").unwrap();
    result = fw
        .replace_all(&result, |caps: &Captures| {
            let n = caps[1].chars().count();
            format!("[FWSPC_{}]", n)
        })
        .to_string();

    // Leading spaces → [SPC_n]
    let leading = Regex::new(r"^( +)").unwrap();
    result = leading
        .replace(&result, |caps: &Captures| {
            let n = caps[1].len();
            format!("[SPC_{}]", n)
        })
        .to_string();

    // Trailing spaces → [SPC_n]
    let trailing = Regex::new(r"( +)$").unwrap();
    result = trailing
        .replace(&result, |caps: &Captures| {
            let n = caps[1].len();
            format!("[SPC_{}]", n)
        })
        .to_string();

    // Interior runs of ASCII spaces (>=2) → [SPC_n]
    let multi = Regex::new(r"( {2,})").unwrap();
    result = multi
        .replace_all(&result, |caps: &Captures| {
            let n = caps[1].len();
            format!("[SPC_{}]", n)
        })
        .to_string();

    // Tabs → [TAB_n]
    let tabs = Regex::new(r"(\t+)").unwrap();
    result = tabs
        .replace_all(&result, |caps: &Captures| {
            let n = caps[1].len();
            format!("[TAB_{}]", n)
        })
        .to_string();

    result
}

fn decode_whitespace(input: &str) -> String {
    let mut result = input.to_string();
    let fw = Regex::new(r"\[FWSPC_(\d+)\]").unwrap();
    result = fw
        .replace_all(&result, |caps: &Captures| {
            let n: usize = caps[1].parse().unwrap_or(0);
            "　".repeat(n)
        })
        .to_string();

    let spc = Regex::new(r"\[SPC_(\d+)\]").unwrap();
    result = spc
        .replace_all(&result, |caps: &Captures| {
            let n: usize = caps[1].parse().unwrap_or(0);
            " ".repeat(n)
        })
        .to_string();

    let tab = Regex::new(r"\[TAB_(\d+)\]").unwrap();
    result = tab
        .replace_all(&result, |caps: &Captures| {
            let n: usize = caps[1].parse().unwrap_or(0);
            "\t".repeat(n)
        })
        .to_string();
    result
}












