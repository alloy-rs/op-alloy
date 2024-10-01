//! Utility methods used by protocol types.

use crate::{
    block_info::DecodeError, L1BlockInfoBedrock, L1BlockInfoEcotone, L1BlockInfoHolocene,
    L1BlockInfoTx,
};
use alloy_primitives::B256;
use op_alloy_consensus::{OpBlock, OpTxEnvelope};
use op_alloy_genesis::{RollupConfig, SystemConfig};

/// Returns if the given `value` is a deposit transaction.
pub fn starts_with_2718_deposit<B>(value: &B) -> bool
where
    B: AsRef<[u8]>,
{
    value.as_ref().first() == Some(&0x7E)
}

/// An error encountered during [OpBlock] conversion.
#[derive(Debug, derive_more::Display)]
pub enum OpBlockConversionError {
    /// Invalid genesis hash.
    #[display("Invalid genesis hash. Expected {_0}, got {_1}")]
    InvalidGenesisHash(B256, B256),
    /// Invalid transaction type.
    #[display("First payload transaction has unexpected type: {_0}")]
    InvalidTxType(u8),
    /// L1 Info error
    #[display("Failed to decode L1 info: {_0}")]
    L1InfoError(DecodeError),
    /// Missing system config in genesis block.
    #[display("Missing system config in genesis block")]
    MissingSystemConfigGenesis,
    /// Empty transactions.
    #[display("Empty transactions in payload. Block hash: {_0}")]
    EmptyTransactions(B256),
}

impl core::error::Error for OpBlockConversionError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::L1InfoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<DecodeError> for OpBlockConversionError {
    fn from(e: DecodeError) -> Self {
        Self::L1InfoError(e)
    }
}

/// Converts the [OpBlock] to a partial [SystemConfig].
pub fn to_system_config(
    block: &OpBlock,
    rollup_config: &RollupConfig,
) -> Result<SystemConfig, OpBlockConversionError> {
    if block.header.number == rollup_config.genesis.l2.number {
        if block.header.hash_slow() != rollup_config.genesis.l2.hash {
            return Err(OpBlockConversionError::InvalidGenesisHash(
                rollup_config.genesis.l2.hash,
                block.header.hash_slow(),
            ));
        }
        return rollup_config
            .genesis
            .system_config
            .ok_or(OpBlockConversionError::MissingSystemConfigGenesis);
    }

    if block.body.transactions.is_empty() {
        return Err(OpBlockConversionError::EmptyTransactions(block.header.hash_slow()));
    }
    let OpTxEnvelope::Deposit(ref tx) = block.body.transactions[0] else {
        return Err(OpBlockConversionError::InvalidTxType(
            block.body.transactions[0].tx_type() as u8
        ));
    };

    let l1_info = L1BlockInfoTx::decode_calldata(tx.input.as_ref())?;
    let l1_fee_scalar = match l1_info {
        L1BlockInfoTx::Bedrock(L1BlockInfoBedrock { l1_fee_scalar, .. }) => l1_fee_scalar,
        L1BlockInfoTx::Ecotone(L1BlockInfoEcotone {
            base_fee_scalar,
            blob_base_fee_scalar,
            ..
        })
        | L1BlockInfoTx::Holocene(L1BlockInfoHolocene {
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
        gas_limit: block.header.gas_limit,
        base_fee_scalar: None,
        blob_base_fee_scalar: None,
    })
}

/// Returns the length of the data after compression through FastLZ, based on
// https://github.com/Vectorized/solady/blob/5315d937d79b335c668896d7533ac603adac5315/js/solady.js
pub(crate) fn flz_compress_len(input: &[u8]) -> u32 {
    let mut idx: u32 = 2;

    let idx_limit: u32 = if input.len() < 13 { 0 } else { input.len() as u32 - 13 };

    let mut anchor = 0;

    let mut size = 0;

    let mut htab = [0; 8192];

    while idx < idx_limit {
        let mut r: u32;
        let mut distance: u32;

        loop {
            let seq = u24(input, idx);
            let hash = hash(seq);
            r = htab[hash as usize];
            htab[hash as usize] = idx;
            distance = idx - r;
            if idx >= idx_limit {
                break;
            }
            idx += 1;
            if distance < 8192 && seq == u24(input, r) {
                break;
            }
        }

        if idx >= idx_limit {
            break;
        }

        idx -= 1;

        if idx > anchor {
            size = literals(idx - anchor, size);
        }

        let len = cmp(input, r + 3, idx + 3, idx_limit + 9);
        size = flz_match(len, size);

        idx = set_next_hash(&mut htab, input, idx + len);
        idx = set_next_hash(&mut htab, input, idx);
        anchor = idx;
    }

    literals(input.len() as u32 - anchor, size)
}

fn literals(r: u32, size: u32) -> u32 {
    let size = size + 0x21 * (r / 0x20);
    let r = r % 0x20;
    if r != 0 {
        size + r + 1
    } else {
        size
    }
}

fn cmp(input: &[u8], p: u32, q: u32, r: u32) -> u32 {
    let mut l = 0;
    let mut r = r - q;
    while l < r {
        if input[(p + l) as usize] != input[(q + l) as usize] {
            r = 0;
        }
        l += 1;
    }
    l
}

fn flz_match(l: u32, size: u32) -> u32 {
    let l = l - 1;
    let size = size + (3 * (l / 262));
    if l % 262 >= 6 {
        size + 3
    } else {
        size + 2
    }
}

fn set_next_hash(htab: &mut [u32; 8192], input: &[u8], idx: u32) -> u32 {
    htab[hash(u24(input, idx)) as usize] = idx;
    idx + 1
}

fn hash(v: u32) -> u16 {
    let hash = (v as u64 * 2654435769) >> 19;
    hash as u16 & 0x1fff
}

fn u24(input: &[u8], idx: u32) -> u32 {
    u32::from(input[idx as usize])
        + (u32::from(input[(idx + 1) as usize]) << 8)
        + (u32::from(input[(idx + 2) as usize]) << 16)

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::bytes;
    use rstest::rstest;

    #[rstest]
    #[case::empty(&[], 0)]
    #[case::thousand_zeros(&[0; 1000], 21)]
    #[case::thousand_forty_twos(&[42; 1000], 21)]
    #[case::short_hex(&bytes!("FACADE"), 4)]
    #[case::sample_contract_call(&bytes!("02f901550a758302df1483be21b88304743f94f80e51afb613d764fa61751affd3313c190a86bb870151bd62fd12adb8e41ef24f3f000000000000000000000000000000000000000000000000000000000000006e000000000000000000000000af88d065e77c8cc2239327c5edb3a432268e5831000000000000000000000000000000000000000000000000000000000003c1e5000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000148c89ed219d02f1a5be012c689b4f5b731827bebe000000000000000000000000c001a033fd89cb37c31b2cba46b6466e040c61fc9b2a3675a7f5f493ebd5ad77c497f8a07cdf65680e238392693019b4092f610222e71b7cec06449cb922b93b6a12744e"), 202)]
    #[case::base_0x5dadeb52979f29fc7a7494c43fdabc5be1d8ff404f3aafe93d729fa8e5d00769(&bytes!("b9047c02f904788221050883036ee48409c6c87383037f6f941195cf65f83b3a5768f3c496d3a05ad6412c64b78644364c5bb000b90404d123b4d80000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000038000000000000000000000000000000000f6476f90447748c19248ccaa31e6b8bfda4eb9d830f5f47df7f0998f7c2123d9e6137761b75d3184efb0f788e3b14516000000000000000000000000000000000000000000000000000044364c5bb000000000000000000000000000f38e53bd45c8225a7c94b513beadaa7afe5d222d0000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000024000000000000000000000000000000000000000000000000000000000000002a000000000000000000000000000000000000000000000000000000000000002c000000000000000000000000000000000000000000000000000000000000002e0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000084d6574614d61736b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000035697066733a2f2f516d656852577a743347745961776343347564745657557233454c587261436746434259416b66507331696f48610000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000cd0d83d9e840f8e27d5c2e365fd365ff1c05b2480000000000000000000000000000000000000000000000000000000000000ce40000000000000000000000000000000000000000000000000000000000000041e4480d358dbae20880960a0a464d63b06565a0c9f9b1b37aa94b522247b23ce149c81359bf4239d1a879eeb41047ec710c15f5c0f67453da59a383e6abd742971c00000000000000000000000000000000000000000000000000000000000000c001a0b57f0ff8516ea29cb26a44ac5055a5420847d1e16a8e7b03b70f0c02291ff2d5a00ad3771e5f39ccacfff0faa8c5d25ef7a1c179f79e66e828ffddcb994c8b512e"), 471)]
    fn test_flz_compress_len(#[case] input: &[u8], #[case] expected: u32) {
        assert_eq!(flz_compress_len(input), expected);
    }

    #[test]
    fn test_flz_compress_len_no_repeats() {
        let mut input = Vec::new();
        let mut len = 0;

        for i in 0..256 {
            input.push(i as u8);
            let prev_len = len;
            len = flz_compress_len(&input);
            assert!(len > prev_len);
        }
    }
}
