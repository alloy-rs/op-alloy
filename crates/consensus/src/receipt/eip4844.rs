#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::borrow::Borrow;

use alloy_consensus::{Eip658Value, Receipt, TxReceipt};
use alloy_primitives::{Bloom, Log};
use alloy_rlp::{BufMut, Decodable, Encodable};
use derive_more::{AsRef, Deref};

use crate::OpTxReceipt;

/// Receipt containing result of transaction execution.
#[derive(Clone, Debug, Deref, AsRef, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct OpEip4844Receipt<T = Log> {
    /// The inner receipt type.
    #[deref]
    #[as_ref]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub inner: Receipt<T>,
    /// Deposit nonce for Optimism deposit transactions
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )
    )]
    pub l1_blob_base_fee: Option<u128>,
    /// L1 blob base fee scalar for transaction
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )
    )]
    pub l1_blob_base_fee_scalar: Option<u128>,
}

impl<T> TxReceipt<T> for OpEip4844Receipt<T>
where
    T: Borrow<Log>,
{
    fn status_or_post_state(&self) -> &Eip658Value {
        self.inner.status_or_post_state()
    }

    fn status(&self) -> bool {
        self.inner.status()
    }

    fn bloom(&self) -> Bloom {
        self.inner.bloom_slow()
    }

    fn cumulative_gas_used(&self) -> u128 {
        self.inner.cumulative_gas_used()
    }

    fn logs(&self) -> &[T] {
        self.inner.logs()
    }
}

impl<T> OpTxReceipt<T> for OpEip4844Receipt<T> where T: Borrow<Log> {}

#[cfg(all(test, feature = "arbitrary"))]
impl<'a, T> arbitrary::Arbitrary<'a> for OpEip4844Receipt<T>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let l1_blob_base_fee = Option::<u128>::arbitrary(u)?;
        let l1_blob_base_fee_scalar =
            l1_blob_base_fee.is_some().then(|| u128::arbitrary(u)).transpose()?;
        Ok(Self {
            inner: Receipt {
                status: Eip658Value::arbitrary(u)?,
                cumulative_gas_used: u128::arbitrary(u)?,
                logs: Vec::<T>::arbitrary(u)?,
            },
            l1_blob_base_fee,
            l1_blob_base_fee_scalar,
        })
    }
}

/// [`OpEip4844Receipt`] with calculated bloom filter, modified for the OP Stack.
///
/// This convenience type allows us to lazily calculate the bloom filter for a
/// receipt, similar to [`Sealed`].
///
/// [`Sealed`]: alloy_consensus::Sealed
#[derive(Clone, Debug, Deref, AsRef, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct OpEip4844ReceiptWithBloom<T = Log> {
    /// The receipt.
    #[deref]
    #[as_ref(forward)]
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub receipt: OpEip4844Receipt<T>,
    /// The bloom filter.
    pub logs_bloom: Bloom,
}

impl<T> OpEip4844ReceiptWithBloom<T>
where
    T: alloy_rlp::Encodable,
{
    fn payload_len(&self) -> usize {
        self.receipt.inner.status.length()
            + self.receipt.inner.cumulative_gas_used.length()
            + self.logs_bloom.length()
            + self.receipt.inner.logs.length()
            + self.receipt.l1_blob_base_fee.map_or(0, |nonce| nonce.length())
            + self.receipt.l1_blob_base_fee_scalar.map_or(0, |version| version.length())
    }
}

impl<T> TxReceipt<T> for OpEip4844ReceiptWithBloom<T>
where
    T: Borrow<Log>,
{
    fn status_or_post_state(&self) -> &Eip658Value {
        self.inner.status_or_post_state()
    }

    fn status(&self) -> bool {
        self.inner.status()
    }

    fn bloom(&self) -> Bloom {
        self.inner.bloom_slow()
    }

    fn cumulative_gas_used(&self) -> u128 {
        self.inner.cumulative_gas_used()
    }

    fn logs(&self) -> &[T] {
        self.inner.logs()
    }
}

impl<T> OpTxReceipt<T> for OpEip4844ReceiptWithBloom<T> where T: Borrow<Log> {}

impl<T> alloy_rlp::Encodable for OpEip4844ReceiptWithBloom<T>
where
    T: alloy_rlp::Encodable,
{
    fn encode(&self, out: &mut dyn BufMut) {
        alloy_rlp::Header { list: true, payload_length: self.payload_len() }.encode(out);
        self.receipt.inner.status.encode(out);
        self.receipt.inner.cumulative_gas_used.encode(out);
        self.logs_bloom.encode(out);
        self.receipt.inner.logs.encode(out);
        if let Some(fee) = self.receipt.l1_blob_base_fee {
            fee.encode(out);
        }
        if let Some(scalar) = self.receipt.l1_blob_base_fee_scalar {
            scalar.encode(out);
        }
    }

    fn length(&self) -> usize {
        let payload_length = self.payload_len();
        payload_length + alloy_rlp::length_of_length(payload_length)
    }
}

impl<T> alloy_rlp::Decodable for OpEip4844ReceiptWithBloom<T>
where
    T: alloy_rlp::Decodable,
{
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let b: &mut &[u8] = &mut &**buf;
        let rlp_head = alloy_rlp::Header::decode(b)?;
        if !rlp_head.list {
            return Err(alloy_rlp::Error::UnexpectedString);
        }
        let started_len = b.len();

        let success = Decodable::decode(b)?;
        let cumulative_gas_used = Decodable::decode(b)?;
        let bloom = Decodable::decode(b)?;
        let logs = Decodable::decode(b)?;

        let remaining = |b: &[u8]| rlp_head.payload_length - (started_len - b.len()) > 0;
        let l1_blob_base_fee = remaining(b).then(|| alloy_rlp::Decodable::decode(b)).transpose()?;
        let l1_blob_base_fee_scalar =
            remaining(b).then(|| alloy_rlp::Decodable::decode(b)).transpose()?;

        let receipt = OpEip4844Receipt {
            inner: Receipt { status: success, cumulative_gas_used, logs },
            l1_blob_base_fee,
            l1_blob_base_fee_scalar,
        };

        let this = Self { receipt, logs_bloom: bloom };
        let consumed = started_len - b.len();
        if consumed != rlp_head.payload_length {
            return Err(alloy_rlp::Error::ListLengthMismatch {
                expected: rlp_head.payload_length,
                got: consumed,
            });
        }
        *buf = *b;
        Ok(this)
    }
}

impl<T> From<OpEip4844Receipt<T>> for OpEip4844ReceiptWithBloom<T>
where
    T: Borrow<Log>,
{
    fn from(receipt: OpEip4844Receipt<T>) -> Self {
        let logs_bloom = receipt.bloom_slow();
        OpEip4844ReceiptWithBloom { receipt, logs_bloom }
    }
}

#[cfg(all(test, feature = "arbitrary"))]
impl<'a, T> arbitrary::Arbitrary<'a> for OpEip4844ReceiptWithBloom<T>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(Self { receipt: OpEip4844Receipt::<T>::arbitrary(u)?, logs_bloom: Bloom::arbitrary(u)? })
    }
}
