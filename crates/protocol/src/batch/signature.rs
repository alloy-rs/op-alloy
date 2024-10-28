//! This module contains the [SpanBatchSignature] type, which represents the ECDSA signature of a
//! transaction within a span batch.

use crate::{SpanBatchError, SpanDecodingError};
use alloy_primitives::{Signature, U256};

/// The ECDSA signature of a transaction within a span batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpanBatchSignature {
    pub(crate) v: u64,
    pub(crate) r: U256,
    pub(crate) s: U256,
}

impl From<Signature> for SpanBatchSignature {
    fn from(value: Signature) -> Self {
        Self { v: value.v().to_u64(), r: value.r(), s: value.s() }
    }
}

impl TryFrom<SpanBatchSignature> for Signature {
    type Error = SpanBatchError;

    fn try_from(value: SpanBatchSignature) -> Result<Self, Self::Error> {
        Self::from_rs_and_parity(value.r, value.s, value.v)
            .map_err(|_| SpanBatchError::Decoding(SpanDecodingError::InvalidTransactionSignature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::Signature;

    #[test]
    fn test_span_batch_signature_conversion() {
        let signature = Signature::from_rs_and_parity(U256::from(1), U256::from(2), 27).unwrap();
        let span_batch_signature = SpanBatchSignature::from(signature);
        let converted_signature = Signature::try_from(span_batch_signature).unwrap();
        assert_eq!(signature, converted_signature);
    }
}
