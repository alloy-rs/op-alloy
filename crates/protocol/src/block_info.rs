//! This module contains the [L1BlockInfoTx] type, and various encoding / decoding methods for it.

use crate::{DepositSourceDomain, L1InfoDepositSource};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use alloy_consensus::Header;
use alloy_eips::BlockNumHash;
use alloy_primitives::{address, Address, Bytes, TxKind, B256, U256, bytes};
use op_alloy_consensus::{OpTxEnvelope, TxDeposit};
use op_alloy_genesis::{RollupConfig, SystemConfig};

use crate::utils::flz_compress_len;

use core::ops::Mul;

/// The system transaction gas limit post-Regolith
const REGOLITH_SYSTEM_TX_GAS: u128 = 1_000_000;
/// The type byte identifier for the L1 scalar format in Ecotone.
const L1_SCALAR_ECOTONE: u8 = 1;
/// The length of an L1 info transaction in Bedrock.
const L1_INFO_TX_LEN_BEDROCK: usize = 4 + 32 * 8;
/// The length of an L1 info transaction in Ecotone.
const L1_INFO_TX_LEN_ECOTONE: usize = 4 + 32 * 5;
/// The length of an L1 info transaction in Holocene.
///
/// The Holocene L1 info transaction size is [L1_INFO_TX_LEN_ECOTONE] + 8 * 2
/// for the EIP-1559 denominator and elasticity parameters.
const L1_INFO_TX_LEN_HOLOCENE: usize = 4 + 32 * 5 + 8 * 2;
/// The 4 byte selector of the
/// "setL1BlockValues(uint64,uint64,uint256,bytes32,uint64,bytes32,uint256,uint256)" function
const L1_INFO_TX_SELECTOR_BEDROCK: [u8; 4] = [0x01, 0x5d, 0x8e, 0xb9];
/// The 4 byte selector of "setL1BlockValuesEcotone()"
const L1_INFO_TX_SELECTOR_ECOTONE: [u8; 4] = [0x44, 0x0a, 0x5e, 0x20];
/// The 4 byte selector of "setL1BlockValuesHolocene()"
const L1_INFO_TX_SELECTOR_HOLOCENE: [u8; 4] = [0xd1, 0xfb, 0xe1, 0x5b];
/// The address of the L1 Block contract
const L1_BLOCK_ADDRESS: Address = address!("4200000000000000000000000000000000000015");
/// The depositor address of the L1 info transaction
const L1_INFO_DEPOSITOR_ADDRESS: Address = address!("deaddeaddeaddeaddeaddeaddeaddeaddead0001");

/// Cost per byte in calldata
const ZERO_BYTE_COST: u64 = 4;
const NON_ZERO_BYTE_COST: u64 = 16;

/// The [L1BlockInfoTx] enum contains variants for the different versions of the L1 block info
/// transaction on OP Stack chains.
///
/// This transaction always sits at the top of the block, and alters the `L1 Block` contract's
/// knowledge of the L1 chain.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum L1BlockInfoTx {
    /// A Bedrock L1 info transaction
    Bedrock(L1BlockInfoBedrock),
    /// An Ecotone L1 info transaction
    Ecotone(L1BlockInfoEcotone),
    /// A Holocene L1 info transaction
    Holocene(L1BlockInfoHolocene),
}

/// Represents the fields within a Bedrock L1 block info transaction.
///
/// Bedrock Binary Format
// +---------+--------------------------+
// | Bytes   | Field                    |
// +---------+--------------------------+
// | 4       | Function signature       |
// | 32      | Number                   |
// | 32      | Time                     |
// | 32      | BaseFee                  |
// | 32      | BlockHash                |
// | 32      | SequenceNumber           |
// | 32      | BatcherHash              |
// | 32      | L1FeeOverhead            |
// | 32      | L1FeeScalar              |
// +---------+--------------------------+
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct L1BlockInfoBedrock {
    /// The current L1 origin block number
    pub number: u64,
    /// The current L1 origin block's timestamp
    pub time: u64,
    /// The current L1 origin block's basefee
    pub base_fee: u64,
    /// The current L1 origin block's hash
    pub block_hash: B256,
    /// The current sequence number
    pub sequence_number: u64,
    /// The address of the batch submitter
    pub batcher_address: Address,
    /// The fee overhead for L1 data
    pub l1_fee_overhead: U256,
    /// The fee scalar for L1 data
    pub l1_fee_scalar: U256,
}

/// Represents the fields within an Ecotone L1 block info transaction.
///
/// Ecotone Binary Format
/// +---------+--------------------------+
/// | Bytes   | Field                    |
/// +---------+--------------------------+
/// | 4       | Function signature       |
/// | 4       | BaseFeeScalar            |
/// | 4       | BlobBaseFeeScalar        |
/// | 8       | SequenceNumber           |
/// | 8       | Timestamp                |
/// | 8       | L1BlockNumber            |
/// | 32      | BaseFee                  |
/// | 32      | BlobBaseFee              |
/// | 32      | BlockHash                |
/// | 32      | BatcherHash              |
/// +---------+--------------------------+
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct L1BlockInfoEcotone {
    /// The current L1 origin block number
    pub number: u64,
    /// The current L1 origin block's timestamp
    pub time: u64,
    /// The current L1 origin block's basefee
    pub base_fee: u64,
    /// The current L1 origin block's hash
    pub block_hash: B256,
    /// The current sequence number
    pub sequence_number: u64,
    /// The address of the batch submitter
    pub batcher_address: Address,
    /// The current blob base fee on L1
    pub blob_base_fee: u128,
    /// The fee scalar for L1 blobspace data
    pub blob_base_fee_scalar: u32,
    /// The fee scalar for L1 data
    pub base_fee_scalar: u32,
}

/// Represents the fields within a Holocene L1 block info transaction.
///
/// Holocene Binary Format
/// +---------+--------------------------+
/// | Bytes   | Field                    |
/// +---------+--------------------------+
/// | 4       | Function signature       |
/// | 4       | BaseFeeScalar            |
/// | 4       | BlobBaseFeeScalar        |
/// | 8       | SequenceNumber           |
/// | 8       | Timestamp                |
/// | 8       | L1BlockNumber            |
/// | 32      | BaseFee                  |
/// | 32      | BlobBaseFee              |
/// | 32      | BlockHash                |
/// | 32      | BatcherHash              |
/// | 8       | Eip1559Denominator       |
/// | 8       | Eip1559Elasticity        |
/// +---------+--------------------------+
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct L1BlockInfoHolocene {
    /// The current L1 origin block number
    pub number: u64,
    /// The current L1 origin block's timestamp
    pub time: u64,
    /// The current L1 origin block's basefee
    pub base_fee: u64,
    /// The current L1 origin block's hash
    pub block_hash: B256,
    /// The current sequence number
    pub sequence_number: u64,
    /// The address of the batch submitter
    pub batcher_address: Address,
    /// The current blob base fee on L1
    pub blob_base_fee: u128,
    /// The fee scalar for L1 blobspace data
    pub blob_base_fee_scalar: u32,
    /// The fee scalar for L1 data
    pub base_fee_scalar: u32,
    /// The EIP-1559 denominator
    pub eip_1559_denominator: u64,
    /// The EIP-1559 elasticity parameter
    pub eip_1559_elasticity: u64,
}

/// An error type for parsing L1 block info transactions.
#[derive(Debug, Copy, Clone)]
pub enum BlockInfoError {
    /// Failed to parse the L1 blob base fee scalar.
    L1BlobBaseFeeScalar,
    /// Failed to parse the base fee scalar.
    BaseFeeScalar,
    /// Failed to parse the EIP-1559 denominator.
    Eip1559Denominator,
    /// Failed to parse the EIP-1559 elasticity parameter.
    Eip1559Elasticity,
}

impl core::fmt::Display for BlockInfoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::L1BlobBaseFeeScalar => {
                write!(f, "Failed to parse the L1 blob base fee scalar")
            }
            Self::BaseFeeScalar => write!(f, "Failed to parse the base fee scalar"),
            Self::Eip1559Denominator => {
                write!(f, "Failed to parse the EIP-1559 denominator")
            }
            Self::Eip1559Elasticity => {
                write!(f, "Failed to parse the EIP-1559 elasticity parameter")
            }
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug)]
pub enum DecodeError {
    InvalidSelector,
    ParseError(String),
    InvalidLength(String),
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSelector => write!(f, "Invalid L1 info transaction selector"),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::InvalidLength(msg) => write!(f, "Invalid data length: {}", msg), /* Handle display for length errors */
        }
    }
}

impl core::error::Error for DecodeError {}

impl L1BlockInfoTx {
    /// Creates a new [L1BlockInfoTx] from the given information.
    pub fn try_new(
        rollup_config: &RollupConfig,
        system_config: &SystemConfig,
        sequence_number: u64,
        l1_header: &Header,
        l2_block_time: u64,
    ) -> Result<Self, BlockInfoError> {
        if rollup_config.is_holocene_active(l2_block_time) {
            let scalar = system_config.scalar.to_be_bytes::<32>();
            let blob_base_fee_scalar = (scalar[0] == L1_SCALAR_ECOTONE)
                .then(|| {
                    Ok::<u32, BlockInfoError>(u32::from_be_bytes(
                        scalar[24..28]
                            .try_into()
                            .map_err(|_| BlockInfoError::L1BlobBaseFeeScalar)?,
                    ))
                })
                .transpose()?
                .unwrap_or_default();
            let base_fee_scalar = u32::from_be_bytes(
                scalar[28..32].try_into().map_err(|_| BlockInfoError::BaseFeeScalar)?,
            );
            let eip_1559_denominator = rollup_config
                .canyon_base_fee_params
                .max_change_denominator
                .try_into()
                .map_err(|_| BlockInfoError::Eip1559Denominator)?;
            let eip_1559_elasticity = rollup_config
                .canyon_base_fee_params
                .elasticity_multiplier
                .try_into()
                .map_err(|_| BlockInfoError::Eip1559Elasticity)?;
            return Ok(Self::Holocene(L1BlockInfoHolocene {
                number: l1_header.number,
                time: l1_header.timestamp,
                base_fee: l1_header.base_fee_per_gas.unwrap_or(0) as u64,
                block_hash: l1_header.hash_slow(),
                sequence_number,
                batcher_address: system_config.batcher_address,
                blob_base_fee: l1_header.blob_fee().unwrap_or(1),
                blob_base_fee_scalar,
                base_fee_scalar,
                eip_1559_denominator,
                eip_1559_elasticity,
            }));
        }
        // In the first block of Ecotone, the L1Block contract has not been upgraded yet due to the
        // upgrade transactions being placed after the L1 info transaction. Because of this,
        // for the first block of Ecotone, we send a Bedrock style L1 block info transaction
        if rollup_config.is_ecotone_active(l2_block_time)
            && rollup_config.ecotone_time.unwrap_or_default() != l2_block_time
        {
            let scalar = system_config.scalar.to_be_bytes::<32>();
            let blob_base_fee_scalar = (scalar[0] == L1_SCALAR_ECOTONE)
                .then(|| {
                    Ok::<u32, BlockInfoError>(u32::from_be_bytes(
                        scalar[24..28]
                            .try_into()
                            .map_err(|_| BlockInfoError::L1BlobBaseFeeScalar)?,
                    ))
                })
                .transpose()?
                .unwrap_or_default();
            let base_fee_scalar = u32::from_be_bytes(
                scalar[28..32].try_into().map_err(|_| BlockInfoError::BaseFeeScalar)?,
            );
            Ok(Self::Ecotone(L1BlockInfoEcotone {
                number: l1_header.number,
                time: l1_header.timestamp,
                base_fee: l1_header.base_fee_per_gas.unwrap_or(0) as u64,
                block_hash: l1_header.hash_slow(),
                sequence_number,
                batcher_address: system_config.batcher_address,
                blob_base_fee: l1_header.blob_fee().unwrap_or(1),
                blob_base_fee_scalar,
                base_fee_scalar,
            }))
        } else {
            Ok(Self::Bedrock(L1BlockInfoBedrock {
                number: l1_header.number,
                time: l1_header.timestamp,
                base_fee: l1_header.base_fee_per_gas.unwrap_or(0) as u64,
                block_hash: l1_header.hash_slow(),
                sequence_number,
                batcher_address: system_config.batcher_address,
                l1_fee_overhead: system_config.overhead,
                l1_fee_scalar: system_config.scalar,
            }))
        }
    }

    /// Creates a new [L1BlockInfoTx] from the given information and returns a typed [TxDeposit] to
    /// include at the top of a block.
    pub fn try_new_with_deposit_tx(
        rollup_config: &RollupConfig,
        system_config: &SystemConfig,
        sequence_number: u64,
        l1_header: &Header,
        l2_block_time: u64,
    ) -> Result<(Self, OpTxEnvelope), BlockInfoError> {
        let l1_info =
            Self::try_new(rollup_config, system_config, sequence_number, l1_header, l2_block_time)?;

        let source = DepositSourceDomain::L1Info(L1InfoDepositSource {
            l1_block_hash: l1_info.block_hash(),
            seq_number: sequence_number,
        });

        let mut deposit_tx = TxDeposit {
            source_hash: source.source_hash(),
            from: L1_INFO_DEPOSITOR_ADDRESS,
            to: TxKind::Call(L1_BLOCK_ADDRESS),
            mint: None,
            value: U256::ZERO,
            gas_limit: 150_000_000,
            is_system_transaction: true,
            input: l1_info.encode_calldata(),
        };

        // With the regolith hardfork, system transactions were deprecated, and we allocate
        // a constant amount of gas for special transactions like L1 block info.
        if rollup_config.is_regolith_active(l2_block_time) {
            deposit_tx.is_system_transaction = false;
            deposit_tx.gas_limit = REGOLITH_SYSTEM_TX_GAS;
        }

        Ok((l1_info, OpTxEnvelope::Deposit(deposit_tx)))
    }

    /// Decodes the [L1BlockInfoEcotone] object from ethereum transaction calldata.
    pub fn decode_calldata(r: &[u8]) -> Result<Self, DecodeError> {
        let selector = r
            .get(0..4)
            .ok_or(DecodeError::ParseError("Slice out of range".to_string()))
            .and_then(|slice| {
                slice.try_into().map_err(|_| {
                    DecodeError::ParseError("Failed to convert 4byte slice to array".to_string())
                })
            })?;
        match selector {
            L1_INFO_TX_SELECTOR_BEDROCK => L1BlockInfoBedrock::decode_calldata(r)
                .map(Self::Bedrock)
                .map_err(|e| DecodeError::ParseError(format!("Bedrock decode error: {}", e))),
            L1_INFO_TX_SELECTOR_ECOTONE => L1BlockInfoEcotone::decode_calldata(r)
                .map(Self::Ecotone)
                .map_err(|e| DecodeError::ParseError(format!("Ecotone decode error: {}", e))),
            L1_INFO_TX_SELECTOR_HOLOCENE => L1BlockInfoHolocene::decode_calldata(r)
                .map(Self::Holocene)
                .map_err(|e| DecodeError::ParseError(format!("Holocene decode error: {}", e))),
            _ => Err(DecodeError::InvalidSelector),
        }
    }

    /// Returns the block hash for the [L1BlockInfoTx].
    pub const fn block_hash(&self) -> B256 {
        match self {
            Self::Bedrock(ref tx) => tx.block_hash,
            Self::Ecotone(ref tx) => tx.block_hash,
            Self::Holocene(ref tx) => tx.block_hash,
        }
    }

    /// Encodes the [L1BlockInfoTx] object into Ethereum transaction calldata.
    pub fn encode_calldata(&self) -> Bytes {
        match self {
            Self::Bedrock(bedrock_tx) => bedrock_tx.encode_calldata(),
            Self::Ecotone(ecotone_tx) => ecotone_tx.encode_calldata(),
            Self::Holocene(holocene_tx) => holocene_tx.encode_calldata(),
        }
    }

    /// Returns the L1 [BlockNumHash] for the info transaction.
    pub const fn id(&self) -> BlockNumHash {
        match self {
            Self::Ecotone(L1BlockInfoEcotone { number, block_hash, .. }) => {
                BlockNumHash { number: *number, hash: *block_hash }
            }
            Self::Bedrock(L1BlockInfoBedrock { number, block_hash, .. }) => {
                BlockNumHash { number: *number, hash: *block_hash }
            }
            Self::Holocene(L1BlockInfoHolocene { number, block_hash, .. }) => {
                BlockNumHash { number: *number, hash: *block_hash }
            }
        }
    }

    /// Returns the L1 fee overhead for the info transaction. After ecotone, this value is ignored.
    pub const fn l1_fee_overhead(&self) -> U256 {
        match self {
            Self::Bedrock(L1BlockInfoBedrock { l1_fee_overhead, .. }) => *l1_fee_overhead,
            Self::Ecotone(_) | Self::Holocene(_) => U256::ZERO,
        }
    }

    /// Returns the batcher address for the info transaction
    pub const fn batcher_address(&self) -> Address {
        match self {
            Self::Bedrock(L1BlockInfoBedrock { batcher_address, .. }) => *batcher_address,
            Self::Ecotone(L1BlockInfoEcotone { batcher_address, .. }) => *batcher_address,
            Self::Holocene(L1BlockInfoHolocene { batcher_address, .. }) => *batcher_address,
        }
    }

    /// Returns the sequence number for the info transaction
    pub const fn sequence_number(&self) -> u64 {
        match self {
            Self::Bedrock(L1BlockInfoBedrock { sequence_number, .. }) => *sequence_number,
            Self::Ecotone(L1BlockInfoEcotone { sequence_number, .. }) => *sequence_number,
            Self::Holocene(L1BlockInfoHolocene { sequence_number, .. }) => *sequence_number,
        }
    }

    /// Returns the L1 block cost
    pub fn calculate_tx_l1_cost(&self, input: &[u8], empty_scalers: bool, ) -> U256 {
        match self {
            Self::Bedrock(bedrock_tx) => bedrock_tx.calculate_tx_l1_cost(input),
            Self::Ecotone(ecotone_tx) => ecotone_tx.calculate_tx_l1_cost(input, empty_scalers),
            Self::Holocene(holocene_tx) => holocene_tx.calculate_tx_l1_cost(input),
        }
    }
}

impl L1BlockInfoBedrock {
    /// Encodes the [L1BlockInfoBedrock] object into Ethereum transaction calldata.
    pub fn encode_calldata(&self) -> Bytes {
        let mut buf = Vec::with_capacity(L1_INFO_TX_LEN_BEDROCK);
        buf.extend_from_slice(L1_INFO_TX_SELECTOR_BEDROCK.as_ref());
        buf.extend_from_slice(U256::from(self.number).to_be_bytes::<32>().as_slice());
        buf.extend_from_slice(U256::from(self.time).to_be_bytes::<32>().as_slice());
        buf.extend_from_slice(U256::from(self.base_fee).to_be_bytes::<32>().as_slice());
        buf.extend_from_slice(self.block_hash.as_slice());
        buf.extend_from_slice(U256::from(self.sequence_number).to_be_bytes::<32>().as_slice());
        buf.extend_from_slice(self.batcher_address.into_word().as_slice());
        buf.extend_from_slice(self.l1_fee_overhead.to_be_bytes::<32>().as_slice());
        buf.extend_from_slice(self.l1_fee_scalar.to_be_bytes::<32>().as_slice());
        buf.into()
    }

    /// Decodes the [L1BlockInfoBedrock] object from ethereum transaction calldata.
    pub fn decode_calldata(r: &[u8]) -> Result<Self, DecodeError> {
        if r.len() != L1_INFO_TX_LEN_BEDROCK {
            return Err(DecodeError::InvalidLength(format!(
                "Invalid calldata length for Bedrock L1 info transaction, expected {}, got {}",
                L1_INFO_TX_LEN_BEDROCK,
                r.len()
            )));
        }

        let number = u64::from_be_bytes(
            r[28..36]
                .try_into()
                .map_err(|_| DecodeError::ParseError("Conversion error for number".to_string()))?,
        );
        let time = u64::from_be_bytes(
            r[60..68]
                .try_into()
                .map_err(|_| DecodeError::ParseError("Conversion error for time".to_string()))?,
        );
        let base_fee =
            u64::from_be_bytes(r[92..100].try_into().map_err(|_| {
                DecodeError::ParseError("Conversion error for base fee".to_string())
            })?);
        let block_hash = B256::from_slice(r[100..132].as_ref());
        let sequence_number = u64::from_be_bytes(r[156..164].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for sequence number".to_string())
        })?);
        let batcher_address = Address::from_slice(r[176..196].as_ref());
        let l1_fee_overhead = U256::from_be_slice(r[196..228].as_ref());
        let l1_fee_scalar = U256::from_be_slice(r[228..260].as_ref());

        Ok(Self {
            number,
            time,
            base_fee,
            block_hash,
            sequence_number,
            batcher_address,
            l1_fee_overhead,
            l1_fee_scalar,
        })
    }

    /// Calculate the data gas for posting the transaction on L1. Calldata costs 16 gas per byte
    /// after compression.
    ///
    /// Prior to fjord, calldata costs 16 gas per non-zero byte and 4 gas per zero byte.
    ///
    /// Prior to regolith, an extra 68 non-zero bytes were included in the rollup data costs to
    /// account for the empty signature.
    pub fn data_gas(&self, input: &[u8]) -> U256 {
        let mut rollup_data_gas_cost = U256::from(input.iter().fold(0, |acc, byte| {
            acc + if *byte == 0x00 {
                ZERO_BYTE_COST
            } else {
                NON_ZERO_BYTE_COST
            }
        }));

        println!("rollup_data_gas_cost: {:?}", rollup_data_gas_cost);

        // Prior to regolith, an extra 68 non zero bytes were included in the rollup data costs.
        rollup_data_gas_cost += U256::from(NON_ZERO_BYTE_COST).mul(U256::from(68));

        rollup_data_gas_cost
    }

    /// Calculate the gas cost of a transaction based on L1 block data posted on L2, pre-Ecotone.
    pub fn calculate_tx_l1_cost(&self, input: &[u8]) -> U256 {
        if input.is_empty() || input.first() == Some(&0x7F) {
            return U256::ZERO;
        }

        let rollup_data_gas_cost = self.data_gas(input);
        println!("rollup_data_gas_cost: {:?}", rollup_data_gas_cost);
        rollup_data_gas_cost
            .saturating_add(self.l1_fee_overhead)
            .saturating_mul(U256::from(self.base_fee))
            .saturating_mul(self.l1_fee_scalar)
            .wrapping_div(U256::from(1_000_000))
    }

}

impl L1BlockInfoEcotone {
    /// Encodes the [L1BlockInfoEcotone] object into Ethereum transaction calldata.
    pub fn encode_calldata(&self) -> Bytes {
        let mut buf = Vec::with_capacity(L1_INFO_TX_LEN_ECOTONE);
        buf.extend_from_slice(L1_INFO_TX_SELECTOR_ECOTONE.as_ref());
        buf.extend_from_slice(self.base_fee_scalar.to_be_bytes().as_ref());
        buf.extend_from_slice(self.blob_base_fee_scalar.to_be_bytes().as_ref());
        buf.extend_from_slice(self.sequence_number.to_be_bytes().as_ref());
        buf.extend_from_slice(self.time.to_be_bytes().as_ref());
        buf.extend_from_slice(self.number.to_be_bytes().as_ref());
        buf.extend_from_slice(U256::from(self.base_fee).to_be_bytes::<32>().as_ref());
        buf.extend_from_slice(U256::from(self.blob_base_fee).to_be_bytes::<32>().as_ref());
        buf.extend_from_slice(self.block_hash.as_ref());
        buf.extend_from_slice(self.batcher_address.into_word().as_ref());
        buf.into()
    }

    /// Decodes the [L1BlockInfoEcotone] object from ethereum transaction calldata.
    pub fn decode_calldata(r: &[u8]) -> Result<Self, DecodeError> {
        if r.len() != L1_INFO_TX_LEN_ECOTONE {
            return Err(DecodeError::InvalidLength(format!(
                "Invalid calldata length for Ecotone L1 info transaction, expected {}, got {}",
                L1_INFO_TX_LEN_ECOTONE,
                r.len()
            )));
        }
        let base_fee_scalar = u32::from_be_bytes(r[4..8].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for base fee scalar".to_string())
        })?);
        let blob_base_fee_scalar = u32::from_be_bytes(r[8..12].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for blob base fee scalar".to_string())
        })?);
        let sequence_number = u64::from_be_bytes(r[12..20].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for sequence number".to_string())
        })?);
        let timestamp =
            u64::from_be_bytes(r[20..28].try_into().map_err(|_| {
                DecodeError::ParseError("Conversion error for timestamp".to_string())
            })?);
        let l1_block_number = u64::from_be_bytes(r[28..36].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for L1 block number".to_string())
        })?);
        let base_fee =
            u64::from_be_bytes(r[60..68].try_into().map_err(|_| {
                DecodeError::ParseError("Conversion error for base fee".to_string())
            })?);
        let blob_base_fee = u128::from_be_bytes(r[84..100].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for blob base fee".to_string())
        })?);
        let block_hash = B256::from_slice(r[100..132].as_ref());
        let batcher_address = Address::from_slice(r[144..164].as_ref());

        Ok(Self {
            number: l1_block_number,
            time: timestamp,
            base_fee,
            block_hash,
            sequence_number,
            batcher_address,
            blob_base_fee,
            blob_base_fee_scalar,
            base_fee_scalar,
        })
    }

    /// Calculate the data gas for posting the transaction on L1. Calldata costs 16 gas per byte
    /// after compression.
    ///
    /// Prior to fjord, calldata costs 16 gas per non-zero byte and 4 gas per zero byte.
    pub fn data_gas(&self, input: &[u8]) -> U256 {
        let mut rollup_data_gas_cost = U256::from(input.iter().fold(0, |acc, byte| {
            acc + if *byte == 0x00 {
                ZERO_BYTE_COST
            } else {
                NON_ZERO_BYTE_COST
            }
        }));

        rollup_data_gas_cost
    }

    /// Calculate the gas cost of a transaction based on L1 block data posted on L2, post-Ecotone.
    ///
    /// [SpecId::ECOTONE] L1 cost function:
    /// `(calldataGas/16)*(l1BaseFee*16*l1BaseFeeScalar + l1BlobBaseFee*l1BlobBaseFeeScalar)/1e6`
    ///
    /// We divide "calldataGas" by 16 to change from units of calldata gas to "estimated # of bytes when compressed".
    /// Known as "compressedTxSize" in the spec.
    ///
    /// Function is actually computed as follows for better precision under integer arithmetic:
    /// `calldataGas*(l1BaseFee*16*l1BaseFeeScalar + l1BlobBaseFee*l1BlobBaseFeeScalar)/16e6`
    pub fn calculate_tx_l1_cost(&self, input: &[u8], empty_scalers: bool) -> U256 {
        if input.is_empty() || input.first() == Some(&0x7F) {
            return U256::ZERO;
        }

        // There is an edgecase where, for the very first Ecotone block (unless it is activated at Genesis), we must
        // use the Bedrock cost function. To determine if this is the case, we can check if the Ecotone parameters are
        // unset.
        if empty_scalers {
            return self.calculate_tx_l1_cost_bedrock(input);
        }

        let rollup_data_gas_cost = self.data_gas(input);
        let l1_fee_scaled = self.calculate_l1_fee_scaled_ecotone();

        l1_fee_scaled
            .saturating_mul(rollup_data_gas_cost)
            .wrapping_div(U256::from(1_000_000 * NON_ZERO_BYTE_COST))
    }

    /// Calculate the gas cost of a transaction based on L1 block data posted on L2, pre-Ecotone.
    pub fn calculate_tx_l1_cost_bedrock(&self, input: &[u8]) -> U256 {
        let rollup_data_gas_cost = self.data_gas(input);
        println!("rollup_data_gas_cost: {:?}", rollup_data_gas_cost);
        rollup_data_gas_cost
            //.saturating_add(self.l1_fee_overhead)
            .saturating_mul(U256::from(self.base_fee))
            .saturating_mul(U256::from(self.base_fee_scalar))
            .wrapping_div(U256::from(1_000_000))
    }

    // l1BaseFee*16*l1BaseFeeScalar + l1BlobBaseFee*l1BlobBaseFeeScalar
    fn calculate_l1_fee_scaled_ecotone(&self) -> U256 {
        let calldata_cost_per_byte: u64 = self
            .base_fee
            .saturating_mul(NON_ZERO_BYTE_COST)
            .saturating_mul(self.base_fee_scalar as u64);
        let blob_cost_per_byte = self
            .blob_base_fee
            .saturating_mul(self.blob_base_fee_scalar as u128);

            U256::from(calldata_cost_per_byte).saturating_add(U256::from(blob_cost_per_byte))
    }
}

impl L1BlockInfoHolocene {
    /// Encodes the [L1BlockInfoHolocene] object into Ethereum transaction calldata.
    pub fn encode_calldata(&self) -> Bytes {
        let mut buf = Vec::with_capacity(L1_INFO_TX_LEN_HOLOCENE);
        buf.extend_from_slice(L1_INFO_TX_SELECTOR_HOLOCENE.as_ref());
        buf.extend_from_slice(self.base_fee_scalar.to_be_bytes().as_ref());
        buf.extend_from_slice(self.blob_base_fee_scalar.to_be_bytes().as_ref());
        buf.extend_from_slice(self.sequence_number.to_be_bytes().as_ref());
        buf.extend_from_slice(self.time.to_be_bytes().as_ref());
        buf.extend_from_slice(self.number.to_be_bytes().as_ref());
        buf.extend_from_slice(U256::from(self.base_fee).to_be_bytes::<32>().as_ref());
        buf.extend_from_slice(U256::from(self.blob_base_fee).to_be_bytes::<32>().as_ref());
        buf.extend_from_slice(self.block_hash.as_ref());
        buf.extend_from_slice(self.batcher_address.into_word().as_ref());
        buf.extend_from_slice(self.eip_1559_denominator.to_be_bytes().as_ref());
        buf.extend_from_slice(self.eip_1559_elasticity.to_be_bytes().as_ref());
        buf.into()
    }

    /// Decodes the [L1BlockInfoHolocene] object from ethereum transaction calldata.
    pub fn decode_calldata(r: &[u8]) -> Result<Self, DecodeError> {
        if r.len() != L1_INFO_TX_LEN_HOLOCENE {
            return Err(DecodeError::InvalidLength(format!(
                "Invalid calldata length for Holocene L1 info transaction, expected {}, got {}",
                L1_INFO_TX_LEN_HOLOCENE,
                r.len()
            )));
        }
        let base_fee_scalar = u32::from_be_bytes(r[4..8].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for base fee scalar".to_string())
        })?);
        let blob_base_fee_scalar = u32::from_be_bytes(r[8..12].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for blob base fee scalar".to_string())
        })?);
        let sequence_number = u64::from_be_bytes(r[12..20].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for sequence number".to_string())
        })?);
        let timestamp =
            u64::from_be_bytes(r[20..28].try_into().map_err(|_| {
                DecodeError::ParseError("Conversion error for timestamp".to_string())
            })?);
        let l1_block_number = u64::from_be_bytes(r[28..36].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for L1 block number".to_string())
        })?);
        let base_fee =
            u64::from_be_bytes(r[60..68].try_into().map_err(|_| {
                DecodeError::ParseError("Conversion error for base fee".to_string())
            })?);
        let blob_base_fee = u128::from_be_bytes(r[84..100].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for blob base fee".to_string())
        })?);
        let block_hash = B256::from_slice(r[100..132].as_ref());
        let batcher_address = Address::from_slice(r[144..164].as_ref());
        let eip_1559_denominator = u64::from_be_bytes(r[164..172].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for EIP-1559 denominator".to_string())
        })?);
        let eip_1559_elasticity = u64::from_be_bytes(r[172..180].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for EIP-1559 elasticity".to_string())
        })?);

        Ok(Self {
            number: l1_block_number,
            time: timestamp,
            base_fee,
            block_hash,
            sequence_number,
            batcher_address,
            blob_base_fee,
            blob_base_fee_scalar,
            base_fee_scalar,
            eip_1559_denominator,
            eip_1559_elasticity,
        })
    }

    /// Calculate the data gas for posting the transaction on L1. Calldata costs 16 gas per byte
    /// after compression.
    pub fn data_gas(&self, input: &[u8]) -> U256 {
        let estimated_size = self.tx_estimated_size_fjord(input);
        return estimated_size
            .saturating_mul(U256::from(NON_ZERO_BYTE_COST))
            .wrapping_div(U256::from(1_000_000));
    }

    // Calculate the estimated compressed transaction size in bytes, scaled by 1e6.
    // This value is computed based on the following formula:
    // max(minTransactionSize, intercept + fastlzCoef*fastlzSize)
    fn tx_estimated_size_fjord(&self, input: &[u8]) -> U256 {
        let fastlz_size = U256::from(flz_compress_len(input));

        fastlz_size
            .saturating_mul(U256::from(836_500))
            .saturating_sub(U256::from(42_585_600))
            .max(U256::from(100_000_000))
    }

    /// Calculate the gas cost of a transaction based on L1 block data posted on L2, post-Fjord.
    ///
    /// [OptimismSpecId::FJORD] L1 cost function:
    /// `estimatedSize*(baseFeeScalar*l1BaseFee*16 + blobFeeScalar*l1BlobBaseFee)/1e12`
    pub fn calculate_tx_l1_cost(&self, input: &[u8]) -> U256 {
        if input.is_empty() || input.first() == Some(&0x7F) {
            return U256::ZERO;
        }

        let l1_fee_scaled = self.calculate_l1_fee_scaled_ecotone();
        let estimated_size = self.tx_estimated_size_fjord(input);

        U256::from(estimated_size)
            .saturating_mul(U256::from(l1_fee_scaled))
            .wrapping_div(U256::from(1_000_000_000_000u64))
    }

    // l1BaseFee*16*l1BaseFeeScalar + l1BlobBaseFee*l1BlobBaseFeeScalar
    fn calculate_l1_fee_scaled_ecotone(&self) -> U256 {
        let calldata_cost_per_byte: u64 = self
            .base_fee
            .saturating_mul(NON_ZERO_BYTE_COST)
            .saturating_mul(self.base_fee_scalar as u64);
        let blob_cost_per_byte = self
            .blob_base_fee
            .saturating_mul(self.blob_base_fee_scalar as u128);

            U256::from(calldata_cost_per_byte).saturating_add(U256::from(blob_cost_per_byte))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::string::ToString;
    use alloy_primitives::{address, b256, hex};

    const RAW_BEDROCK_INFO_TX: [u8; L1_INFO_TX_LEN_BEDROCK] = hex!("015d8eb9000000000000000000000000000000000000000000000000000000000117c4eb0000000000000000000000000000000000000000000000000000000065280377000000000000000000000000000000000000000000000000000000026d05d953392012032675be9f94aae5ab442de73c5f4fb1bf30fa7dd0d2442239899a40fc00000000000000000000000000000000000000000000000000000000000000040000000000000000000000006887246668a3b87f54deb3b94ba47a6f63f3298500000000000000000000000000000000000000000000000000000000000000bc00000000000000000000000000000000000000000000000000000000000a6fe0");
    const RAW_ECOTONE_INFO_TX: [u8; L1_INFO_TX_LEN_ECOTONE] = hex!("440a5e2000000558000c5fc5000000000000000500000000661c277300000000012bec20000000000000000000000000000000000000000000000000000000026e9f109900000000000000000000000000000000000000000000000000000000000000011c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add30000000000000000000000006887246668a3b87f54deb3b94ba47a6f63f32985");
    const RAW_HOLOCENE_INFO_TX: [u8; L1_INFO_TX_LEN_HOLOCENE] = hex!("d1fbe15b00000558000c5fc5000000000000000500000000661c277300000000012bec20000000000000000000000000000000000000000000000000000000026e9f109900000000000000000000000000000000000000000000000000000000000000011c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add30000000000000000000000006887246668a3b87f54deb3b94ba47a6f63f3298500000000000012340000000000005678");

    fn get_default_bedrock_l1_info() -> L1BlockInfoBedrock {
        let rollup_config = RollupConfig::default();
        let system_config = SystemConfig::default();
        let sequence_number = 0;
        let l1_header = Header::default();
        let l2_block_time = 0;

        let l1_info = L1BlockInfoTx::try_new(
            &rollup_config,
            &system_config,
            sequence_number,
            &l1_header,
            l2_block_time,
        )
        .unwrap();

        let L1BlockInfoTx::Bedrock(l1_info) = l1_info else {
            panic!("Wrong fork");
        };

        assert_eq!(l1_info.number, l1_header.number);
        assert_eq!(l1_info.time, l1_header.timestamp);
        assert_eq!(l1_info.base_fee, l1_header.base_fee_per_gas.unwrap_or(0) as u64);
        assert_eq!(l1_info.block_hash, l1_header.hash_slow());
        assert_eq!(l1_info.sequence_number, sequence_number);
        assert_eq!(l1_info.batcher_address, system_config.batcher_address);
        assert_eq!(l1_info.l1_fee_overhead, system_config.overhead);
        assert_eq!(l1_info.l1_fee_scalar, system_config.scalar);

        return l1_info;
    }

    fn get_default_ecotone_l1_info() -> L1BlockInfoEcotone {
        let rollup_config = RollupConfig { ecotone_time: Some(1), ..Default::default() };
        let system_config = SystemConfig::default();
        let sequence_number = 0;
        let l1_header = Header::default();
        let l2_block_time = 0xFF;

        let l1_info = L1BlockInfoTx::try_new(
            &rollup_config,
            &system_config,
            sequence_number,
            &l1_header,
            l2_block_time,
        )
        .unwrap();

        let L1BlockInfoTx::Ecotone(l1_info) = l1_info else {
            panic!("Wrong fork");
        };

        assert_eq!(l1_info.number, l1_header.number);
        assert_eq!(l1_info.time, l1_header.timestamp);
        assert_eq!(l1_info.base_fee, l1_header.base_fee_per_gas.unwrap_or(0) as u64);
        assert_eq!(l1_info.block_hash, l1_header.hash_slow());
        assert_eq!(l1_info.sequence_number, sequence_number);
        assert_eq!(l1_info.batcher_address, system_config.batcher_address);
        assert_eq!(l1_info.blob_base_fee, l1_header.blob_fee().unwrap_or(1));

        return l1_info;
    }

    fn get_default_holocene_l1_info() -> L1BlockInfoHolocene {
        let rollup_config = RollupConfig { holocene_time: Some(1), ..Default::default() };
        let system_config = SystemConfig::default();
        let sequence_number = 0;
        let l1_header = Header::default();
        let l2_block_time = 0xFF;

        let l1_info = L1BlockInfoTx::try_new(
            &rollup_config,
            &system_config,
            sequence_number,
            &l1_header,
            l2_block_time,
        )
        .unwrap();

        let L1BlockInfoTx::Holocene(l1_info) = l1_info else {
            panic!("Wrong fork");
        };

        return l1_info;
    }


    #[test]
    fn bedrock_l1_block_info_invalid_len() {
        let err = L1BlockInfoBedrock::decode_calldata(&[0xde, 0xad]);
        assert!(err.is_err());
        assert_eq!(
            err.err().unwrap().to_string(),
            "Invalid data length: Invalid calldata length for Bedrock L1 info transaction, expected 260, got 2"
        );
    }

    #[test]
    fn ecotone_l1_block_info_invalid_len() {
        let err = L1BlockInfoEcotone::decode_calldata(&[0xde, 0xad]);
        assert!(err.is_err());
        assert_eq!(
            err.err().unwrap().to_string(),
            "Invalid data length: Invalid calldata length for Ecotone L1 info transaction, expected 164, got 2"
        );
    }

    #[test]
    fn test_l1_block_info_tx_block_hash_bedrock() {
        let bedrock = L1BlockInfoTx::Bedrock(L1BlockInfoBedrock {
            block_hash: b256!("392012032675be9f94aae5ab442de73c5f4fb1bf30fa7dd0d2442239899a40fc"),
            ..Default::default()
        });
        assert_eq!(
            bedrock.block_hash(),
            b256!("392012032675be9f94aae5ab442de73c5f4fb1bf30fa7dd0d2442239899a40fc")
        );
    }

    #[test]
    fn test_l1_block_info_tx_block_hash_ecotone() {
        let ecotone = L1BlockInfoTx::Ecotone(L1BlockInfoEcotone {
            block_hash: b256!("1c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add3"),
            ..Default::default()
        });
        assert_eq!(
            ecotone.block_hash(),
            b256!("1c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add3")
        );
    }

    #[test]
    fn bedrock_l1_block_info_tx_roundtrip() {
        let expected = L1BlockInfoBedrock {
            number: 18334955,
            time: 1697121143,
            base_fee: 10419034451,
            block_hash: b256!("392012032675be9f94aae5ab442de73c5f4fb1bf30fa7dd0d2442239899a40fc"),
            sequence_number: 4,
            batcher_address: address!("6887246668a3b87f54deb3b94ba47a6f63f32985"),
            l1_fee_overhead: U256::from(0xbc),
            l1_fee_scalar: U256::from(0xa6fe0),
        };

        let L1BlockInfoTx::Bedrock(decoded) =
            L1BlockInfoTx::decode_calldata(RAW_BEDROCK_INFO_TX.as_ref()).unwrap()
        else {
            panic!("Wrong fork");
        };
        assert_eq!(expected, decoded);
        assert_eq!(RAW_BEDROCK_INFO_TX, decoded.encode_calldata().as_ref());
    }

    #[test]
    fn ecotone_l1_block_info_tx_roundtrip() {
        let expected = L1BlockInfoEcotone {
            number: 19655712,
            time: 1713121139,
            base_fee: 10445852825,
            block_hash: b256!("1c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add3"),
            sequence_number: 5,
            batcher_address: address!("6887246668a3b87f54deb3b94ba47a6f63f32985"),
            blob_base_fee: 1,
            blob_base_fee_scalar: 810949,
            base_fee_scalar: 1368,
        };

        let L1BlockInfoTx::Ecotone(decoded) =
            L1BlockInfoTx::decode_calldata(RAW_ECOTONE_INFO_TX.as_ref()).unwrap()
        else {
            panic!("Wrong fork");
        };
        assert_eq!(expected, decoded);
        assert_eq!(decoded.encode_calldata().as_ref(), RAW_ECOTONE_INFO_TX);
    }

    #[test]
    fn holocene_l1_block_info_tx_roundtrip() {
        let expected = L1BlockInfoHolocene {
            number: 19655712,
            time: 1713121139,
            base_fee: 10445852825,
            block_hash: b256!("1c4c84c50740386c7dc081efddd644405f04cde73e30a2e381737acce9f5add3"),
            sequence_number: 5,
            batcher_address: address!("6887246668a3b87f54deb3b94ba47a6f63f32985"),
            blob_base_fee: 1,
            blob_base_fee_scalar: 810949,
            base_fee_scalar: 1368,
            eip_1559_denominator: 0x1234,
            eip_1559_elasticity: 0x5678,
        };

        let L1BlockInfoTx::Holocene(decoded) =
            L1BlockInfoTx::decode_calldata(RAW_HOLOCENE_INFO_TX.as_ref()).unwrap()
        else {
            panic!("Wrong fork");
        };
        assert_eq!(expected, decoded);
        assert_eq!(decoded.encode_calldata().as_ref(), RAW_HOLOCENE_INFO_TX);
    }

    #[test]
    fn try_new_with_deposit_tx_ecotone() {
        let l1_info = get_default_ecotone_l1_info();

        let scalar = SystemConfig::default().scalar.to_be_bytes::<32>();
        let blob_base_fee_scalar = (scalar[0] == L1_SCALAR_ECOTONE)
            .then(|| {
                u32::from_be_bytes(
                    scalar[24..28].try_into().expect("Failed to parse L1 blob base fee scalar"),
                )
            })
            .unwrap_or_default();
        let base_fee_scalar =
            u32::from_be_bytes(scalar[28..32].try_into().expect("Failed to parse base fee scalar"));
        assert_eq!(l1_info.blob_base_fee_scalar, blob_base_fee_scalar);
        assert_eq!(l1_info.base_fee_scalar, base_fee_scalar);
    }

    #[test]
    fn test_data_gas_bedrock() {
        let l1_info = get_default_bedrock_l1_info();

        // 0xFACADE = 6 nibbles = 3 bytes
        // 0xFACADE = 1111 1010 . 1100 1010 . 1101 1110
        let input_1 = bytes!("FACADE");

        // 0xFA00CA00DE = 10 nibbles = 5 bytes
        // 0xFA00CA00DE = 1111 1010 . 0000 0000 . 1100 1010 . 0000 0000 . 1101 1110
        let input_2 = bytes!("FA00CA00DE");

        // Pre-regolith (ie bedrock) has an extra 68 non-zero bytes
        // gas cost = 3 non-zero bytes * NON_ZERO_BYTE_COST + NON_ZERO_BYTE_COST * 68
        // gas cost = 3 * 16 + 68 * 16 = 1136
        let bedrock_data_gas = l1_info.data_gas(&input_1);
        assert_eq!(bedrock_data_gas, U256::from(1136));

        // Pre-regolith (ie bedrock) has an extra 68 non-zero bytes
        // gas cost = 3 non-zero * NON_ZERO_BYTE_COST + 2 * ZERO_BYTE_COST + NON_ZERO_BYTE_COST * 68
        // gas cost = 3 * 16 + 2 * 4 + 68 * 16 = 1144
        let bedrock_data_gas = l1_info.data_gas(&input_2);
        assert_eq!(bedrock_data_gas, U256::from(1144));
    }

    #[test]
    fn test_data_gas_ecotone() {
        let l1_info = get_default_ecotone_l1_info();

        // 0xFACADE = 6 nibbles = 3 bytes
        // 0xFACADE = 1111 1010 . 1100 1010 . 1101 1110
        let input_1 = bytes!("FACADE");

        // 0xFA00CA00DE = 10 nibbles = 5 bytes
        // 0xFA00CA00DE = 1111 1010 . 0000 0000 . 1100 1010 . 0000 0000 . 1101 1110
        let input_2 = bytes!("FA00CA00DE");
        
        // Regolith has no added 68 non zero bytes
        // gas cost = 3 * 16 = 48
        let regolith_data_gas = l1_info.data_gas(&input_1);
        assert_eq!(regolith_data_gas, U256::from(48));
        // Regolith has no added 68 non zero bytes
        // gas cost = 3 * 16 + 2 * 4 = 56
        let regolith_data_gas = l1_info.data_gas(&input_2);
        assert_eq!(regolith_data_gas, U256::from(56));
    }

    #[test]
    fn test_data_gas_holocene() {
        let l1_info = get_default_holocene_l1_info();

        // 0xFACADE = 6 nibbles = 3 bytes
        // 0xFACADE = 1111 1010 . 1100 1010 . 1101 1110
        let input_1 = bytes!("FACADE");

        // 0xFA00CA00DE = 10 nibbles = 5 bytes
        // 0xFA00CA00DE = 1111 1010 . 0000 0000 . 1100 1010 . 0000 0000 . 1101 1110
        let input_2 = bytes!("FA00CA00DE");

        // Fjord has a minimum compressed size of 100 bytes
        // gas cost = 100 * 16 = 1600
        let fjord_data_gas = l1_info.data_gas(&input_1);
        assert_eq!(fjord_data_gas, U256::from(1600));

        // Fjord has a minimum compressed size of 100 bytes
        // gas cost = 100 * 16 = 1600
        let fjord_data_gas = l1_info.data_gas(&input_2);
        assert_eq!(fjord_data_gas, U256::from(1600));
    }

    #[test]
    fn test_calculate_tx_l1_cost_bedrock() {
        let mut l1_block_bedrock = get_default_bedrock_l1_info();
        l1_block_bedrock.base_fee = 1_000;
        l1_block_bedrock.l1_fee_overhead = U256::from(1_000);
        l1_block_bedrock.l1_fee_scalar = U256::from(1_000);

        let input = bytes!("FACADE");
        let gas_cost = l1_block_bedrock.calculate_tx_l1_cost(&input);
        assert_eq!(gas_cost, U256::from(2136));

        // Zero rollup data gas cost should result in zero
        let input = bytes!("");
        let gas_cost = l1_block_bedrock.calculate_tx_l1_cost(&input);
        assert_eq!(gas_cost, U256::ZERO);

        // Deposit transactions with the EIP-2718 type of 0x7F should result in zero
        let input = bytes!("7FFACADE");
        let gas_cost = l1_block_bedrock.calculate_tx_l1_cost(&input);
        assert_eq!(gas_cost, U256::ZERO);
    }

    #[test]
    fn test_calculate_tx_l1_cost_ecotone() {
        let mut l1_block_ecotone = get_default_ecotone_l1_info();
        l1_block_ecotone.base_fee = 1_000;
        l1_block_ecotone.blob_base_fee = 1_000;
        l1_block_ecotone.blob_base_fee_scalar = 1_000;
        l1_block_ecotone.base_fee_scalar = 1_000;

        // calldataGas * (l1BaseFee * 16 * l1BaseFeeScalar + l1BlobBaseFee * l1BlobBaseFeeScalar) / (16 * 1e6)
        // = (16 * 3) * (1000 * 16 * 1000 + 1000 * 1000) / (16 * 1e6)
        // = 51
        let input = bytes!("FACADE");
        let gas_cost = l1_block_ecotone.calculate_tx_l1_cost(&input, false);
        assert_eq!(gas_cost, U256::from(51));

        // Zero rollup data gas cost should result in zero
        let input = bytes!("");
        let gas_cost = l1_block_ecotone.calculate_tx_l1_cost(&input, false);
        assert_eq!(gas_cost, U256::ZERO);

        // Deposit transactions with the EIP-2718 type of 0x7F should result in zero
        let input = bytes!("7FFACADE");
        let gas_cost = l1_block_ecotone.calculate_tx_l1_cost(&input, false);
        assert_eq!(gas_cost, U256::ZERO);

        // If the scalars are empty, the bedrock cost function should be used.
        let input = bytes!("FACADE");
        let gas_cost = l1_block_ecotone.calculate_tx_l1_cost(&input, true);
        assert_eq!(gas_cost, U256::from(1048));
    }

    #[test]
    fn test_calculate_tx_l1_cost_holocene() {
        let mut l1_block_holocene = get_default_holocene_l1_info();

        // l1FeeScaled = baseFeeScalar*l1BaseFee*16 + blobFeeScalar*l1BlobBaseFee
        //             = 1000 * 1000 * 16 + 1000 * 1000
        //             = 17e6
        l1_block_holocene.base_fee = 1_000;
        l1_block_holocene.blob_base_fee = 1_000;
        l1_block_holocene.blob_base_fee_scalar = 1_000;
        l1_block_holocene.base_fee_scalar = 1_000;

        // fastLzSize = 4
        // estimatedSize = max(minTransactionSize, intercept + fastlzCoef*fastlzSize)
        //               = max(100e6, 836500*4 - 42585600)
        //               = 100e6
        let input = bytes!("FACADE");
        // l1Cost = estimatedSize * l1FeeScaled / 1e12
        //        = 100e6 * 17 / 1e6
        //        = 1700
        let gas_cost = l1_block_holocene.calculate_tx_l1_cost(&input);
        assert_eq!(gas_cost, U256::from(1700));

        // fastLzSize = 202
        // estimatedSize = max(minTransactionSize, intercept + fastlzCoef*fastlzSize)
        //               = max(100e6, 836500*202 - 42585600)
        //               = 126387400
        let input = bytes!("02f901550a758302df1483be21b88304743f94f80e51afb613d764fa61751affd3313c190a86bb870151bd62fd12adb8e41ef24f3f000000000000000000000000000000000000000000000000000000000000006e000000000000000000000000af88d065e77c8cc2239327c5edb3a432268e5831000000000000000000000000000000000000000000000000000000000003c1e5000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000148c89ed219d02f1a5be012c689b4f5b731827bebe000000000000000000000000c001a033fd89cb37c31b2cba46b6466e040c61fc9b2a3675a7f5f493ebd5ad77c497f8a07cdf65680e238392693019b4092f610222e71b7cec06449cb922b93b6a12744e");
        // l1Cost = estimatedSize * l1FeeScaled / 1e12
        //        = 126387400 * 17 / 1e6
        //        = 2148
        let gas_cost = l1_block_holocene.calculate_tx_l1_cost(&input);
        assert_eq!(gas_cost, U256::from(2148));

        // Zero rollup data gas cost should result in zero
        let input = bytes!("");
        let gas_cost = l1_block_holocene.calculate_tx_l1_cost(&input);
        assert_eq!(gas_cost, U256::ZERO);

        // Deposit transactions with the EIP-2718 type of 0x7F should result in zero
        let input = bytes!("7FFACADE");
        let gas_cost = l1_block_holocene.calculate_tx_l1_cost(&input);
        assert_eq!(gas_cost, U256::ZERO);
    }

    #[test]
    fn calculate_tx_l1_cost_fjord() {
        let mut l1_block_holocene = get_default_holocene_l1_info();

        // L1 block info for OP mainnet fjord block 124665056
        // <https://optimistic.etherscan.io/block/124665056>
        l1_block_holocene.base_fee = 1055991687;
        l1_block_holocene.blob_base_fee = 1;
        l1_block_holocene.blob_base_fee_scalar = 1014213;
        l1_block_holocene.base_fee_scalar = 5227;

        // second tx in OP mainnet Fjord block 124665056
        // <https://optimistic.etherscan.io/tx/0x1059e8004daff32caa1f1b1ef97fe3a07a8cf40508f5b835b66d9420d87c4a4a>
        const TX: &[u8] = &hex!("02f904940a8303fba78401d6d2798401db2b6d830493e0943e6f4f7866654c18f536170780344aa8772950b680b904246a761202000000000000000000000000087000a300de7200382b55d40045000000e5d60e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003a0000000000000000000000000000000000000000000000000000000000000022482ad56cb0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000120000000000000000000000000dc6ff44d5d932cbd77b52e5612ba0529dc6226f1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000044095ea7b300000000000000000000000021c4928109acb0659a88ae5329b5374a3024694c0000000000000000000000000000000000000000000000049b9ca9a6943400000000000000000000000000000000000000000000000000000000000000000000000000000000000021c4928109acb0659a88ae5329b5374a3024694c000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000024b6b55f250000000000000000000000000000000000000000000000049b9ca9a694340000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000415ec214a3950bea839a7e6fbb0ba1540ac2076acd50820e2d5ef83d0902cdffb24a47aff7de5190290769c4f0a9c6fabf63012986a0d590b1b571547a8c7050ea1b00000000000000000000000000000000000000000000000000000000000000c080a06db770e6e25a617fe9652f0958bd9bd6e49281a53036906386ed39ec48eadf63a07f47cf51a4a40b4494cf26efc686709a9b03939e20ee27e59682f5faa536667e");

        // l1 gas used for tx and l1 fee for tx, from OP mainnet block scanner
        // https://optimistic.etherscan.io/tx/0x1059e8004daff32caa1f1b1ef97fe3a07a8cf40508f5b835b66d9420d87c4a4a
        let expected_data_gas = U256::from(4471);
        let expected_l1_fee = U256::from_be_bytes(hex!(
            "00000000000000000000000000000000000000000000000000000005bf1ab43d"
        ));

        // test
        let data_gas = l1_block_holocene.data_gas(TX);
        assert_eq!(data_gas, expected_data_gas);
        let l1_fee = l1_block_holocene.calculate_tx_l1_cost(TX);
        assert_eq!(l1_fee, expected_l1_fee)
    }
}
