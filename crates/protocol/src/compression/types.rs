//! Compression types.

use crate::BrotliLevel;
use alloc::borrow::Borrow;

/// The type of compressor to use.
///
/// See: <https://github.com/ethereum-optimism/optimism/blob/042433b89ce38ccc15456e9673829f6783bb97ac/op-batcher/compressor/compressors.go#L20>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressorType {
    /// The ratio compression.
    Ratio,
    /// The shadow compression.
    Shadow,
}

/// The compression algorithm type.
///
/// See:
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgo {
    /// The fastest brotli compression level.
    Brotli9,
    /// The default brotli compression level.
    Brotli10,
    /// The best brotli compression level.
    Brotli11,
    /// The zlib compression.
    Zlib,
}

impl<A: Borrow<CompressionAlgo>> From<A> for BrotliLevel {
    fn from(algo: A) -> Self {
        match algo.borrow() {
            CompressionAlgo::Brotli9 => Self::Brotli9,
            CompressionAlgo::Brotli11 => Self::Brotli11,
            _ => Self::Brotli10,
        }
    }
}
