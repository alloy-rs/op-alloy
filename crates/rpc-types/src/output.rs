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
    /// block that has received attestations from two-thirds of Ethereum’s validator set,
    /// unlikely to reorg but still possible
    pub safe_l1: L1BlockRef,
    /// finalized_l1 is a justified block that is 1 epoch behind the most recently justified block,
    /// considered extremely unlikely to reorg
    pub finalized_l1: L1BlockRef,
    /// UnsafeL2 is the absolute tip of the L2 chain, pointing to block data not yet submitted to
    /// L1. The sequencer is building this, and verifiers may also be ahead of the
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn test_helper<'a, T>(json_str: &'a str)
    where
        T: Serialize + Deserialize<'a>,
    {
        let deserialize = serde_json::from_str::<T>(json_str).unwrap();
        assert_eq!(json!(json_str), json!(serde_json::to_string(&deserialize).unwrap()));
    }

    #[test]
    fn test_output_response() {
        let output_response_json = r#"{"version":"0x0000000000000000000000000000000000000000000000000000000000000000","outputRoot":"0xf1119e7d0fef8c54ab799be80fc61f503cea4e5c0aa1cf7ac104ef3a104f3bd1","blockRef":{"hash":"0x6d39c46aabc847f5f2664a22bbc5f65a57286603095a9ebc946d1ed19ef4925c","number":118818299,"parentHash":"0x8a0876a165da864c223d30e444b1c003fb59920c88dfb12157c0f83826e0f8ed","timestamp":1713235375,"l1origin":{"blockHash":"0x807da416f5aaa26fa228e0cf53e76fab783b56d7996c717663335b40e0b28824"},"sequenceNumber":4},"withdrawalStorageRoot":"0x5c9a29a8ad2ecf97fb4bdea74c715fd2c13fa87d4861414478bc4579601c3585","stateRoot":"0x16849c0a93d00bb2d7ceacda11a1478854d2bbb0a377b4d6793b67a3f05eb6fe","syncStatus":{"current_l1":{"hash":"0x2f0f186d0fece338aa563f5dfc49a73cba5607445ff87aca833fd1d6833c5e05","number":19661406,"parentHash":"0x2c7c564d2960c8035fa6962ebf071668fdcdf8ca004bca5adfd04166ce32aacc","timestamp":1713190115},"current_l1_finalized":{"hash":"0xbd916c8552f5dcd68d2cc836a4d173426e85e6625845cfb3fb60610d383670db","number":19665084,"parentHash":"0xe16fade2cddae87d0f9487600481f980619a138de735c97626239edf08c53275","timestamp":1713234647},"head_l1":{"hash":"0xf98493dcc3d82fe9af339c0a81b0f96172a56764f9abcff464c740e0cb3ccee7","number":19665175,"parentHash":"0xfbab86e5b807916c7ddfa395db794cdf4162128b9770eb8eb829679d81d74328","timestamp":1713235763},"safe_l1":{"hash":"0xfb8f07e551eb65c3282aaefe9a4954c15672e0077b2a5a1db18fcd2126cbc922","number":19665115,"parentHash":"0xfc0d62788fb9cda1cacb54a0e53ca398289436a6b68d1ba69db2942500b4ce5f","timestamp":1713235031},"finalized_l1":{"hash":"0xbd916c8552f5dcd68d2cc836a4d173426e85e6625845cfb3fb60610d383670db","number":19665084,"parentHash":"0xe16fade2cddae87d0f9487600481f980619a138de735c97626239edf08c53275","timestamp":1713234647},"unsafe_l2":{"hash":"0x3540517a260316758a4872f7626e8b9e009968b6d8cfa9c11bfd3a03e7656bd5","number":118818499,"parentHash":"0x09f30550e6d6f217691e185bf1a2b4665b83f43fc8dbcc68c0bfd513e6805590","timestamp":1713235775,"l1origin":{"blockHash":"0x036003c1c6561123a2f6573b7a34e9598bd023199e259d91765ee2c8677d9c07"},"sequenceNumber":0},"safe_l2":{"hash":"0x2e8c339104e3ce0a81c636a10ea9181acbfd3c195d43f2f2dacce8f869b1cca8","number":118795493,"parentHash":"0xaac10ffe0a2cbd572a0ee8aa0b09341ad7bbec491f0bf328dd526637617b1b4a","timestamp":1713189763,"l1origin":{"blockHash":"0x55c6ed6a81829e9dffc9c968724af657fcf8e0b497188d05476e94801eb483ce"},"sequenceNumber":1},"finalized_l2":{"hash":"0x2e8c339104e3ce0a81c636a10ea9181acbfd3c195d43f2f2dacce8f869b1cca8","number":118795493,"parentHash":"0xaac10ffe0a2cbd572a0ee8aa0b09341ad7bbec491f0bf328dd526637617b1b4a","timestamp":1713189763,"l1origin":{"blockHash":"0x55c6ed6a81829e9dffc9c968724af657fcf8e0b497188d05476e94801eb483ce"},"sequenceNumber":1},"pending_safe_l2":{"hash":"0x2e8c339104e3ce0a81c636a10ea9181acbfd3c195d43f2f2dacce8f869b1cca8","number":118795493,"parentHash":"0xaac10ffe0a2cbd572a0ee8aa0b09341ad7bbec491f0bf328dd526637617b1b4a","timestamp":1713189763,"l1origin":{"blockHash":"0x55c6ed6a81829e9dffc9c968724af657fcf8e0b497188d05476e94801eb483ce"},"sequenceNumber":1}}}"#;
        test_helper::<OutputResponse>(output_response_json);
    }

    #[test]
    fn serialize_sync_status() {
        let sync_status_json = r#"{"current_l1":{"hash":"0x2f0f186d0fece338aa563f5dfc49a73cba5607445ff87aca833fd1d6833c5e05","number":19661406,"parentHash":"0x2c7c564d2960c8035fa6962ebf071668fdcdf8ca004bca5adfd04166ce32aacc","timestamp":1713190115},"current_l1_finalized":{"hash":"0x4d769506bbfe27051715225af5ec4189f6bbd235b6d32db809dd8f5a03737b03","number":19665052,"parentHash":"0xc6324687f2baf8cc48eebd15df3a461b2b2838b5f5b16615531fc31788edb8c4","timestamp":1713234263},"head_l1":{"hash":"0xfc5ab77c6c08662a3b4d85b8c86010b7aecfc2c0369e4458f80357530db8e919","number":19665141,"parentHash":"0x099792a293002b987f3507524b28614f399b2b5ed607788520963c251844113c","timestamp":1713235355},"safe_l1":{"hash":"0xbd916c8552f5dcd68d2cc836a4d173426e85e6625845cfb3fb60610d383670db","number":19665084,"parentHash":"0xe16fade2cddae87d0f9487600481f980619a138de735c97626239edf08c53275","timestamp":1713234647},"finalized_l1":{"hash":"0x4d769506bbfe27051715225af5ec4189f6bbd235b6d32db809dd8f5a03737b03","number":19665052,"parentHash":"0xc6324687f2baf8cc48eebd15df3a461b2b2838b5f5b16615531fc31788edb8c4","timestamp":1713234263},"unsafe_l2":{"hash":"0x6d39c46aabc847f5f2664a22bbc5f65a57286603095a9ebc946d1ed19ef4925c","number":118818299,"parentHash":"0x8a0876a165da864c223d30e444b1c003fb59920c88dfb12157c0f83826e0f8ed","timestamp":1713235375,"l1origin":{"blockHash":"0x807da416f5aaa26fa228e0cf53e76fab783b56d7996c717663335b40e0b28824"},"sequenceNumber":4},"safe_l2":{"hash":"0x2e8c339104e3ce0a81c636a10ea9181acbfd3c195d43f2f2dacce8f869b1cca8","number":118795493,"parentHash":"0xaac10ffe0a2cbd572a0ee8aa0b09341ad7bbec491f0bf328dd526637617b1b4a","timestamp":1713189763,"l1origin":{"blockHash":"0x55c6ed6a81829e9dffc9c968724af657fcf8e0b497188d05476e94801eb483ce"},"sequenceNumber":1},"finalized_l2":{"hash":"0x2e8c339104e3ce0a81c636a10ea9181acbfd3c195d43f2f2dacce8f869b1cca8","number":118795493,"parentHash":"0xaac10ffe0a2cbd572a0ee8aa0b09341ad7bbec491f0bf328dd526637617b1b4a","timestamp":1713189763,"l1origin":{"blockHash":"0x55c6ed6a81829e9dffc9c968724af657fcf8e0b497188d05476e94801eb483ce"},"sequenceNumber":1},"pending_safe_l2":{"hash":"0x2e8c339104e3ce0a81c636a10ea9181acbfd3c195d43f2f2dacce8f869b1cca8","number":118795493,"parentHash":"0xaac10ffe0a2cbd572a0ee8aa0b09341ad7bbec491f0bf328dd526637617b1b4a","timestamp":1713189763,"l1origin":{"blockHash":"0x55c6ed6a81829e9dffc9c968724af657fcf8e0b497188d05476e94801eb483ce"},"sequenceNumber":1}}"#;
        test_helper::<SyncStatus>(sync_status_json);
    }
}
