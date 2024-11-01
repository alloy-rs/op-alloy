//! Optimism specific types related to transactions.

use alloy_consensus::Transaction as ConsensusTransaction;
use alloy_eips::{eip2930::AccessList, eip7702::SignedAuthorization};
use alloy_primitives::{Address, BlockHash, Bytes, ChainId, TxKind, B256, U256};
use alloy_serde::OtherFields;
use op_alloy_consensus::OpTxEnvelope;
use serde::{Deserialize, Serialize};

mod request;
pub use request::OpTransactionRequest;

/// OP Transaction type
#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut,
)]
#[serde(try_from = "tx_serde::TransactionSerdeHelper", into = "tx_serde::TransactionSerdeHelper")]
#[cfg_attr(all(any(test, feature = "arbitrary"), feature = "k256"), derive(arbitrary::Arbitrary))]
pub struct Transaction {
    /// Ethereum Transaction Types
    #[deref]
    #[deref_mut]
    pub inner: alloy_rpc_types_eth::Transaction<OpTxEnvelope>,

    /// Deposit receipt version for deposit transactions post-canyon
    pub deposit_receipt_version: Option<u64>,
}

impl ConsensusTransaction for Transaction {
    fn chain_id(&self) -> Option<ChainId> {
        self.inner.chain_id()
    }

    fn nonce(&self) -> u64 {
        self.inner.nonce()
    }

    fn gas_limit(&self) -> u64 {
        self.inner.gas_limit()
    }

    fn gas_price(&self) -> Option<u128> {
        self.inner.gas_price()
    }

    fn max_fee_per_gas(&self) -> u128 {
        self.inner.max_fee_per_gas()
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        self.inner.max_priority_fee_per_gas()
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        self.inner.max_fee_per_blob_gas()
    }

    fn priority_fee_or_price(&self) -> u128 {
        self.inner.priority_fee_or_price()
    }

    fn to(&self) -> Option<Address> {
        self.inner.to()
    }

    fn kind(&self) -> TxKind {
        self.inner.kind()
    }

    fn value(&self) -> U256 {
        self.inner.value()
    }

    fn input(&self) -> &Bytes {
        self.inner.input()
    }

    fn ty(&self) -> u8 {
        self.inner.ty()
    }

    fn access_list(&self) -> Option<&AccessList> {
        self.inner.access_list()
    }

    fn blob_versioned_hashes(&self) -> Option<&[B256]> {
        self.inner.blob_versioned_hashes()
    }

    fn authorization_list(&self) -> Option<&[SignedAuthorization]> {
        self.inner.authorization_list()
    }
}

impl alloy_network_primitives::TransactionResponse for Transaction {
    fn tx_hash(&self) -> alloy_primitives::TxHash {
        self.inner.tx_hash()
    }

    fn block_hash(&self) -> Option<BlockHash> {
        self.inner.block_hash()
    }

    fn block_number(&self) -> Option<u64> {
        self.inner.block_number()
    }

    fn transaction_index(&self) -> Option<u64> {
        self.inner.transaction_index()
    }

    fn from(&self) -> alloy_primitives::Address {
        self.inner.from()
    }

    fn to(&self) -> Option<alloy_primitives::Address> {
        ConsensusTransaction::to(&self.inner)
    }
}

/// Optimism specific transaction fields
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[doc(alias = "OptimismTxFields")]
#[serde(rename_all = "camelCase")]
pub struct OpTransactionFields {
    /// The ETH value to mint on L2
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub mint: Option<u128>,
    /// Hash that uniquely identifies the source of the deposit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_hash: Option<B256>,
    /// Field indicating whether the transaction is a system transaction, and therefore
    /// exempt from the L2 gas limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_system_tx: Option<bool>,
    /// Deposit receipt version for deposit transactions post-canyon
    #[serde(default, skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")]
    pub deposit_receipt_version: Option<u64>,
}

impl From<OpTransactionFields> for OtherFields {
    fn from(value: OpTransactionFields) -> Self {
        serde_json::to_value(value).unwrap().try_into().unwrap()
    }
}

impl AsRef<OpTxEnvelope> for Transaction {
    fn as_ref(&self) -> &OpTxEnvelope {
        self.inner.as_ref()
    }
}

mod tx_serde {
    use super::*;
    use serde::de::Error;

    #[derive(Serialize, Deserialize)]
    struct MaybeFrom {
        from: Option<Address>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct TransactionSerdeHelper {
        #[serde(flatten)]
        inner: OpTxEnvelope,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        block_hash: Option<BlockHash>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]
        block_number: Option<u64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]
        transaction_index: Option<u64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "alloy_serde::quantity::opt"
        )]
        deposit_receipt_version: Option<u64>,

        #[serde(flatten)]
        from: MaybeFrom,
    }

    impl From<Transaction> for TransactionSerdeHelper {
        fn from(value: Transaction) -> Self {
            let Transaction {
                inner:
                    alloy_rpc_types_eth::Transaction {
                        inner,
                        block_hash,
                        block_number,
                        transaction_index,
                        from,
                    },
                deposit_receipt_version,
            } = value;

            let from = if matches!(inner, OpTxEnvelope::Deposit(_)) { None } else { Some(from) };

            Self {
                inner,
                block_hash,
                block_number,
                transaction_index,
                deposit_receipt_version,
                from: MaybeFrom { from },
            }
        }
    }

    impl TryFrom<TransactionSerdeHelper> for Transaction {
        type Error = serde_json::Error;

        fn try_from(value: TransactionSerdeHelper) -> Result<Self, Self::Error> {
            let TransactionSerdeHelper {
                inner,
                block_hash,
                block_number,
                transaction_index,
                deposit_receipt_version,
                from,
            } = value;

            let from = if let Some(from) = from.from {
                from
            } else if let OpTxEnvelope::Deposit(tx) = &inner {
                tx.from
            } else {
                return Err(serde_json::Error::custom("missing `from` field"));
            };

            Ok(Self {
                inner: alloy_rpc_types_eth::Transaction {
                    inner,
                    block_hash,
                    block_number,
                    transaction_index,
                    from,
                },
                deposit_receipt_version,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deserialize_deposit() {
        // cast rpc eth_getTransactionByHash
        // 0xbc9329afac05556497441e2b3ee4c5d4da7ca0b2a4c212c212d0739e94a24df9 --rpc-url optimism
        let rpc_tx = r#"{"blockHash":"0x9d86bb313ebeedf4f9f82bf8a19b426be656a365648a7c089b618771311db9f9","blockNumber":"0x798ad0b","hash":"0xbc9329afac05556497441e2b3ee4c5d4da7ca0b2a4c212c212d0739e94a24df9","transactionIndex":"0x0","type":"0x7e","nonce":"0x152ea95","input":"0x440a5e200000146b000f79c50000000000000003000000006725333f000000000141e287000000000000000000000000000000000000000000000000000000012439ee7e0000000000000000000000000000000000000000000000000000000063f363e973e96e7145ff001c81b9562cba7b6104eeb12a2bc4ab9f07c27d45cd81a986620000000000000000000000006887246668a3b87f54deb3b94ba47a6f63f32985","mint":"0x0","sourceHash":"0x04e9a69416471ead93b02f0c279ab11ca0b635db5c1726a56faf22623bafde52","r":"0x0","s":"0x0","v":"0x0","gas":"0xf4240","from":"0xdeaddeaddeaddeaddeaddeaddeaddeaddead0001","to":"0x4200000000000000000000000000000000000015","depositReceiptVersion":"0x1","value":"0x0","gasPrice":"0x0"}"#;

        let tx = serde_json::from_str::<Transaction>(rpc_tx).unwrap();

        let OpTxEnvelope::Deposit(inner) = tx.as_ref() else {
            panic!("Expected deposit transaction");
        };
        assert_eq!(tx.from, inner.from);
    }
}
