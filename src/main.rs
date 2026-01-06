use std::collections::HashMap;

struct Embeddings {
    vocab: HashMap<String, usize>,
    data: Vec<f32>,
    dim: usize
}

fn main() {
    let filename = "toy_embed.txt";

    let content: String = std::fs::read_to_string(filename)
        .expect("Failed to read the embeddings file");

    for line in content.lines() {

        let mut iter = line.split_whitespace();
        let word = iter.next().expect("missing word");
        let values: Vec<f32> = iter
            .map(|x| x.parse::<f32>().expect("failed to parse float"))
            .collect();

        println!("{word:?} -> {values:?}");
    }
}
