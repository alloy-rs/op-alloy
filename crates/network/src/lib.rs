#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use alloy_network::*;

use alloy_consensus::{ReceiptWithBloom, TxEnvelope, TxType, TypedTransaction};
use alloy_primitives::Address;
use op_alloy_consensus::{OpReceipt, OpTxEnvelope, OpTxType, OpTypedTransaction};
use op_alloy_rpc_types::OpTransactionRequest;

/// Types for an Op-stack network.
#[derive(Clone, Copy, Debug)]
pub struct Optimism {
    _private: (),
}

impl Network for Optimism {
    type TxType = OpTxType;

    type TxEnvelope = op_alloy_consensus::OpTxEnvelope;

    type UnsignedTx = op_alloy_consensus::OpTypedTransaction;

    type ReceiptEnvelope = ReceiptWithBloom<OpReceipt>;

    type Header = alloy_consensus::Header;

    type TransactionRequest = op_alloy_rpc_types::OpTransactionRequest;

    type TransactionResponse = op_alloy_rpc_types::Transaction;

    type ReceiptResponse = op_alloy_rpc_types::OpTransactionReceipt;

    type HeaderResponse = alloy_rpc_types_eth::Header;

    type BlockResponse =
        alloy_rpc_types_eth::Block<Self::TransactionResponse, Self::HeaderResponse>;
}

impl NetworkTransactionBuilder<Optimism> for OpTransactionRequest {
    fn complete_type(&self, ty: OpTxType) -> Result<(), Vec<&'static str>> {
        match ty {
            OpTxType::Deposit => Err(vec!["not implemented for deposit tx"]),
            _ => {
                let ty = TxType::try_from(ty as u8).unwrap();
                self.as_ref().complete_type(ty)
            }
        }
    }

    #[doc(alias = "output_transaction_type")]
    fn output_tx_type(&self) -> OpTxType {
        match self.as_ref().preferred_type() {
            TxType::Eip1559 | TxType::Eip4844 => OpTxType::Eip1559,
            TxType::Eip2930 => OpTxType::Eip2930,
            TxType::Eip7702 => OpTxType::Eip7702,
            TxType::Legacy => OpTxType::Legacy,
        }
    }

    #[doc(alias = "output_transaction_type_checked")]
    fn output_tx_type_checked(&self) -> Option<OpTxType> {
        self.as_ref().buildable_type().map(|tx_ty| match tx_ty {
            TxType::Eip1559 | TxType::Eip4844 => OpTxType::Eip1559,
            TxType::Eip2930 => OpTxType::Eip2930,
            TxType::Eip7702 => OpTxType::Eip7702,
            TxType::Legacy => OpTxType::Legacy,
        })
    }

    fn prep_for_submission(&mut self) {
        self.as_mut().prep_for_submission();
    }

    fn build_unsigned(self) -> BuildResult<OpTypedTransaction, Optimism> {
        if let Err((tx_type, missing)) = self.as_ref().missing_keys() {
            let tx_type = OpTxType::try_from(tx_type as u8).unwrap();
            return Err(TransactionBuilderError::InvalidTransactionRequest(tx_type, missing)
                .into_unbuilt(self));
        }
        Ok(self.build_typed_tx().expect("checked by missing_keys"))
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

use alloy_provider::fillers::{
    ChainIdFiller, GasFiller, JoinFill, NonceFiller, RecommendedFillers,
};

impl RecommendedFillers for Optimism {
    type RecommendedFillers = JoinFill<GasFiller, JoinFill<NonceFiller, ChainIdFiller>>;

    fn recommended_fillers() -> Self::RecommendedFillers {
        Default::default()
    }
}
