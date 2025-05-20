#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub use alloy_network::*;

use alloy_consensus::{TxEnvelope, TxType, TypedTransaction};
use alloy_primitives::{Address, Bytes, ChainId, TxKind, U256};
use alloy_rpc_types_eth::{AccessList, TransactionInput, TransactionRequest};
use op_alloy_consensus::{OpTxEnvelope, OpTxType, OpTypedTransaction};

/// Types for an Op-stack network.
#[derive(Clone, Copy, Debug)]
pub struct Optimism {
    _private: (),
}

impl Network for Optimism {
    type TxType = OpTxType;

    type TxEnvelope = op_alloy_consensus::OpTxEnvelope;

    type UnsignedTx = op_alloy_consensus::OpTypedTransaction;

    type ReceiptEnvelope = op_alloy_consensus::OpReceiptEnvelope;

    type Header = alloy_consensus::Header;

    type TransactionRequest = alloy_rpc_types_eth::TransactionRequest;

    type TransactionResponse = op_alloy_rpc_types::Transaction;

    type ReceiptResponse = op_alloy_rpc_types::OpTransactionReceipt;

    type HeaderResponse = alloy_rpc_types_eth::Header;

    type BlockResponse =
        alloy_rpc_types_eth::Block<Self::TransactionResponse, Self::HeaderResponse>;
}

impl TransactionBuilder<Optimism> for TransactionRequest {
    fn chain_id(&self) -> Option<ChainId> {
        self.chain_id
    }

    fn set_chain_id(&mut self, chain_id: ChainId) {
        self.chain_id = Some(chain_id);
    }

    fn nonce(&self) -> Option<u64> {
        self.nonce
    }

    fn set_nonce(&mut self, nonce: u64) {
        self.nonce = Some(nonce);
    }

    fn input(&self) -> Option<&Bytes> {
        self.input.input()
    }

    fn set_input<T: Into<Bytes>>(&mut self, input: T) {
        self.input = TransactionInput::both(input.into())
    }

    fn from(&self) -> Option<Address> {
        self.from
    }

    fn set_from(&mut self, from: Address) {
        self.from = Some(from);
    }

    fn kind(&self) -> Option<TxKind> {
        self.to
    }

    fn clear_kind(&mut self) {
        self.to = None;
    }

    fn set_kind(&mut self, kind: TxKind) {
        self.to = Some(kind);
    }

    fn value(&self) -> Option<U256> {
        self.value
    }

    fn set_value(&mut self, value: U256) {
        self.value = Some(value);
    }

    fn gas_price(&self) -> Option<u128> {
        self.gas_price
    }

    fn set_gas_price(&mut self, gas_price: u128) {
        self.gas_price = Some(gas_price);
    }

    fn max_fee_per_gas(&self) -> Option<u128> {
        self.max_fee_per_gas
    }

    fn set_max_fee_per_gas(&mut self, max_fee_per_gas: u128) {
        self.max_fee_per_gas = Some(max_fee_per_gas);
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        self.max_priority_fee_per_gas
    }

    fn set_max_priority_fee_per_gas(&mut self, max_priority_fee_per_gas: u128) {
        self.max_priority_fee_per_gas = Some(max_priority_fee_per_gas);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.gas
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.gas = Some(gas_limit);
    }

    fn access_list(&self) -> Option<&AccessList> {
        self.access_list.as_ref()
    }

    fn set_access_list(&mut self, access_list: AccessList) {
        self.access_list = Some(access_list);
    }

    fn complete_type(&self, ty: OpTxType) -> Result<(), Vec<&'static str>> {
        match ty {
            OpTxType::Deposit => Err(vec!["not implemented for deposit tx"]),
            _ => {
                let ty = TxType::try_from(ty as u8).unwrap();
                match ty {
                    TxType::Legacy => self.complete_legacy(),
                    TxType::Eip2930 => self.complete_2930(),
                    TxType::Eip1559 => self.complete_1559(),
                    TxType::Eip4844 => Err(vec!["cannot build eip4844 tx"]),
                    TxType::Eip7702 => self.complete_7702(),
                }
            }
        }
    }

    fn can_submit(&self) -> bool {
        // value and data may be None. If they are, they will be set to default.
        // gas fields and nonce may be None, if they are, they will be populated
        // with default values by the RPC server
        self.from.is_some()
    }

    fn can_build(&self) -> bool {
        // value and data may be none. If they are, they will be set to default
        // values.

        // chain_id and from may be none.
        let common = self.gas.is_some() && self.nonce.is_some();

        let legacy = self.gas_price.is_some();
        let eip2930 = legacy && self.access_list.is_some();

        let eip1559 = self.max_fee_per_gas.is_some() && self.max_priority_fee_per_gas.is_some();

        let eip4844 = eip1559 && self.sidecar.is_some() && self.to.is_some();

        let eip7702 = eip1559 && self.authorization_list().is_some();

        // required deposit fields
        let deposit = self.value.is_some()
            && self.from.is_some()
            && self.to.is_some()
            && (self.input.data.is_some() || self.input.input.is_some());
        // cannot build is eip4844 fields are set.
        common && (legacy || eip2930 || eip1559 || eip7702 || deposit) && !eip4844
    }

    #[doc(alias = "output_transaction_type")]
    fn output_tx_type(&self) -> OpTxType {
        match self.preferred_type() {
            TxType::Eip1559 | TxType::Eip4844 => OpTxType::Eip1559,
            TxType::Eip2930 => OpTxType::Eip2930,
            TxType::Eip7702 => OpTxType::Eip7702,
            TxType::Legacy => OpTxType::Legacy,
        }
    }

    #[doc(alias = "output_transaction_type_checked")]
    fn output_tx_type_checked(&self) -> Option<OpTxType> {
        self.buildable_type().map(|tx_ty| match tx_ty {
            TxType::Eip1559 | TxType::Eip4844 => OpTxType::Eip1559,
            TxType::Eip2930 => OpTxType::Eip2930,
            TxType::Eip7702 => OpTxType::Eip7702,
            TxType::Legacy => OpTxType::Legacy,
        })
    }

    fn prep_for_submission(&mut self) {
        self.transaction_type = Some(self.preferred_type() as u8);
        self.trim_conflicting_keys();
    }

    fn build_unsigned(self) -> BuildResult<OpTypedTransaction, Optimism> {
        if let Err((tx_type, missing)) = self.missing_keys() {
            let tx_type = OpTxType::try_from(tx_type as u8).unwrap();
            return Err(TransactionBuilderError::InvalidTransactionRequest(tx_type, missing)
                .into_unbuilt(self));
        }
        todo!()
    }

    async fn build<W: NetworkWallet<Optimism>>(
        self,
        wallet: &W,
    ) -> Result<<Optimism as Network>::TxEnvelope, TransactionBuilderError<Optimism>> {
        Ok(wallet.sign_request(self).await?)
    }
}

impl NetworkWallet<Optimism> for EthereumWallet {
    fn default_signer_address(&self) -> Address {
        NetworkWallet::<Ethereum>::default_signer_address(self)
    }

    fn has_signer_for(&self, address: &Address) -> bool {
        NetworkWallet::<Ethereum>::has_signer_for(self, address)
    }

    fn signer_addresses(&self) -> impl Iterator<Item = Address> {
        NetworkWallet::<Ethereum>::signer_addresses(self)
    }

    async fn sign_transaction_from(
        &self,
        sender: Address,
        tx: OpTypedTransaction,
    ) -> alloy_signer::Result<OpTxEnvelope> {
        let tx = match tx {
            OpTypedTransaction::Legacy(tx) => TypedTransaction::Legacy(tx),
            OpTypedTransaction::Eip2930(tx) => TypedTransaction::Eip2930(tx),
            OpTypedTransaction::Eip1559(tx) => TypedTransaction::Eip1559(tx),
            OpTypedTransaction::Eip7702(tx) => TypedTransaction::Eip7702(tx),
            OpTypedTransaction::Deposit(_) => {
                return Err(alloy_signer::Error::other("not implemented for deposit tx"));
            }
        };
        let tx = NetworkWallet::<Ethereum>::sign_transaction_from(self, sender, tx).await?;

        Ok(match tx {
            TxEnvelope::Eip1559(tx) => OpTxEnvelope::Eip1559(tx),
            TxEnvelope::Eip2930(tx) => OpTxEnvelope::Eip2930(tx),
            TxEnvelope::Eip7702(tx) => OpTxEnvelope::Eip7702(tx),
            TxEnvelope::Legacy(tx) => OpTxEnvelope::Legacy(tx),
            _ => unreachable!(),
        })
    }
}
