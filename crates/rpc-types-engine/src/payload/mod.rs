//! Versioned Optimism execution payloads

pub mod v3;
pub mod v4;

use crate::{OpExecutionPayloadSidecar, OpExecutionPayloadV4};
use alloy_consensus::{Block, EMPTY_ROOT_HASH};
use alloy_eips::Decodable2718;
use alloy_primitives::B256;
use alloy_rpc_types_engine::{ExecutionPayloadV2, ExecutionPayloadV3, PayloadError};

/// An execution payload, which can be either [`ExecutionPayloadV2`], [`ExecutionPayloadV3`], or
/// [`OpExecutionPayloadV4`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum OpExecutionPayload {
    /// V2 payload
    V2(ExecutionPayloadV2),
    /// V3 payload
    V3(ExecutionPayloadV3),
    /// V4 payload
    V4(OpExecutionPayloadV4),
}

impl OpExecutionPayload {
    /// Returns a reference to the V2 payload, if any.
    pub const fn as_v2(&self) -> &ExecutionPayloadV2 {
        match self {
            Self::V2(payload) => payload,
            Self::V3(payload) => &payload.payload_inner,
            Self::V4(payload) => &payload.payload_inner.payload_inner,
        }
    }

    /// Returns a mutable reference to the V2 payload, if any.
    pub fn as_v2_mut(&mut self) -> &ExecutionPayloadV2 {
        match self {
            Self::V2(payload) => payload,
            Self::V3(payload) => &mut payload.payload_inner,
            Self::V4(payload) => &payload.payload_inner.payload_inner,
        }
    }
    /// Returns a reference to the V3 payload, if any.
    pub const fn as_v3(&self) -> Option<&ExecutionPayloadV3> {
        match self {
            Self::V2(payload) => None,
            Self::V3(payload) => Some(&payload),
            Self::V4(payload) => Some(&payload.payload_inner),
        }
    }

    /// Returns a mutable reference to the V3 payload, if any.
    pub fn as_v3_mut(&mut self) -> Option<&ExecutionPayloadV3> {
        match self {
            Self::V2(payload) => None,
            Self::V3(payload) => Some(payload),
            Self::V4(payload) => Some(&payload.payload_inner),
        }
    }
    /// Returns the parent hash for the payload.
    pub const fn parent_hash(&self) -> B256 {
        self.as_v2().payload_inner.parent_hash
    }

    /// Returns the block hash for the payload.
    pub const fn block_hash(&self) -> B256 {
        self.as_v2().payload_inner.block_hash
    }

    /// Returns the block number for this payload.
    pub const fn block_number(&self) -> u64 {
        self.as_v2().payload_inner.block_number
    }

    /// Converts [`OpExecutionPayload`] to [`Block`].
    ///
    /// Caution: This does not set fields that are not part of the payload and only part of the
    /// [`OpExecutionPayloadSidecar`]:
    /// - parent_beacon_block_root
    ///
    /// See also: [`OpExecutionPayload::try_into_block_with_sidecar`]
    pub fn try_into_block<T: Decodable2718>(self) -> Result<Block<T>, PayloadError> {
        match self {
            Self::V2(payload) => payload.try_into_block(),
            Self::V3(payload) => payload.try_into_block(),
            Self::V4(payload) => payload.payload_inner.try_into_block(),
        }
    }

    /// Tries to create a new unsealed block from the given payload and payload sidecar.
    ///
    /// Performs additional validation of `extra_data` and `base_fee_per_gas` fields.
    ///
    /// # Note
    ///
    /// The log bloom is assumed to be validated during serialization.
    ///
    /// See <https://github.com/ethereum/go-ethereum/blob/79a478bb6176425c2400e949890e668a3d9a3d05/core/beacon/types.go#L145>
    pub fn try_into_block_with_sidecar<T: Decodable2718>(
        self,
        sidecar: &OpExecutionPayloadSidecar,
    ) -> Result<Block<T>, PayloadError> {
        let mut base_payload = self.try_into_block()?;
        base_payload.header.parent_beacon_block_root = sidecar.parent_beacon_block_root();
        base_payload.header.requests_hash = Some(EMPTY_ROOT_HASH);
        base_payload.header.withdrawals_root = sidecar.withdrawals_root().copied();

        Ok(base_payload)
    }
}
