mod embeddings;
mod fs_scan;
mod data_saver;

fn main() {
    let path = std::path::Path::new("data/cc.en.300.vec");
    // let embeddings = embeddings::Embeddings::load_txt(path);

    // let tokens = embeddings::tokenize("This is an example query.");
    // println!("parsed tokens {:?}", tokens);

    let pdf_patth = std::path::Path::new("data/text/Foucault Subject and Power.pdf");
    fs_scan::chunk_pdf_text(pdf_patth, 900).unwrap();
}
