//! Flashblock delta execution payload types.

use alloc::vec::Vec;
use alloy_eips::eip4895::Withdrawal;
use alloy_primitives::{B256, Bloom, Bytes};

/// Execution payload delta envelope.
///
/// This enum allows for future versioning of flashblock execution payload delta types.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OpFlashblockExecutionPayloadDelta {
    /// Version 1 execution payload delta.
    V1(OpFlashblockExecutionPayloadDeltaV1),
}

impl OpFlashblockExecutionPayloadDelta {
    /// Returns the state root.
    pub const fn state_root(&self) -> B256 {
        match self {
            Self::V1(delta) => delta.state_root,
        }
    }

    /// Returns the receipts root.
    pub const fn receipts_root(&self) -> B256 {
        match self {
            Self::V1(delta) => delta.receipts_root,
        }
    }

    /// Returns the logs bloom.
    pub const fn logs_bloom(&self) -> Bloom {
        match self {
            Self::V1(delta) => delta.logs_bloom,
        }
    }

    /// Returns the gas used.
    pub const fn gas_used(&self) -> u64 {
        match self {
            Self::V1(delta) => delta.gas_used,
        }
    }

    /// Returns the block hash.
    pub const fn block_hash(&self) -> B256 {
        match self {
            Self::V1(delta) => delta.block_hash,
        }
    }

    /// Returns the transactions.
    pub fn transactions(&self) -> &[Bytes] {
        match self {
            Self::V1(delta) => &delta.transactions,
        }
    }

    /// Returns the withdrawals.
    pub fn withdrawals(&self) -> &[Withdrawal] {
        match self {
            Self::V1(delta) => &delta.withdrawals,
        }
    }

    /// Returns the withdrawals root.
    pub const fn withdrawals_root(&self) -> B256 {
        match self {
            Self::V1(delta) => delta.withdrawals_root,
        }
    }
}

impl From<OpFlashblockExecutionPayloadDeltaV1> for OpFlashblockExecutionPayloadDelta {
    fn from(delta: OpFlashblockExecutionPayloadDeltaV1) -> Self {
        Self::V1(delta)
    }
}

/// Represents the modified portions of an execution payload within a flashblock.
/// This structure contains only the fields that can be updated during block construction,
/// such as state root, receipts, logs, and new transactions. Other immutable block fields
/// like parent hash and block number are excluded since they remain constant throughout
/// the block's construction.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpFlashblockExecutionPayloadDeltaV1 {
    /// The state root of the block.
    pub state_root: B256,
    /// The receipts root of the block.
    pub receipts_root: B256,
    /// The logs bloom of the block.
    pub logs_bloom: Bloom,
    /// The gas used of the block.
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity"))]
    pub gas_used: u64,
    /// The block hash of the block.
    pub block_hash: B256,
    /// The transactions of the block.
    pub transactions: Vec<Bytes>,
    /// Array of [`Withdrawal`] enabled with V2
    pub withdrawals: Vec<Withdrawal>,
    /// The withdrawals root of the block.
    pub withdrawals_root: B256,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn test_delta_serde_roundtrip() {
        let delta = OpFlashblockExecutionPayloadDeltaV1 {
            state_root: B256::random(),
            receipts_root: B256::random(),
            logs_bloom: Bloom::default(),
            gas_used: 21_000,
            block_hash: B256::random(),
            transactions: vec![Bytes::from(vec![1, 2, 3])],
            withdrawals: vec![],
            withdrawals_root: B256::random(),
        };

        let json = serde_json::to_string(&delta).unwrap();
        let decoded: OpFlashblockExecutionPayloadDeltaV1 = serde_json::from_str(&json).unwrap();
        assert_eq!(delta, decoded);
    }

    #[test]
    fn test_delta_camel_case_serialization() {
        let delta = OpFlashblockExecutionPayloadDeltaV1 {
            state_root: B256::ZERO,
            receipts_root: B256::ZERO,
            logs_bloom: Bloom::ZERO,
            gas_used: 0,
            block_hash: B256::ZERO,
            transactions: vec![],
            withdrawals: vec![],
            withdrawals_root: B256::ZERO,
        };

        let json = serde_json::to_string(&delta).unwrap();
        assert!(json.contains("state_root"));
        assert!(json.contains("receipts_root"));
        assert!(json.contains("logs_bloom"));
        assert!(json.contains("gas_used"));
        assert!(json.contains("block_hash"));
        assert!(json.contains("withdrawals_root"));
    }

    #[test]
    fn test_delta_with_withdrawals() {
        let withdrawal = Withdrawal {
            index: 0,
            validator_index: 1,
            address: alloy_primitives::Address::ZERO,
            amount: 1000,
        };

        let delta = OpFlashblockExecutionPayloadDeltaV1 {
            state_root: B256::ZERO,
            receipts_root: B256::ZERO,
            logs_bloom: Bloom::ZERO,
            gas_used: 0,
            block_hash: B256::ZERO,
            transactions: vec![],
            withdrawals: vec![withdrawal],
            withdrawals_root: B256::ZERO,
        };

        let json = serde_json::to_string(&delta).unwrap();
        let decoded: OpFlashblockExecutionPayloadDeltaV1 = serde_json::from_str(&json).unwrap();
        assert_eq!(delta.withdrawals.len(), 1);
        assert_eq!(decoded.withdrawals.len(), 1);
    }
}
