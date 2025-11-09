//! Flashblock base execution payload types.

use alloy_primitives::{Address, B256, Bytes, U256};

/// Execution payload base envelope.
///
/// This enum allows for future versioning of flashblock execution payload base types.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OpFlashblockExecutionPayloadBase {
    /// Version 1 execution payload base.
    V1(OpFlashblockExecutionPayloadBaseV1),
}

impl OpFlashblockExecutionPayloadBase {
    /// Returns a reference to the V1 base payload.
    pub const fn as_v1(&self) -> &OpFlashblockExecutionPayloadBaseV1 {
        match self {
            Self::V1(base) => base,
        }
    }

    /// Returns a mutable reference to the V1 base payload.
    pub const fn as_v1_mut(&mut self) -> &mut OpFlashblockExecutionPayloadBaseV1 {
        match self {
            Self::V1(base) => base,
        }
    }

    /// Consumes self and returns the V1 base payload.
    pub fn into_v1(self) -> OpFlashblockExecutionPayloadBaseV1 {
        match self {
            Self::V1(base) => base,
        }
    }

    /// Returns the parent beacon block root.
    pub const fn parent_beacon_block_root(&self) -> B256 {
        self.as_v1().parent_beacon_block_root
    }

    /// Returns the parent hash.
    pub const fn parent_hash(&self) -> B256 {
        self.as_v1().parent_hash
    }

    /// Returns the fee recipient.
    pub const fn fee_recipient(&self) -> Address {
        self.as_v1().fee_recipient
    }

    /// Returns the prev randao value.
    pub const fn prev_randao(&self) -> B256 {
        self.as_v1().prev_randao
    }

    /// Returns the block number.
    pub const fn block_number(&self) -> u64 {
        self.as_v1().block_number
    }

    /// Returns the gas limit.
    pub const fn gas_limit(&self) -> u64 {
        self.as_v1().gas_limit
    }

    /// Returns the timestamp.
    pub const fn timestamp(&self) -> u64 {
        self.as_v1().timestamp
    }

    /// Returns a reference to the extra data.
    pub const fn extra_data(&self) -> &Bytes {
        &self.as_v1().extra_data
    }

    /// Returns a reference to the base fee per gas.
    pub const fn base_fee_per_gas(&self) -> &U256 {
        &self.as_v1().base_fee_per_gas
    }
}

impl<'a> From<OpFlashblockExecutionPayloadBaseRef<'a>> for OpFlashblockExecutionPayloadBase {
    fn from(base: OpFlashblockExecutionPayloadBaseRef<'a>) -> Self {
        match base {
            OpFlashblockExecutionPayloadBaseRef::V1(v1) => Self::V1(v1.clone()),
        }
    }
}

/// Borrowed reference to execution payload base.
///
/// This enum allows for future versioning of flashblock execution payload base types
/// while providing zero-cost access to the inner fields via [`Deref`](core::ops::Deref).
#[derive(Debug, Clone, Copy)]
pub enum OpFlashblockExecutionPayloadBaseRef<'a> {
    /// Version 1 execution payload base reference.
    V1(&'a OpFlashblockExecutionPayloadBaseV1),
}

impl<'a> core::ops::Deref for OpFlashblockExecutionPayloadBaseRef<'a> {
    type Target = OpFlashblockExecutionPayloadBaseV1;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::V1(inner) => inner,
        }
    }
}

impl From<OpFlashblockExecutionPayloadBaseV1> for OpFlashblockExecutionPayloadBase {
    fn from(base: OpFlashblockExecutionPayloadBaseV1) -> Self {
        Self::V1(base)
    }
}

/// Immutable block properties shared across all flashblocks in a sequence.
///
/// These properties remain constant throughout the block construction process
/// and are set at the beginning of the flashblock sequence.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OpFlashblockExecutionPayloadBaseV1 {
    /// Parent beacon block root.
    pub parent_beacon_block_root: B256,
    /// Hash of the parent block.
    pub parent_hash: B256,
    /// Address that receives fees for this block.
    pub fee_recipient: Address,
    /// The previous randao value.
    pub prev_randao: B256,
    /// Block number.
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity"))]
    pub block_number: u64,
    /// Gas limit for this block.
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity"))]
    pub gas_limit: u64,
    /// Block timestamp.
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity"))]
    pub timestamp: u64,
    /// Extra data for the block.
    pub extra_data: Bytes,
    /// Base fee per gas for this block.
    pub base_fee_per_gas: U256,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_serde_roundtrip() {
        let base = OpFlashblockExecutionPayloadBaseV1 {
            parent_beacon_block_root: B256::random(),
            parent_hash: B256::random(),
            fee_recipient: Address::random(),
            prev_randao: B256::random(),
            block_number: 100,
            gas_limit: 30_000_000,
            timestamp: 1234567890,
            extra_data: Bytes::from(vec![1, 2, 3]),
            base_fee_per_gas: U256::from(1000000000u64),
        };

        let json = serde_json::to_string(&base).unwrap();
        let decoded: OpFlashblockExecutionPayloadBaseV1 = serde_json::from_str(&json).unwrap();
        assert_eq!(base, decoded);
    }

    #[test]
    fn test_base_snake_case_serialization() {
        let base = OpFlashblockExecutionPayloadBaseV1 {
            parent_beacon_block_root: B256::ZERO,
            parent_hash: B256::ZERO,
            fee_recipient: Address::ZERO,
            prev_randao: B256::ZERO,
            block_number: 1,
            gas_limit: 30_000_000,
            timestamp: 1234567890,
            extra_data: Bytes::default(),
            base_fee_per_gas: U256::from(1000000000u64),
        };

        let json = serde_json::to_string(&base).unwrap();
        assert!(json.contains("parent_beacon_block_root"));
        assert!(json.contains("parent_hash"));
        assert!(json.contains("fee_recipient"));
        assert!(json.contains("prev_randao"));
        assert!(json.contains("block_number"));
        assert!(json.contains("gas_limit"));
        assert!(json.contains("base_fee_per_gas"));
    }
}
