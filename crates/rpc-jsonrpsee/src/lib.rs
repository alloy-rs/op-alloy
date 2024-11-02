#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub mod traits;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use op_alloy_rpc_types_engine::{ProtocolVersion, SuperchainSignal};

/// Engine API extension for Optimism superchain signaling
#[cfg_attr(not(feature = "client"), rpc(server, namespace = "engine"))]
#[cfg_attr(feature = "client", rpc(server, client, namespace = "engine"))]
pub trait EngineApiExt {
    /// Signal superchain v1 message
    ///
    /// The execution engine SHOULD warn when the recommended version is newer than the current
    /// version. The execution engine SHOULD take safety precautions if it does not meet
    /// the required version.
    ///
    /// # Returns
    /// The latest supported OP-Stack protocol version of the execution engine.
    ///
    /// See: https://specs.optimism.io/protocol/exec-engine.html#engine_signalsuperchainv1
    #[method(name = "engine_signalSuperchainV1")]
    async fn signal_superchain_v1(&self, signal: SuperchainSignal) -> RpcResult<ProtocolVersion>;
}
