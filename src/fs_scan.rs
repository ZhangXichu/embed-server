use regex::Regex;
use anyhow::{Context, Result};
use std::path::Path;

fn normalize_ws(s: &str) -> String {
    // Collapses all whitespace to single spaces.
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Chunks text extracted from a PDF file located at `path` into segments
/// not exceeding `max_chars` in length. Sentences are determined by
/// punctuation marks (., !, ?).
pub fn chunk_pdf_text(path: &Path, max_chars: usize) -> Result<()> {
    let path = path
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {}", path.display()))?;

    let text = pdf_extract::extract_text(&path)
        .with_context(|| format!("Failed to extract pdf text: {}", path.display()))?;

    // println!("{}", text);

    let re: Regex = Regex::new(r#"(?s)(.+?)([.!?]+)(\s+|$)"#).unwrap();

    let mut last_end = 0usize;

    let mut counter = 0usize;

    for caps in re.captures_iter(&text) {
        let m = caps.get(0).unwrap();
        last_end = m.end();

        let body = caps.get(1).unwrap().as_str();
        let punct = caps.get(2).unwrap().as_str();

        let mut sentence = normalize_ws(body);
        if sentence.is_empty() {
            continue;
        }
        sentence.push_str(punct);

        //////// debug ////////////////
        if !sentence.trim().is_empty() {
            print!("{}. {} \n", counter, sentence);
            counter += 1;
        }
        ////////////////////////////////
    }

    // Remainder (no ending punctuation)
    let rem = text[last_end..].trim();
    if !rem.is_empty() {
        print!("Remainder: {} \n", rem);
    }

    Ok(())
}


fn hard_split(s: &str, max_chars: usize) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0usize;
    let mut count = 0usize;

    for (i, _) in s.char_indices() {
        count += 1;
        if count > max_chars {
            parts.push(&s[start..i]);
            start = i;
            count = 1;
        }
    }
    if start < s.len() {
        parts.push(&s[start..]);
    }
    parts
}


#[cfg(test)]
mod tests {
    use super::hard_split;

    fn char_len(s: &str) -> usize {
        s.chars().count()
    }

    #[test]
    fn hard_split_long_sentence_ascii() {
        let s = "In the Sicilian Defense, Black immediately fights for control of the center \
                 by advancing the c-pawn, creating an asymmetrical structure that often leads \
                 to sharp, unbalanced middlegames where precise calculation matters.";
        let max = 40;

        let parts = hard_split(s, max);

        assert!(!parts.is_empty(), "should produce at least one part");

        for (i, p) in parts.iter().enumerate() {
            assert!(
                !p.is_empty(),
                "part #{i} should be non-empty"
            );
            assert!(
                char_len(p) <= max,
                "part #{i} too long: {} > {max}",
                char_len(p)
            );
        }

        let joined = parts.concat();
        assert_eq!(joined, s, "joining parts should reconstruct original");
    }

    #[test]
    fn hard_split_very_long_sentence_reconstructs_exactly() {
        let s = "This is a deliberately long sentence designed to exceed typical chunk limits, \
                 containing multiple clauses, commas, and descriptive phrases so that we can \
                 validate that hard_split preserves every character in the original string, \
                 in the correct order, without dropping or duplicating anything.";
        let max = 25;

        let parts = hard_split(s, max);

        // expect multiple pieces
        assert!(parts.len() >= 3, "should split into multiple parts");

        let joined = parts.concat();
        assert_eq!(joined, s);

        // sanity: first part is exactly max chars unless the string is shorter
        assert_eq!(char_len(parts[0]), max.min(char_len(s)));
    }

    #[test]
    fn hard_split_handles_unicode_safely() {
        // Contains Unicode characters (em dash, accented, CJK, emoji)
        let s = "Unicode test — naïve café 中文測試 🙂. \
                 We want to ensure splitting respects UTF-8 boundaries and preserves content.";
        let max = 20;

        let parts = hard_split(s, max);

        assert!(!parts.is_empty());

        for (i, p) in parts.iter().enumerate() {
            assert!(
                char_len(p) <= max,
                "unicode part #{i} too long: {} > {max}",
                char_len(p)
            );
        }

        let joined = parts.concat();
        assert_eq!(joined, s);
    }

    #[test]
    fn hard_split_max_chars_one_splits_into_single_chars() {
        let s = "Longer sentence to test max_chars=1 behavior!";
        let parts = hard_split(s, 1);

        assert_eq!(parts.len(), char_len(s));
        for p in &parts {
            assert_eq!(char_len(p), 1);
        }

        assert_eq!(parts.concat(), s);
    }
}