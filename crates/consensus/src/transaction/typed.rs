use crate::{OpTxEnvelope, OpTxType, TxDeposit};
use alloy_consensus::{
    transaction::RlpEcdsaTx, SignableTransaction, Signed, Transaction, TxEip1559, TxEip2930,
    TxEip7702, TxLegacy,
};
use alloy_eips::eip2930::AccessList;
use alloy_primitives::{Address, Bytes, ChainId, PrimitiveSignature as Signature, TxKind, B256};

/// The TypedTransaction enum represents all Ethereum transaction request types, modified for the OP
/// Stack.
///
/// Its variants correspond to specific allowed transactions:
/// 1. Legacy (pre-EIP2718) [`TxLegacy`]
/// 2. EIP2930 (state access lists) [`TxEip2930`]
/// 3. EIP1559 [`TxEip1559`]
/// 4. Deposit [`TxDeposit`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        from = "serde_from::MaybeTaggedTypedTransaction",
        into = "serde_from::TaggedTypedTransaction"
    )
)]
pub enum OpTypedTransaction {
    /// Legacy transaction
    Legacy(TxLegacy),
    /// EIP-2930 transaction
    Eip2930(TxEip2930),
    /// EIP-1559 transaction
    Eip1559(TxEip1559),
    /// EIP-7702 transaction
    Eip7702(TxEip7702),
    /// Optimism deposit transaction
    Deposit(TxDeposit),
}

impl From<TxLegacy> for OpTypedTransaction {
    fn from(tx: TxLegacy) -> Self {
        Self::Legacy(tx)
    }
}

impl From<TxEip2930> for OpTypedTransaction {
    fn from(tx: TxEip2930) -> Self {
        Self::Eip2930(tx)
    }
}

impl From<TxEip1559> for OpTypedTransaction {
    fn from(tx: TxEip1559) -> Self {
        Self::Eip1559(tx)
    }
}

impl From<TxDeposit> for OpTypedTransaction {
    fn from(tx: TxDeposit) -> Self {
        Self::Deposit(tx)
    }
}

impl From<OpTxEnvelope> for OpTypedTransaction {
    fn from(envelope: OpTxEnvelope) -> Self {
        match envelope {
            OpTxEnvelope::Legacy(tx) => Self::Legacy(tx.strip_signature()),
            OpTxEnvelope::Eip2930(tx) => Self::Eip2930(tx.strip_signature()),
            OpTxEnvelope::Eip1559(tx) => Self::Eip1559(tx.strip_signature()),
            OpTxEnvelope::Eip7702(tx) => Self::Eip7702(tx.strip_signature()),
            OpTxEnvelope::Deposit(tx) => Self::Deposit(tx.into_inner()),
        }
    }
}

impl OpTypedTransaction {
    /// Return the [`OpTxType`] of the inner txn.
    pub const fn tx_type(&self) -> OpTxType {
        match self {
            Self::Legacy(_) => OpTxType::Legacy,
            Self::Eip2930(_) => OpTxType::Eip2930,
            Self::Eip1559(_) => OpTxType::Eip1559,
            Self::Eip7702(_) => OpTxType::Eip7702,
            Self::Deposit(_) => OpTxType::Deposit,
        }
    }

    /// Return the inner legacy transaction if it exists.
    pub const fn legacy(&self) -> Option<&TxLegacy> {
        match self {
            Self::Legacy(tx) => Some(tx),
            _ => None,
        }
    }

    /// Return the inner EIP-2930 transaction if it exists.
    pub const fn eip2930(&self) -> Option<&TxEip2930> {
        match self {
            Self::Eip2930(tx) => Some(tx),
            _ => None,
        }
    }

    /// Return the inner EIP-1559 transaction if it exists.
    pub const fn eip1559(&self) -> Option<&TxEip1559> {
        match self {
            Self::Eip1559(tx) => Some(tx),
            _ => None,
        }
    }

    /// Return the inner deposit transaction if it exists.
    pub const fn deposit(&self) -> Option<&TxDeposit> {
        match self {
            Self::Deposit(tx) => Some(tx),
            _ => None,
        }
    }

    fn delegate_signable_mut<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut dyn SignableTransaction<Signature>) -> R,
    {
        match self {
            Self::Legacy(tx) => Some(f(tx)),
            Self::Eip2930(tx) => Some(f(tx)),
            Self::Eip1559(tx) => Some(f(tx)),
            Self::Eip7702(tx) => Some(f(tx)),
            Self::Deposit(_) => None,
        }
    }

    fn delegate_signable<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&dyn SignableTransaction<Signature>) -> R,
    {
        match self {
            Self::Legacy(tx) => Some(f(tx)),
            Self::Eip2930(tx) => Some(f(tx)),
            Self::Eip1559(tx) => Some(f(tx)),
            Self::Eip7702(tx) => Some(f(tx)),
            Self::Deposit(_) => None,
        }
    }
}

impl Transaction for OpTypedTransaction {
    fn chain_id(&self) -> Option<alloy_primitives::ChainId> {
        match self {
            Self::Legacy(tx) => tx.chain_id(),
            Self::Eip2930(tx) => tx.chain_id(),
            Self::Eip1559(tx) => tx.chain_id(),
            Self::Eip7702(tx) => tx.chain_id(),
            Self::Deposit(tx) => tx.chain_id(),
        }
    }

    fn is_create(&self) -> bool {
        match self {
            Self::Legacy(tx) => tx.is_create(),
            Self::Eip2930(tx) => tx.is_create(),
            Self::Eip1559(tx) => tx.is_create(),
            Self::Eip7702(tx) => tx.is_create(),
            Self::Deposit(tx) => tx.is_create(),
        }
    }

    fn nonce(&self) -> u64 {
        match self {
            Self::Legacy(tx) => tx.nonce(),
            Self::Eip2930(tx) => tx.nonce(),
            Self::Eip1559(tx) => tx.nonce(),
            Self::Eip7702(tx) => tx.nonce(),
            Self::Deposit(tx) => tx.nonce(),
        }
    }

    fn gas_limit(&self) -> u64 {
        match self {
            Self::Legacy(tx) => tx.gas_limit(),
            Self::Eip2930(tx) => tx.gas_limit(),
            Self::Eip1559(tx) => tx.gas_limit(),
            Self::Eip7702(tx) => tx.gas_limit(),
            Self::Deposit(tx) => tx.gas_limit(),
        }
    }

    fn gas_price(&self) -> Option<u128> {
        match self {
            Self::Legacy(tx) => tx.gas_price(),
            Self::Eip2930(tx) => tx.gas_price(),
            Self::Eip1559(tx) => tx.gas_price(),
            Self::Eip7702(tx) => tx.gas_price(),
            Self::Deposit(tx) => tx.gas_price(),
        }
    }

    fn max_fee_per_gas(&self) -> u128 {
        match self {
            Self::Legacy(tx) => tx.max_fee_per_gas(),
            Self::Eip2930(tx) => tx.max_fee_per_gas(),
            Self::Eip1559(tx) => tx.max_fee_per_gas(),
            Self::Eip7702(tx) => tx.max_fee_per_gas(),
            Self::Deposit(tx) => tx.max_fee_per_gas(),
        }
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        match self {
            Self::Legacy(tx) => tx.max_priority_fee_per_gas(),
            Self::Eip2930(tx) => tx.max_priority_fee_per_gas(),
            Self::Eip1559(tx) => tx.max_priority_fee_per_gas(),
            Self::Eip7702(tx) => tx.max_priority_fee_per_gas(),
            Self::Deposit(tx) => tx.max_priority_fee_per_gas(),
        }
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        match self {
            Self::Legacy(tx) => tx.max_fee_per_blob_gas(),
            Self::Eip2930(tx) => tx.max_fee_per_blob_gas(),
            Self::Eip1559(tx) => tx.max_fee_per_blob_gas(),
            Self::Eip7702(tx) => tx.max_fee_per_blob_gas(),
            Self::Deposit(tx) => tx.max_fee_per_blob_gas(),
        }
    }

    fn priority_fee_or_price(&self) -> u128 {
        match self {
            Self::Legacy(tx) => tx.priority_fee_or_price(),
            Self::Eip2930(tx) => tx.priority_fee_or_price(),
            Self::Eip1559(tx) => tx.priority_fee_or_price(),
            Self::Eip7702(tx) => tx.priority_fee_or_price(),
            Self::Deposit(tx) => tx.priority_fee_or_price(),
        }
    }

    fn to(&self) -> Option<Address> {
        match self {
            Self::Legacy(tx) => tx.to(),
            Self::Eip2930(tx) => tx.to(),
            Self::Eip1559(tx) => tx.to(),
            Self::Eip7702(tx) => tx.to(),
            Self::Deposit(tx) => tx.to(),
        }
    }

    fn kind(&self) -> TxKind {
        match self {
            Self::Legacy(tx) => tx.kind(),
            Self::Eip2930(tx) => tx.kind(),
            Self::Eip1559(tx) => tx.kind(),
            Self::Eip7702(tx) => tx.kind(),
            Self::Deposit(tx) => tx.kind(),
        }
    }

    fn value(&self) -> alloy_primitives::U256 {
        match self {
            Self::Legacy(tx) => tx.value(),
            Self::Eip2930(tx) => tx.value(),
            Self::Eip1559(tx) => tx.value(),
            Self::Eip7702(tx) => tx.value(),
            Self::Deposit(tx) => tx.value(),
        }
    }

    fn input(&self) -> &Bytes {
        match self {
            Self::Legacy(tx) => tx.input(),
            Self::Eip2930(tx) => tx.input(),
            Self::Eip1559(tx) => tx.input(),
            Self::Eip7702(tx) => tx.input(),
            Self::Deposit(tx) => tx.input(),
        }
    }

    fn ty(&self) -> u8 {
        match self {
            Self::Legacy(_) => OpTxType::Legacy as u8,
            Self::Eip2930(_) => OpTxType::Eip2930 as u8,
            Self::Eip1559(_) => OpTxType::Eip1559 as u8,
            Self::Eip7702(_) => OpTxType::Eip7702 as u8,
            Self::Deposit(_) => OpTxType::Deposit as u8,
        }
    }

    fn access_list(&self) -> Option<&AccessList> {
        match self {
            Self::Legacy(tx) => tx.access_list(),
            Self::Eip2930(tx) => tx.access_list(),
            Self::Eip1559(tx) => tx.access_list(),
            Self::Eip7702(tx) => tx.access_list(),
            Self::Deposit(tx) => tx.access_list(),
        }
    }

    fn blob_versioned_hashes(&self) -> Option<&[alloy_primitives::B256]> {
        match self {
            Self::Legacy(tx) => tx.blob_versioned_hashes(),
            Self::Eip2930(tx) => tx.blob_versioned_hashes(),
            Self::Eip1559(tx) => tx.blob_versioned_hashes(),
            Self::Eip7702(tx) => tx.blob_versioned_hashes(),
            Self::Deposit(tx) => tx.blob_versioned_hashes(),
        }
    }

    fn authorization_list(&self) -> Option<&[alloy_eips::eip7702::SignedAuthorization]> {
        match self {
            Self::Legacy(tx) => tx.authorization_list(),
            Self::Eip2930(tx) => tx.authorization_list(),
            Self::Eip1559(tx) => tx.authorization_list(),
            Self::Eip7702(tx) => tx.authorization_list(),
            Self::Deposit(tx) => tx.authorization_list(),
        }
    }

    fn is_dynamic_fee(&self) -> bool {
        match self {
            Self::Legacy(tx) => tx.is_dynamic_fee(),
            Self::Eip2930(tx) => tx.is_dynamic_fee(),
            Self::Eip1559(tx) => tx.is_dynamic_fee(),
            Self::Eip7702(tx) => tx.is_dynamic_fee(),
            Self::Deposit(tx) => tx.is_dynamic_fee(),
        }
    }

    fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        match self {
            Self::Legacy(tx) => tx.effective_gas_price(base_fee),
            Self::Eip2930(tx) => tx.effective_gas_price(base_fee),
            Self::Eip1559(tx) => tx.effective_gas_price(base_fee),
            Self::Eip7702(tx) => tx.effective_gas_price(base_fee),
            Self::Deposit(tx) => tx.effective_gas_price(base_fee),
        }
    }
}

impl SignableTransaction<Signature> for OpTypedTransaction {
    fn set_chain_id(&mut self, chain_id: ChainId) {
        self.delegate_signable_mut(|tx| tx.set_chain_id(chain_id)).unwrap_or(())
    }

    fn set_chain_id_checked(&mut self, chain_id: ChainId) -> bool {
        self.delegate_signable_mut(|tx| tx.set_chain_id_checked(chain_id)).unwrap_or(true)
    }

    fn encode_for_signing(&self, out: &mut dyn alloy_rlp::BufMut) {
        self.delegate_signable(|tx| tx.encode_for_signing(out)).unwrap_or(())
    }

    fn payload_len_for_signature(&self) -> usize {
        self.delegate_signable(|tx| tx.payload_len_for_signature()).unwrap_or(0)
    }

    fn encoded_for_signing(&self) -> Vec<u8> {
        self.delegate_signable(|tx| tx.encoded_for_signing()).unwrap_or_default()
    }

    fn signature_hash(&self) -> B256 {
        self.delegate_signable(|tx| tx.signature_hash()).unwrap_or(B256::ZERO)
    }

    fn into_signed(self, signature: Signature) -> Signed<Self, Signature>
    where
        Self: Sized,
    {
        match self {
            Self::Deposit(tx) => {
                Signed::new_unchecked(Self::Deposit(tx), TxDeposit::signature(), B256::ZERO)
            }
            Self::Legacy(tx) => {
                let hash = tx.tx_hash(&signature);
                Signed::new_unchecked(Self::Legacy(tx), signature, hash)
            }
            Self::Eip2930(tx) => {
                let hash = tx.tx_hash(&signature);
                Signed::new_unchecked(Self::Eip2930(tx), signature, hash)
            }
            Self::Eip1559(tx) => {
                let hash = tx.tx_hash(&signature);
                Signed::new_unchecked(Self::Eip1559(tx), signature, hash)
            }
            Self::Eip7702(tx) => {
                let hash = tx.tx_hash(&signature);
                Signed::new_unchecked(Self::Eip7702(tx), signature, hash)
            }
        }
    }
}

#[cfg(feature = "serde")]
mod serde_from {
    //! NB: Why do we need this?
    //!
    //! Because the tag may be missing, we need an abstraction over tagged (with
    //! type) and untagged (always legacy). This is
    //! [`MaybeTaggedTypedTransaction`].
    //!
    //! The tagged variant is [`TaggedTypedTransaction`], which always has a
    //! type tag.
    //!
    //! We serialize via [`TaggedTypedTransaction`] and deserialize via
    //! [`MaybeTaggedTypedTransaction`].
    use super::*;

    #[derive(Debug, serde::Deserialize)]
    #[serde(untagged)]
    pub(crate) enum MaybeTaggedTypedTransaction {
        Tagged(TaggedTypedTransaction),
        Untagged(TxLegacy),
    }

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "type")]
    pub(crate) enum TaggedTypedTransaction {
        /// Legacy transaction
        #[serde(rename = "0x00", alias = "0x0")]
        Legacy(TxLegacy),
        /// EIP-2930 transaction
        #[serde(rename = "0x01", alias = "0x1")]
        Eip2930(TxEip2930),
        /// EIP-1559 transaction
        #[serde(rename = "0x02", alias = "0x2")]
        Eip1559(TxEip1559),
        /// EIP-7702 transaction
        #[serde(rename = "0x04", alias = "0x4")]
        Eip7702(TxEip7702),
        /// Deposit transaction
        #[serde(rename = "0x7e", alias = "0x7E", serialize_with = "crate::serde_deposit_tx_rpc")]
        Deposit(TxDeposit),
    }

    impl From<MaybeTaggedTypedTransaction> for OpTypedTransaction {
        fn from(value: MaybeTaggedTypedTransaction) -> Self {
            match value {
                MaybeTaggedTypedTransaction::Tagged(tagged) => tagged.into(),
                MaybeTaggedTypedTransaction::Untagged(tx) => Self::Legacy(tx),
            }
        }
    }

    impl From<TaggedTypedTransaction> for OpTypedTransaction {
        fn from(value: TaggedTypedTransaction) -> Self {
            match value {
                TaggedTypedTransaction::Legacy(signed) => Self::Legacy(signed),
                TaggedTypedTransaction::Eip2930(signed) => Self::Eip2930(signed),
                TaggedTypedTransaction::Eip1559(signed) => Self::Eip1559(signed),
                TaggedTypedTransaction::Eip7702(signed) => Self::Eip7702(signed),
                TaggedTypedTransaction::Deposit(tx) => Self::Deposit(tx),
            }
        }
    }

    impl From<OpTypedTransaction> for TaggedTypedTransaction {
        fn from(value: OpTypedTransaction) -> Self {
            match value {
                OpTypedTransaction::Legacy(signed) => Self::Legacy(signed),
                OpTypedTransaction::Eip2930(signed) => Self::Eip2930(signed),
                OpTypedTransaction::Eip1559(signed) => Self::Eip1559(signed),
                OpTypedTransaction::Eip7702(signed) => Self::Eip7702(signed),
                OpTypedTransaction::Deposit(tx) => Self::Deposit(tx),
            }
        }
    }
}
