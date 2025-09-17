//! # Storage Compression
//!
//! This module provides compression utilities for efficient storage
//! of analysis results with configurable compression algorithms.

use anyhow::Result;

/// Compression utilities for storage optimization
pub struct CompressionManager {
    /// Compression algorithm to use
    algorithm: CompressionAlgorithm,
}

/// Supported compression algorithms
#[derive(Debug, Clone)]
pub enum CompressionAlgorithm {
    /// Gzip compression
    Gzip,
    /// No compression
    None,
}

impl CompressionManager {
    /// Create a new compression manager
    pub fn new(algorithm: CompressionAlgorithm) -> Self {
        Self { algorithm }
    }

    /// Compress data
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.algorithm {
            CompressionAlgorithm::Gzip => {
                use flate2::write::GzEncoder;
                use flate2::Compression;
                use std::io::Write;

                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(data)?;
                Ok(encoder.finish()?)
            }
            CompressionAlgorithm::None => Ok(data.to_vec()),
        }
    }

    /// Decompress data
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.algorithm {
            CompressionAlgorithm::Gzip => {
                use flate2::read::GzDecoder;
                use std::io::Read;

                let mut decoder = GzDecoder::new(data);
                let mut result = Vec::new();
                decoder.read_to_end(&mut result)?;
                Ok(result)
            }
            CompressionAlgorithm::None => Ok(data.to_vec()),
        }
    }
}
