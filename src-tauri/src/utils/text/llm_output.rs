use regex::Regex;

/// Smart LLM output cleaning and extraction
///
/// This function removes LLM thinking blocks, input/output tags, and other artifacts
/// while preserving legitimate game content that might contain similar patterns.
pub fn clean_llm_output(content: &str) -> String {
    let content = content.trim();

    // Remove all occurrences of <<<INPUT_START>>> and <<<INPUT_END>>> tags
    let mut cleaned = content
        .replace("<<<INPUT_START>>>", "")
        .replace("<<<INPUT_END>>>", "");

    // Remove everything between <think> and </think> tags (including the tags)
    while let Some(start_idx) = cleaned.find("<think>") {
        if let Some(end_idx) = cleaned.find("</think>") {
            if start_idx < end_idx {
                // Remove the entire <think>...</think> block
                cleaned.replace_range(start_idx..end_idx + 8, "");
            } else {
                // Malformed tags, just remove the opening tag
                cleaned.replace_range(start_idx..start_idx + 7, "");
            }
        } else {
            // No closing tag found, just remove the opening tag
            cleaned.replace_range(start_idx..start_idx + 7, "");
        }
    }

    // Remove everything between <thinking> and </thinking> tags (including the tags)
    while let Some(start_idx) = cleaned.find("<thinking>") {
        if let Some(end_idx) = cleaned.find("</thinking>") {
            if start_idx < end_idx {
                // Remove the entire <thinking>...</thinking> block
                cleaned.replace_range(start_idx..end_idx + 11, "");
            } else {
                // Malformed tags, just remove the opening tag
                cleaned.replace_range(start_idx..start_idx + 10, "");
            }
        } else {
            // No closing tag found, just remove the opening tag
            cleaned.replace_range(start_idx..start_idx + 10, "");
        }
    }

    // Remove parenthetical notes like "(Note: ...)" first
    let parenthetical_note = Regex::new(r"\([^)]*[Nn]ote:[^)]*\)").unwrap();
    cleaned = parenthetical_note.replace_all(&cleaned, "").to_string();

    // Remove LLM commentary artifacts (case-insensitive)
    let commentary_patterns = [
        r"(?i)note:\s*.*$",
        r"(?i)p\.s\.:\s*.*$",
        r"(?i)explanation:\s*.*$",
        r"(?i)comment:\s*.*$",
    ];

    for pattern in &commentary_patterns {
        let regex = Regex::new(pattern).unwrap();
        cleaned = regex.replace_all(&cleaned, "").to_string();
    }

    // Clean up any remaining whitespace and newlines
    cleaned = cleaned
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();

    // Quality validation: ensure cleaning doesn't remove all content
    if cleaned.is_empty() {
        content.to_string()
    } else {
        cleaned
    }
}
