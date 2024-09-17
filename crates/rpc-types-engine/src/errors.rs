//! Error types for conversions between Optimism Execution Payload Envelope
//! types and external types.

use alloy_eips::eip2718::Eip2718Error;
use alloy_primitives::B256;
use op_alloy_protocol::block_info::DecodeError;

/// An error that can occur when converting an [OptimismExecutionPayloadEnvelopeV4] to an
/// [L2BlockInfo].
#[derive(Debug)]
pub enum ToL2BlockRefError {
    /// The genesis block hash does not match the expected value.
    InvalidGenesisHash,
    /// The L2 block is missing the L1 info deposit transaction.
    MissingL1InfoDeposit(B256),
    /// The first payload transaction has an unexpected type.
    UnexpectedTxType(u8),
    /// Failed to decode the first transaction into an [OpTxEnvelope].
    TxEnvelopeDecodeError(Eip2718Error),
    /// The first payload transaction is not a deposit transaction.
    FirstTxNonDeposit(u8),
    /// Failed to decode the [L1BlockInfoTx] from the deposit transaction.
    BlockInfoDecodeError(DecodeError),
}

#[cfg(feature = "std")]
impl std::error::Error for ToL2BlockRefError {}

impl core::fmt::Display for ToL2BlockRefError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ToL2BlockRefError::InvalidGenesisHash => write!(f, "Invalid genesis hash"),
            ToL2BlockRefError::MissingL1InfoDeposit(block_hash) => {
                write!(f, "L2 block is missing L1 info deposit transaction ({})", block_hash)
            }
            ToL2BlockRefError::UnexpectedTxType(ty) => {
                write!(f, "First payload transaction has unexpected type: {}", ty)
            }
            ToL2BlockRefError::TxEnvelopeDecodeError(t) => {
                write!(f, "Failed to decode the first transaction into an OpTxEnvelope: {:?}", t)
            }
            ToL2BlockRefError::FirstTxNonDeposit(ty) => {
                write!(f, "First payload transaction is not a deposit transaction, type: {}", ty)
            }
            ToL2BlockRefError::BlockInfoDecodeError(t) => write!(
                f,
                "Failed to decode the L1BlockInfoTx from the deposit transaction: {:?}",
                t
            ),
        }
    }
}

/// An error that can occur when converting an [OptimismExecutionPayloadEnvelopeV4] to a
/// [SystemConfig].
#[derive(Debug)]
pub enum ToSystemConfigError {
    /// The genesis block hash does not match the expected value.
    InvalidGenesisHash,
    /// The L2 block is missing the L1 info deposit transaction.
    MissingL1InfoDeposit(B256),
    /// The first payload transaction has an unexpected type.
    UnexpectedTxType(u8),
    /// Failed to decode the first transaction into an [OpTxEnvelope].
    TxEnvelopeDecodeError(Eip2718Error),
    /// The first payload transaction is not a deposit transaction.
    FirstTxNonDeposit(u8),
    /// Failed to decode the [L1BlockInfoTx] from the deposit transaction.
    BlockInfoDecodeError(DecodeError),
    /// Missing system config in the genesis block.
    MissingSystemConfig,
}

#[cfg(feature = "std")]
impl std::error::Error for ToSystemConfigError {}

impl core::fmt::Display for ToSystemConfigError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ToSystemConfigError::InvalidGenesisHash => write!(f, "Invalid genesis hash"),
            ToSystemConfigError::MissingL1InfoDeposit(block_hash) => {
                write!(f, "L2 block is missing L1 info deposit transaction ({})", block_hash)
            }
            ToSystemConfigError::UnexpectedTxType(ty) => {
                write!(f, "First payload transaction has unexpected type: {}", ty)
            }
            ToSystemConfigError::TxEnvelopeDecodeError(t) => {
                write!(f, "Failed to decode the first transaction into an OpTxEnvelope: {:?}", t)
            }
            ToSystemConfigError::FirstTxNonDeposit(ty) => {
                write!(f, "First payload transaction is not a deposit transaction, type: {}", ty)
            }
            ToSystemConfigError::BlockInfoDecodeError(t) => write!(
                f,
                "Failed to decode the L1BlockInfoTx from the deposit transaction: {:?}",
                t
            ),
            ToSystemConfigError::MissingSystemConfig => {
                write!(f, "Missing system config in the genesis block")
            }
        }
    }
}
