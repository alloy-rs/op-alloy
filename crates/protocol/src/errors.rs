//! Error types for protocol conversions.

use crate::DecodeError;
use alloy_primitives::B256;

/// An error encountered during [OpBlock] conversion.
///
/// [OpBlock]: op_alloy_consensus::OpBlock
#[derive(Debug, thiserror::Error)]
pub enum OpBlockConversionError {
    /// Invalid genesis hash.
    #[error("Invalid genesis hash. Expected {0}, got {1}")]
    InvalidGenesisHash(B256, B256),
    /// Invalid transaction type.
    #[error("First payload transaction has unexpected type: {0}")]
    InvalidTxType(u8),
    /// L1 Info error
    #[error("Failed to decode L1 info: {0}")]
    L1InfoError(#[from] DecodeError),
    /// Missing system config in genesis block.
    #[error("Missing system config in genesis block")]
    MissingSystemConfigGenesis,
    /// Empty transactions.
    #[error("Empty transactions in payload. Block hash: {0}")]
    EmptyTransactions(B256),
    /// EIP-1559 parameter decoding error.
    #[error("Failed to decode EIP-1559 parameters from header's `nonce` field.")]
    Eip1559DecodeError,
}
