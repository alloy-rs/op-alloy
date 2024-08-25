//! Op types related to sync.
use serde::{Deserialize, Serialize};
use alloy_primitives::{BlockNumber, B256};
use alloy_rpc_types_eth::BlockId;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L2BlockRef {
    pub hash: Option<B256>,
    pub number: Option<BlockNumber>,
    pub parent_hash: Option<B256>,
    pub timestamp: Option<u64>,
    pub l1origin: Option<BlockId>,
    pub sequence_number: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L1BlockRef {
    pub hash: Option<B256>,
    pub number: Option<BlockNumber>,
    pub parent_hash: Option<B256>,
    pub timestamp: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncStatus {
    pub current_l1: L1BlockRef,
    pub current_l1_finalized: L1BlockRef,
    pub head_l1: L1BlockRef,
    pub safe_l1: L1BlockRef,
    pub finalized_l1: L1BlockRef,
    pub unsafe_l2: L2BlockRef,
    pub safe_l2: L2BlockRef,
    pub finalized_l2: L2BlockRef,
    pub pending_safe_l2: L2BlockRef,
}
