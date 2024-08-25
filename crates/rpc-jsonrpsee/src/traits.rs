//! Rollup Node

use jsonrpsee_types::ResponsePayload;
use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use alloy_eips::BlockNumberOrTag;
use op_alloy_rpc_types::sync::{OutputResponse, RollupConfig, SyncStatus};

/// Optimism specified rpc interface.
/// https://docs.optimism.io/builders/node-operators/json-rpc
/// https://github.com/ethereum-optimism/optimism/blob/8dd17a7b114a7c25505cd2e15ce4e3d0f7e3f7c1/op-node/node/api.go#L114
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "optimism"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "optimism"))]
pub trait RollupNode {
    /// Get the output root at a specific block.
    #[method(name = "outputAtBlock")]
    async fn optimism_output_at_block(
        &self,
        block_number: BlockNumberOrTag,
    ) -> RpcResult<OutputResponse>;

    /// Get the synchronization status.
    #[method(name = "syncStatus")]
    async fn optimism_sync_status(&self) -> RpcResult<SyncStatus>;

    /// Get the rollup configuration parameters.
    #[method(name = "rollupConfig")]
    async fn optimism_rollup_config(&self) -> RpcResult<RollupConfig>;

    /// Get the software version.
    #[method(name = "version")]
    async fn optimism_version(&self) -> RpcResult<String>;
}
