//! Receipt types for Optimism.

use alloy_consensus::TxReceipt;

mod envelope;
pub use envelope::OpReceiptEnvelope;

mod receipts;
pub use receipts::{OpDepositReceipt, OpDepositReceiptWithBloom};

/// Receipt is the result of a transaction execution.
pub trait OpTxReceipt: TxReceipt {
    /// Returns the deposit nonce of the transaction.
    fn deposit_nonce(&self) -> Option<u64>;

    /// Returns the deposit receipt version of the transaction.
    fn deposit_receipt_version(&self) -> Option<u64>;
}
