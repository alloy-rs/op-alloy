#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use core::borrow::Borrow;

use alloy_consensus::{Eip658Value, Receipt, TxReceipt};
use alloy_primitives::{Bloom, Log};
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
