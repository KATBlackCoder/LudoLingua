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

    // Remove trailing artifacts that are clearly LLM commentary
    // Only remove these if they appear at the end of the text
    let trailing_artifacts = [
        "Note:",
        "P.S.:",
        "Explanation:",
        "Note:",
        "P.S.:",
        "Explanation:",
        "Note:",
        "P.S.:",
        "Explanation:",
    ];
    
    for artifact in &trailing_artifacts {
        if cleaned.ends_with(artifact) {
            cleaned = cleaned.trim_end_matches(artifact).trim().to_string();
        }
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

/// Enhanced LLM output cleaning with support for multiple structured tag patterns
/// 
/// This function handles different LLM provider formats and removes various
/// structured tags while preserving game content.
pub fn clean_llm_output_advanced(content: &str) -> String {
    let mut cleaned = content.to_string();

    // Remove various thinking/processing tags from different LLM providers
    let thinking_patterns = [
        (r"<think>.*?</think>", ""),
        (r"<thinking>.*?</thinking>", ""),
        (r"<reasoning>.*?</reasoning>", ""),
        (r"<analysis>.*?</analysis>", ""),
        (r"<process>.*?</process>", ""),
    ];

    for (pattern, replacement) in &thinking_patterns {
        let regex = Regex::new(pattern).unwrap();
        cleaned = regex.replace_all(&cleaned, *replacement).to_string();
    }

    // Remove input/output markers
    let io_patterns = [
        "<<<INPUT_START>>>",
        "<<<INPUT_END>>>",
        "<<<OUTPUT_START>>>",
        "<<<OUTPUT_END>>>",
        "```input",
        "```output",
        "```translation",
    ];

    for pattern in &io_patterns {
        cleaned = cleaned.replace(pattern, "");
    }

    // Remove trailing commentary artifacts
    let trailing_patterns = [
        r"Note:.*$",
        r"P\.S\..*$",
        r"Explanation:.*$",
        r"Comment:.*$",
        r"Note:.*$",
        r"P\.S\..*$",
        r"Explanation:.*$",
        r"Comment:.*$",
    ];

    for pattern in &trailing_patterns {
        let regex = Regex::new(pattern).unwrap();
        cleaned = regex.replace(&cleaned, "").to_string();
    }

    // Final cleanup with whitespace normalization
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
