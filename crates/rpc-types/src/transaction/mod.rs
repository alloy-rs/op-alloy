//! Optimism specific types related to transactions.

use alloy_primitives::B256;

pub mod tx_type;
pub use tx_type::TxType;

/// OP Transaction type
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(any(test, feature = "arbitrary"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Transaction {
    /// Ethereum Transaction Types
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub inner: alloy_rpc_types::Transaction,
    /// The ETH value to mint on L2
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub mint: Option<u128>,
    /// Hash that uniquely identifies the source of the deposit.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub source_hash: Option<B256>,
    /// Field indicating whether the transaction is a system transaction, and therefore
    /// exempt from the L2 gas limit.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub is_system_tx: Option<bool>,
    /// Deposit receipt version for deposit transactions post-canyon
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    pub deposit_receipt_version: Option<u64>,
}
