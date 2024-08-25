//! RPC errors specific to OP.

use alloy_rpc_types_eth::error::EthRpcErrorCode;
use revm_primitives::{InvalidTransaction, OptimismInvalidTransaction};
use std::fmt::Display;

/// Optimism specific errors.
#[derive(Debug, thiserror::Error)]
pub enum OpEthApiError<EthApiError: Display> {
    /// L1 ethereum error.
    #[error(transparent)]
    Eth(#[from] EthApiError),
    /// Thrown when calculating L1 gas fee.
    #[error("failed to calculate l1 gas fee")]
    L1BlockFeeError,
    /// Thrown when calculating L1 gas used
    #[error("failed to calculate l1 gas used")]
    L1BlockGasError,
    /// Wrapper for [`revm_primitives::InvalidTransaction`].
    #[error(transparent)]
    InvalidTransaction(OptimismInvalidTransactionError),
}

/// Constructs an internal JSON-RPC error.
fn internal_rpc_err(msg: impl Into<String>) -> jsonrpsee_types::error::ErrorObject<'static> {
    rpc_err(jsonrpsee_types::error::INTERNAL_ERROR_CODE, msg, None)
}

/// Constructs a JSON-RPC error, consisting of `code`, `message` and optional `data`.
fn rpc_err(
    code: i32,
    msg: impl Into<String>,
    data: Option<&[u8]>,
) -> jsonrpsee_types::error::ErrorObject<'static> {
    jsonrpsee_types::error::ErrorObject::owned(
        code,
        msg.into(),
        data.map(|data| {
            jsonrpsee_core::to_json_raw_value(&alloy_primitives::hex::encode_prefixed(data))
                .expect("serializing String can't fail")
        }),
    )
}

impl<EthApiError> From<OpEthApiError<EthApiError>> for jsonrpsee_types::error::ErrorObject<'static>
where
    EthApiError: Into<jsonrpsee_types::error::ErrorObject<'static>> + Display,
{
    fn from(err: OpEthApiError<EthApiError>) -> Self {
        match err {
            OpEthApiError::Eth(err) => err.into(),
            OpEthApiError::L1BlockFeeError | OpEthApiError::L1BlockGasError => {
                internal_rpc_err(err.to_string())
            }
            OpEthApiError::InvalidTransaction(err) => err.into(),
        }
    }
}

/// Optimism specific invalid transaction errors
#[derive(thiserror::Error, Copy, Clone, Debug)]
pub enum OptimismInvalidTransactionError {
    /// A deposit transaction was submitted as a system transaction post-regolith.
    #[error("no system transactions allowed after regolith")]
    DepositSystemTxPostRegolith,
    /// A deposit transaction halted post-regolith
    #[error("deposit transaction halted after regolith")]
    HaltedDepositPostRegolith,
}

impl From<OptimismInvalidTransactionError> for jsonrpsee_types::error::ErrorObject<'static> {
    fn from(err: OptimismInvalidTransactionError) -> Self {
        match err {
            OptimismInvalidTransactionError::DepositSystemTxPostRegolith
            | OptimismInvalidTransactionError::HaltedDepositPostRegolith => {
                rpc_err(EthRpcErrorCode::TransactionRejected.code(), err.to_string(), None)
            }
        }
    }
}

impl TryFrom<InvalidTransaction> for OptimismInvalidTransactionError {
    type Error = InvalidTransaction;

    fn try_from(err: InvalidTransaction) -> Result<Self, Self::Error> {
        match err {
            InvalidTransaction::OptimismError(err) => match err {
                OptimismInvalidTransaction::DepositSystemTxPostRegolith => {
                    Ok(Self::DepositSystemTxPostRegolith)
                }
                OptimismInvalidTransaction::HaltedDepositPostRegolith => {
                    Ok(Self::HaltedDepositPostRegolith)
                }
            },
            _ => Err(err),
        }
    }
}
