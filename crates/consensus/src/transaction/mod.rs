//! Tramsaction types for Optimism.

mod deposit;
pub use deposit::{DepositTransaction, TxDeposit};

mod tx_type;
pub use tx_type::{DEPOSIT_TX_TYPE_ID, OpTxType};

mod envelope;
pub use envelope::OpTxEnvelope;

mod typed;
pub use typed::OpTypedTransaction;

mod pooled;
pub use pooled::OpPooledTransaction;

#[cfg(feature = "serde")]
pub use deposit::serde_deposit_tx_rpc;

/// Bincode-compatible serde implementations for transaction types.
#[cfg(all(feature = "serde", feature = "serde-bincode-compat"))]
pub(super) mod serde_bincode_compat {
    pub use super::deposit::serde_bincode_compat::TxDeposit;
}
