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
    /// Returns a reference to the V1 delta payload.
    pub const fn as_v1(&self) -> &OpFlashblockExecutionPayloadDeltaV1 {
        match self {
            Self::V1(delta) => delta,
        }
    }

    /// Returns a mutable reference to the V1 delta payload.
    pub const fn as_v1_mut(&mut self) -> &mut OpFlashblockExecutionPayloadDeltaV1 {
        match self {
            Self::V1(delta) => delta,
        }
    }

    /// Consumes self and returns the V1 delta payload.
    pub fn into_v1(self) -> OpFlashblockExecutionPayloadDeltaV1 {
        match self {
            Self::V1(delta) => delta,
        }
    }

    /// Returns the state root.
    pub const fn state_root(&self) -> B256 {
        self.as_v1().state_root
    }

    /// Returns the receipts root.
    pub const fn receipts_root(&self) -> B256 {
        self.as_v1().receipts_root
    }

    /// Returns a reference to the logs bloom.
    pub const fn logs_bloom(&self) -> &Bloom {
        &self.as_v1().logs_bloom
    }

    /// Returns the gas used.
    pub const fn gas_used(&self) -> u64 {
        self.as_v1().gas_used
    }

    /// Returns the block hash.
    pub const fn block_hash(&self) -> B256 {
        self.as_v1().block_hash
    }

    /// Returns a reference to the transactions.
    pub const fn transactions(&self) -> &Vec<Bytes> {
        &self.as_v1().transactions
    }

    /// Returns a reference to the withdrawals.
    pub const fn withdrawals(&self) -> &Vec<Withdrawal> {
        &self.as_v1().withdrawals
    }

    /// Returns the withdrawals root.
    pub const fn withdrawals_root(&self) -> B256 {
        self.as_v1().withdrawals_root
    }

    /// Returns the blob gas used.
    pub const fn blob_gas_used(&self) -> u64 {
        self.as_v1().blob_gas_used
    }
}

impl<'a> From<OpFlashblockExecutionPayloadDeltaRef<'a>> for OpFlashblockExecutionPayloadDelta {
    fn from(delta: OpFlashblockExecutionPayloadDeltaRef<'a>) -> Self {
        match delta {
            OpFlashblockExecutionPayloadDeltaRef::V1(v1) => Self::V1(v1.clone()),
        }
    }
}

/// Borrowed reference to execution payload delta.
///
/// This enum allows for future versioning of flashblock execution payload delta types
/// while providing zero-cost access to the inner fields via [`Deref`](core::ops::Deref).
#[derive(Debug, Clone, Copy)]
pub enum OpFlashblockExecutionPayloadDeltaRef<'a> {
    /// Version 1 execution payload delta reference.
    V1(&'a OpFlashblockExecutionPayloadDeltaV1),
}

impl<'a> core::ops::Deref for OpFlashblockExecutionPayloadDeltaRef<'a> {
    type Target = OpFlashblockExecutionPayloadDeltaV1;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::V1(inner) => inner,
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
    /// The estimated cumulative blob gas used for the block. Introduced in Jovian.
    /// spec: <https://docs.optimism.io/notices/upgrade-17#block-header-changes>
    /// Defaults to 0 if not present (for pre-Jovian blocks).
    #[cfg_attr(feature = "serde", serde(default))]
    pub blob_gas_used: u64,
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
            blob_gas_used: 0,
        };

        let json = serde_json::to_string(&delta).unwrap();
        let decoded: OpFlashblockExecutionPayloadDeltaV1 = serde_json::from_str(&json).unwrap();
        assert_eq!(delta, decoded);
    }

    #[test]
    fn test_delta_snake_case_serialization() {
        let delta = OpFlashblockExecutionPayloadDeltaV1 {
            state_root: B256::ZERO,
            receipts_root: B256::ZERO,
            logs_bloom: Bloom::ZERO,
            gas_used: 0,
            block_hash: B256::ZERO,
            transactions: vec![],
            withdrawals: vec![],
            withdrawals_root: B256::ZERO,
            blob_gas_used: 0,
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
            blob_gas_used: 0,
        };

        let json = serde_json::to_string(&delta).unwrap();
        let decoded: OpFlashblockExecutionPayloadDeltaV1 = serde_json::from_str(&json).unwrap();
        assert_eq!(delta.withdrawals.len(), 1);
        assert_eq!(decoded.withdrawals.len(), 1);
    }
}
