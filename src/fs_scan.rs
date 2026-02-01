use regex::Regex;
use anyhow::{Context, Result};
use std::path::Path;

pub fn process_pdf(path: &Path, max_chars: usize) -> Result<()> {
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

        let sentence = format!("{}{}", body.trim(), punct);
        if !sentence.trim().is_empty() {
            print!("{}. {} \n", counter, sentence);
            counter += 1;
        }
    }

    // Remainder (no ending punctuation)
    let rem = text[last_end..].trim();
    if !rem.is_empty() {
        print!("{} \n", rem);
    }

    Ok(())
}

