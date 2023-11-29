//! Example binary

fn main() {
    let text = std::fs::read("README.md").unwrap();
    let compressed = snappy_rs::compress(&text).unwrap();
    println!(
        "Compression ratio: {:.1}%",
        100.0 * compressed.len() as f64 / text.len() as f64
    );
}
