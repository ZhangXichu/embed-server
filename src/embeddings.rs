use std::{
    collections::HashMap,
    io::{self, ErrorKind},
    path::Path,
};

pub struct Embeddings {
    vocab: HashMap<String, usize>,
    data: Vec<f32>,
    dim: usize,
}

impl Embeddings {

    /// Load embeddings from a text file in the following format:
    /// word1 v1 v2 v3 ...
    /// word2 v1 v2 v3 ...
    /// ...
    /// The first line can optionally contain the number of words and the dimension,
    /// but it will be ignored in this implementation.
    pub fn load_txt(filepath: &Path) -> Self {
        let content: String =
            std::fs::read_to_string(filepath).expect("Failed to read the embeddings file");

        let mut embeddings = Embeddings {
            vocab: HashMap::new(),
            data: Vec::new(),
            dim: 0,
        };

        let mut word_counter: usize = 0;

        for (idx, line) in content.lines().enumerate() {
            let mut iter = line.split_whitespace();

            let first = match iter.next() {
                Some(v) => v,
                None => continue,
            };

            // ---- fastText header: "N D" on first line ----
            if idx == 0 {
                let second = iter.next();
                if let (Some(n), Some(d)) = (first.parse::<usize>().ok(), second.and_then(|s| s.parse::<usize>().ok())) {
                    embeddings.dim = d;
                    assert!(embeddings.dim > 0, "embedding dimension must be > 0");
                    println!("Header detected: {} words, dim = {}", n, d);
                    continue; // skip header line
                }
                // else: fall through → first line is a normal embedding
            }

            let word = first;

            let values: Vec<f32> = iter
                .map(|x| x.parse::<f32>().expect("failed to parse float"))
                .collect();

            if values.is_empty() {
                continue;
            }

            assert!(
                values.len() == embeddings.dim,
                "embedding dimension mismatch for word {:?}, at line {}: expected {}, got {}",
                &word,
                idx,
                embeddings.dim,
                values.len()
            );

            embeddings.vocab.insert(word.to_string(), word_counter);
            embeddings.data.extend_from_slice(&values);

            if word_counter == 0 {
                println!("word '{}' -> {:?}", word, &values);
            }

            word_counter += 1;
        }

        embeddings
    }

    pub fn embed_tokens<'a>(
            tokens: impl IntoIterator<Item = &'a str>
        ) -> Option<Vec<f32>> {

        Option::None
    }

}

pub fn tokenize(query: &str) -> Vec<String> {
    query
        .to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_txt() {
        let path = Path::new("toy_embed.txt");
        let emb = Embeddings::load_txt(path);

        assert_eq!(emb.dim, 4);

        assert_eq!(emb.vocab.len(), 3);
        assert_eq!(emb.vocab.get("king"), Some(&0));
        assert_eq!(emb.vocab.get("queen"), Some(&1));
        assert_eq!(emb.vocab.get("man"), Some(&2));

        let expected: Vec<f32> = vec![
            // king
            0.1, 0.2, 0.3, 0.4, // queen
            1.0, 2.0, 3.0, 4.0, // man
            -0.5, 0.0, 0.5, 1.5,
        ];

        assert_eq!(emb.data.len(), expected.len());

        for (i, (a, b)) in emb.data.iter().zip(expected.iter()).enumerate() {
            assert!((a - b).abs() < 1e-6, "mismatch at index {i}: {a} vs {b}");
        }
    }
}
