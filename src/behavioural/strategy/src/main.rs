mod compressor;

use compressor::*;

fn main() {
    let mut compressor = Compressor::new(Box::new(SimpleCompression));
    let input = "aaabbc";

    let compressed = compressor.compress(input);
    println!("Compressed: {}", compressed);
    let decompressed = compressor.decompress(&compressed);
    println!("Decompressed: {}", decompressed);

    compressor.set_strategy(Box::new(NoCompression));
    let compressed = compressor.compress(input);
    println!("Compressed: {}", compressed);
    let decompressed = compressor.decompress(&compressed);
    println!("Decompressed: {}", decompressed);
}
