#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

// Re-export EIP2718 Types and Traits
pub use alloy_eips::eip2718::{Decodable2718, Eip2718Error, Eip2718Result, Encodable2718};

// Re-export Alloy RLP Traits
pub use alloy_rlp::{Decodable, Encodable};

// Re-export Alloy Consensus Types
pub use alloy_consensus::{Eip658Value, Receipt, ReceiptWithBloom, Transaction, TxReceipt};

mod receipt;
pub use receipt::{OpDepositReceipt, OpDepositReceiptWithBloom, OpReceiptEnvelope, OpTxReceipt};

mod transaction;
pub use transaction::{OpTxEnvelope, OpTxType, OpTypedTransaction, TxDeposit, DEPOSIT_TX_TYPE_ID};
