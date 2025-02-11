//! Additional compatibility implementations.

use alloy_eips::Typed2718;
use alloy_network::{UnknownTxEnvelope, UnknownTypedTransaction};
use alloy_rpc_types_eth::ConversionError;
use crate::{TxDeposit, DEPOSIT_TX_TYPE_ID};

impl TryFrom<UnknownTxEnvelope> for TxDeposit {
    type Error = ConversionError;

    fn try_from(value: UnknownTxEnvelope) -> Result<Self, Self::Error> {
       value.inner.try_into()
    }
}

impl TryFrom<UnknownTypedTransaction> for TxDeposit {
    type Error = ConversionError;

    fn try_from(value: UnknownTypedTransaction) -> Result<Self, Self::Error> {
        if !value.is_type(DEPOSIT_TX_TYPE_ID) {
            return Err(ConversionError::Custom("invalid transaction type".to_string()))
        }
        TxDeposit {
            source_hash: value.fields.,
            from: Default::default(),
            to: Default::default(),
            mint: None,
            value: Default::default(),
            gas_limit: 0,
            is_system_transaction: false,
            input: Default::default(),
        };
        
        todo!()
    }
}