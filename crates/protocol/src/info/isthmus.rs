//! Isthmus L1 Block Info transaction types.

use alloc::{format, string::ToString, vec::Vec};
use alloy_primitives::{Address, Bytes, B256, U256};

use crate::DecodeError;

/// Represents the fields within an Isthnus L1 block info transaction.
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
/// | 4       | OperatorFeeScalar        |
/// | 8       | OperatorFeeConstant      |
/// +---------+--------------------------+
#[derive(Debug, Clone, Hash, Eq, PartialEq, Default, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct L1BlockInfoIsthmus {
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
    /// The operator fee scalar
    pub operator_fee_scalar: u32,
    /// The operator fee constant
    pub operator_fee_constant: u64,
}

impl L1BlockInfoIsthmus {
    /// The type byte identifier for the L1 scalar format in Isthmus.
    pub const L1_SCALAR: u8 = 2;

    /// The length of an L1 info transaction in Isthmus.
    pub const L1_INFO_TX_LEN: usize = 4 + 32 * 5 + 4 + 8;

    /// The 4 byte selector of "setL1BlockValuesIsthmus()"
    pub const L1_INFO_TX_SELECTOR: [u8; 4] = [0x09, 0x89, 0x99, 0xbe];

    /// Encodes the [L1BlockInfoIsthmus] object into Ethereum transaction calldata.
    pub fn encode_calldata(&self) -> Bytes {
        let mut buf = Vec::with_capacity(Self::L1_INFO_TX_LEN);
        buf.extend_from_slice(Self::L1_INFO_TX_SELECTOR.as_ref());
        buf.extend_from_slice(self.base_fee_scalar.to_be_bytes().as_ref());
        buf.extend_from_slice(self.blob_base_fee_scalar.to_be_bytes().as_ref());
        buf.extend_from_slice(self.sequence_number.to_be_bytes().as_ref());
        buf.extend_from_slice(self.time.to_be_bytes().as_ref());
        buf.extend_from_slice(self.number.to_be_bytes().as_ref());
        buf.extend_from_slice(U256::from(self.base_fee).to_be_bytes::<32>().as_ref());
        buf.extend_from_slice(U256::from(self.blob_base_fee).to_be_bytes::<32>().as_ref());
        buf.extend_from_slice(self.block_hash.as_ref());
        buf.extend_from_slice(self.batcher_address.into_word().as_ref());
        buf.extend_from_slice(self.operator_fee_scalar.to_be_bytes().as_ref());
        buf.extend_from_slice(self.operator_fee_constant.to_be_bytes().as_ref());
        buf.into()
    }

    /// Decodes the [L1BlockInfoIsthmus] object from ethereum transaction calldata.
    pub fn decode_calldata(r: &[u8]) -> Result<Self, DecodeError> {
        if r.len() != Self::L1_INFO_TX_LEN {
            return Err(DecodeError::InvalidLength(format!(
                "Invalid calldata length for Isthmus L1 info transaction, expected {}, got {}",
                Self::L1_INFO_TX_LEN,
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
        let time =
            u64::from_be_bytes(r[20..28].try_into().map_err(|_| {
                DecodeError::ParseError("Conversion error for timestamp".to_string())
            })?);
        let number = u64::from_be_bytes(r[28..36].try_into().map_err(|_| {
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
        let operator_fee_scalar = u32::from_be_bytes(r[164..168].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for operator fee scalar".to_string())
        })?);
        let operator_fee_constant = u64::from_be_bytes(r[168..176].try_into().map_err(|_| {
            DecodeError::ParseError("Conversion error for operator fee constant".to_string())
        })?);

        Ok(Self {
            number,
            time,
            base_fee,
            block_hash,
            sequence_number,
            batcher_address,
            blob_base_fee,
            blob_base_fee_scalar,
            base_fee_scalar,
            operator_fee_scalar,
            operator_fee_constant,
        })
    }
}