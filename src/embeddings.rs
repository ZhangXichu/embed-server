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

/// Load embeddings from a text file in the following format:
/// word1 v1 v2 v3 ...
/// word2 v1 v2 v3 ...
/// ...
/// The first line can optionally contain the number of words and the dimension,
/// but it will be ignored in this implementation.
pub fn load_txt(filepath: &Path) -> Embeddings {
    let content: String =
        std::fs::read_to_string(filepath).expect("Failed to read the embeddings file");

    let mut embeddings = Embeddings {
        vocab: HashMap::new(),
        data: Vec::new(),
        dim: 0,
    };

    for (idx, line) in content.lines().enumerate() {
        let mut iter = line.split_whitespace();
        let word = iter.next().expect("missing word");
        let values: Vec<f32> = iter
            .map(|x| x.parse::<f32>().expect("failed to parse float"))
            .collect();

        if values.is_empty() {
            panic!("empty word");
        }

        if idx == 1 {
            embeddings.dim = values.len();

            println!(
                "Dimension: {}. {:?} -> {:?}",
                embeddings.dim, &word, &values
            );
        } else if idx > 1 {
            // skip first line
            assert!(
                values.len() == embeddings.dim,
                "embedding dimension mismatch at word {:?}: expected {}, got {}",
                &word,
                embeddings.dim,
                values.len()
            );
        }

        embeddings.vocab.insert(word.to_string(), idx);
        embeddings.data.extend_from_slice(&values);
    }

    embeddings
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
        let emb = load_txt(path);

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
