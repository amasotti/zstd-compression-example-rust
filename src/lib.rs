//! Lib file with auxiliary methods for invoking zstd compression and random string generation.

pub mod compressor {
    use zstd::stream::{ copy_encode, copy_decode };
    use std::io::Cursor;

    /// Struct for zstd compression operations.
    pub struct ZstdCompressor;

    impl ZstdCompressor {
        /// Compresses the input data using zstd algorithm.
        ///
        /// # Arguments
        ///
        /// * `data` - A byte slice containing the data to be compressed.
        ///
        /// # Returns
        ///
        /// A `Vec<u8>` containing the compressed data.
        pub fn compress(data: &[u8]) -> Vec<u8> {
            let mut compressed = Vec::new();
            copy_encode(Cursor::new(data), &mut compressed, 3).unwrap();
            compressed
        }

        /// Decompresses the input data using zstd algorithm.
        ///
        /// # Arguments
        ///
        /// * `compressed_data` - A byte slice containing the compressed data.
        /// * `original_size` - The size of the original, uncompressed data.
        ///
        /// # Returns
        ///
        /// A `Vec<u8>` containing the decompressed data.
        pub fn decompress(compressed_data: &[u8], original_size: usize) -> Vec<u8> {
            let mut decompressed = Vec::with_capacity(original_size);
            copy_decode(Cursor::new(compressed_data), &mut decompressed).unwrap();
            decompressed
        }
    }
}

/// Module for generating random strings.
pub mod string_generator {
    use rand::{distributions::Alphanumeric, Rng};

    pub struct StringGenerator;

    impl StringGenerator {
        /// Generates a random alphanumeric string of the specified length.
        ///
        /// # Arguments
        ///
        /// * `length` - The desired length of the generated string.
        ///
        /// # Returns
        ///
        /// A `String` containing random alphanumeric characters.
        pub fn generate_string(length: usize) -> String {
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_decompression() {
        let original = string_generator::StringGenerator::generate_string(100_000);
        let compressed = compressor::ZstdCompressor::compress(original.as_bytes());
        let decompressed = compressor::ZstdCompressor::decompress(&compressed, original.len());
        assert_eq!(original.as_bytes(), decompressed);
    }

    #[test]
    fn test_string_generator() {
        let length = 100;
        let generated = string_generator::StringGenerator::generate_string(length);
        assert_eq!(generated.len(), length);
        assert!(generated.chars().all(|c| c.is_ascii_alphanumeric()));
    }
}
