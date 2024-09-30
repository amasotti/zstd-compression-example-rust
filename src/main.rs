use zstd_compression_example_rust::compressor::ZstdCompressor;
use zstd_compression_example_rust::string_generator::StringGenerator;

fn main() {
    let test_lengths = vec![10, 50, 100, 500, 1000, 5000, 10000, 50000,500_000];

    for length in test_lengths {
        let original_string = StringGenerator::generate_string(length);
        let original_bytes = original_string.as_bytes();
        let compressed_bytes = ZstdCompressor::compress(original_bytes);

        let compression_ratio = compressed_bytes.len() as f64 / original_bytes.len() as f64;

        println!(
            "Original length: {}, Compressed length: {}, Ratio: {:.2}",
            original_bytes.len(),
            compressed_bytes.len(),
            compression_ratio
        );

        let decompressed_bytes = ZstdCompressor::decompress(&compressed_bytes, original_bytes.len());
        assert_eq!(original_bytes, decompressed_bytes, "Decompression failed");
    }

    println!("All tests passed successfully!");
}
