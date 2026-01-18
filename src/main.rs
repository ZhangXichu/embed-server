mod embeddings;

fn main() {
    let path = std::path::Path::new("data/cc.en.300.vec");
    let embeddings = embeddings::Embeddings::load_txt(path);

    // let tokens = embeddings::tokenize("This is an example query.");
    // println!("parsed tokens {:?}", tokens);
}
