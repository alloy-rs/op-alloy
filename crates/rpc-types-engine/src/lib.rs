#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use alloy_rpc_types_engine::ExecutionPayloadV1;
pub use op_alloy_genesis::{RollupConfig, SystemConfig};
pub use op_alloy_protocol::L2BlockInfo;

use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::B256;
use op_alloy_consensus::{OpTxEnvelope, OpTxType};
use op_alloy_protocol::{BlockInfo, L1BlockInfoBedrock, L1BlockInfoEcotone, L1BlockInfoTx};

mod attributes;
pub use attributes::{OptimismAttributesWithParent, OptimismPayloadAttributes};

mod payload_v3;
pub use payload_v3::OptimismExecutionPayloadEnvelopeV3;

mod payload_v4;
pub use payload_v4::OptimismExecutionPayloadEnvelopeV4;

mod errors;
pub use errors::{ToL2BlockRefError, ToSystemConfigError};

/// Converts a payload into its most inner [ExecutionPayloadV1].
pub trait IntoInnerPayload {
    /// Converts the payload into its most inner [ExecutionPayloadV1].
    fn inner_payload(&self) -> &crate::ExecutionPayloadV1;
}

/// OptimismPayload trait defines conversion methods for Optimism-specific payloads.
pub trait OptimismPayload {
    /// Converts the payload into an [L2BlockInfo].
    fn to_l2_block_ref(
        &self,
        rollup_config: &RollupConfig,
    ) -> Result<L2BlockInfo, ToL2BlockRefError>;

    /// Converts the payload into a [SystemConfig].
    fn to_system_config(
        &self,
        rollup_config: &RollupConfig,
    ) -> Result<SystemConfig, ToSystemConfigError>;
}

impl<T> OptimismPayload for T
where
    T: IntoInnerPayload,
{
    fn to_l2_block_ref(
        &self,
        rollup_config: &RollupConfig,
    ) -> Result<L2BlockInfo, ToL2BlockRefError> {
        let inner_payload = self.inner_payload();

        let (l1_origin, sequence_number) =
            if inner_payload.block_number == rollup_config.genesis.l2.number {
                if inner_payload.block_hash != rollup_config.genesis.l2.hash {
                    return Err(ToL2BlockRefError::InvalidGenesisHash);
                }
                (rollup_config.genesis.l1, 0)
            } else {
                if inner_payload.transactions.is_empty() {
                    return Err(ToL2BlockRefError::MissingL1InfoDeposit(inner_payload.block_hash));
                }

                let ty = inner_payload.transactions[0][0];
                if ty != OpTxType::Deposit as u8 {
                    return Err(ToL2BlockRefError::UnexpectedTxType(ty));
                }
                let tx = OpTxEnvelope::decode_2718(&mut inner_payload.transactions[0].as_ref())
                    .map_err(ToL2BlockRefError::TxEnvelopeDecodeError)?;

                let OpTxEnvelope::Deposit(tx) = tx else {
                    return Err(ToL2BlockRefError::FirstTxNonDeposit(tx.tx_type().into()));
                };

                let l1_info = L1BlockInfoTx::decode_calldata(tx.input.as_ref())
                    .map_err(ToL2BlockRefError::BlockInfoDecodeError)?;
                (l1_info.id(), l1_info.sequence_number())
            };

        Ok(L2BlockInfo {
            block_info: BlockInfo {
                hash: inner_payload.block_hash,
                number: inner_payload.block_number,
                parent_hash: inner_payload.parent_hash,
                timestamp: inner_payload.timestamp,
            },
            l1_origin,
            seq_num: sequence_number,
        })
    }

    fn to_system_config(
        &self,
        rollup_config: &RollupConfig,
    ) -> Result<SystemConfig, ToSystemConfigError> {
        let inner_payload = self.inner_payload();

        if inner_payload.block_number == rollup_config.genesis.l2.number {
            if inner_payload.block_hash != rollup_config.genesis.l2.hash {
                return Err(ToSystemConfigError::InvalidGenesisHash);
            }
            return rollup_config
                .genesis
                .system_config
                .ok_or(ToSystemConfigError::MissingSystemConfig);
        }

        if inner_payload.transactions.is_empty() {
            return Err(ToSystemConfigError::MissingL1InfoDeposit(inner_payload.block_hash));
        }
        let ty = inner_payload.transactions[0][0];
        if ty != OpTxType::Deposit as u8 {
            return Err(ToSystemConfigError::UnexpectedTxType(ty));
        }
        let tx = OpTxEnvelope::decode_2718(&mut inner_payload.transactions[0].as_ref())
            .map_err(ToSystemConfigError::TxEnvelopeDecodeError)?;

        let OpTxEnvelope::Deposit(tx) = tx else {
            return Err(ToSystemConfigError::FirstTxNonDeposit(tx.tx_type().into()));
        };

        let l1_info = L1BlockInfoTx::decode_calldata(tx.input.as_ref())
            .map_err(ToSystemConfigError::BlockInfoDecodeError)?;
        let l1_fee_scalar = match l1_info {
            L1BlockInfoTx::Bedrock(L1BlockInfoBedrock { l1_fee_scalar, .. }) => l1_fee_scalar,
            L1BlockInfoTx::Ecotone(L1BlockInfoEcotone {
                base_fee_scalar,
                blob_base_fee_scalar,
                ..
            }) => {
                // Translate Ecotone values back into encoded scalar if needed.
                // We do not know if it was derived from a v0 or v1 scalar,
                // but v1 is fine, a 0 blob base fee has the same effect.
                let mut buf = B256::ZERO;
                buf[0] = 0x01;
                buf[24..28].copy_from_slice(blob_base_fee_scalar.to_be_bytes().as_ref());
                buf[28..32].copy_from_slice(base_fee_scalar.to_be_bytes().as_ref());
                buf.into()
            }
        };

        Ok(SystemConfig {
            batcher_address: l1_info.batcher_address(),
            overhead: l1_info.l1_fee_overhead(),
            scalar: l1_fee_scalar,
            gas_limit: inner_payload.gas_limit,
            base_fee_scalar: None,
            blob_base_fee_scalar: None,
        })
    }
}
