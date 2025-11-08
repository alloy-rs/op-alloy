//! Optimism Flashblocks types.
//!
//! Flashblocks provide real-time visibility into block construction on Base L2,
//! allowing users to see transaction effects before blocks are finalized.
//!
//! See: [Base Flashblocks Documentation](https://docs.base.org/chain/flashblocks)

mod base;
pub use base::{OpFlashblockExecutionPayloadBase, OpFlashblockExecutionPayloadBaseV1};

mod delta;
pub use delta::{OpFlashblockExecutionPayloadDelta, OpFlashblockExecutionPayloadDeltaV1};

mod metadata;
pub use metadata::{OpFlashblockMetadata, OpFlashblockMetadataV1};

mod payload;
pub use payload::{OpFlashblockPayload, OpFlashblockPayloadV1};

mod error;
pub use error::OpFlashblockError;