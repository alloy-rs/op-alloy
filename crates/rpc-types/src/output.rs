//! OP types for Output Responses

use alloy_primitives::{BlockNumber, B256};
use alloy_rpc_types_eth::BlockId;
use serde::{Deserialize, Serialize};

// https://github.com/ethereum-optimism/optimism/blob/develop/op-service/eth/id.go#L52
/// Representation of an L1 block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L1BlockRef {
    /// hash of the L1 block
    pub hash: B256,
    /// L1 block number
    pub number: BlockNumber,
    /// block hash of the L1 parent
    pub parent_hash: B256,
    /// timestamp of the L1 block
    pub timestamp: u64,
}

// https://github.com/ethereum-optimism/optimism/blob/develop/op-service/eth/id.go#L33
/// Representation of an L2 block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L2BlockRef {
    /// hash of the reference bedrock block
    pub hash: B256,
    /// bedrock block number
    pub number: BlockNumber,
    /// hash of the parent block
    pub parent_hash: B256,
    /// timestamp associated with the reference block
    pub timestamp: u64,
    ///
    pub l1origin: BlockId,
    /// distance to first block of epoch
    pub sequence_number: u64,
}

// https://github.com/ethereum-optimism/optimism/blob/develop/op-service/eth/sync_status.go#L5
/// SyncStatus is a snapshot of the driver.
/// Values may be zeroed if not yet initialized.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncStatus {
    /// CurrentL1 is the L1 block that the derivation process is last idled at.
    /// This may not be fully derived into L2 data yet.
    /// The safe L2 blocks were produced/included fully from the L1 chain up to and including
    /// this L1 Block.
    /// If the node is synced, this matches the HeadL1, minus the verifier confirmation distance.
    pub current_l1: L1BlockRef,
    /// CurrentL1Finalized is a legacy sync-status attribute. This is deprecated.
    /// A previous version of the L1 finalization-signal was updated only after the block was
    /// retrieved by number.
    /// This attribute just matches FinalizedL1 now.
    pub current_l1_finalized: L1BlockRef,
    /// HeadL1 is the perceived head of the L1 chain, no confirmation distance.
    /// The head is not guaranteed to build on the other L1 sync status fields,
    /// as the node may be in progress of resetting to adapt to a L1 reorg.
    pub head_l1: L1BlockRef,
    ///
    pub safe_l1: L1BlockRef,
    ///
    pub finalized_l1: L1BlockRef,
    /// UnsafeL2 is the absolute tip of the L2 chain, pointing to block data not yet submitted to L1.
    /// The sequencer is building this, and verifiers may also be ahead of the
    /// SafeL2 block if they sync blocks via p2p or other offchain sources.
    pub unsafe_l2: L2BlockRef,
    /// SafeL2 points to the L2 block that was derived from the L1 chain.
    /// At this point may still reorg if the L1 chain reorgs.
    pub safe_l2: L2BlockRef,
    /// FinalizedL2 points to the L2 block that it was derived fully from finalized L1 information,
    /// thus irreversible.
    pub finalized_l2: L2BlockRef,
    /// PendingSafeL2 points to the L2 block processed from the batch, but not consolidated to the
    /// safe block yet.
    pub pending_safe_l2: L2BlockRef,
}

/// Output response for optimism_outputAtBlock
/// the output root at a specific block
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputResponse {
    /// the output root version number, beginning at 0
    pub version: B256,
    /// the output root
    pub output_root: B256,
    /// instance of L2BlockRef
    pub block_ref: L2BlockRef,
    /// storage root of the L2toL1MessageParser contract
    pub withdrawal_storage_root: B256,
    /// the state root
    pub state_root: B256,
    /// snapshot of the driver
    pub sync_status: SyncStatus,
}
