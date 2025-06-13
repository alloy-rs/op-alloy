//! Contains the transaction type identifier for Optimism.

use core::fmt::Display;

use crate::transaction::envelope::OpTxType;
use alloy_rlp::{BufMut, Decodable, Encodable};

/// Identifier for an Optimism deposit transaction
pub const DEPOSIT_TX_TYPE_ID: u8 = 126; // 0x7E

impl Display for OpTxType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Legacy => write!(f, "legacy"),
            Self::Eip2930 => write!(f, "eip2930"),
            Self::Eip1559 => write!(f, "eip1559"),
            Self::Eip7702 => write!(f, "eip7702"),
            Self::Deposit => write!(f, "deposit"),
        }
    }
}

impl OpTxType {
    /// List of all variants.
    pub const ALL: [Self; 5] =
        [Self::Legacy, Self::Eip2930, Self::Eip1559, Self::Eip7702, Self::Deposit];

    /// Returns `true` if the type is [`OpTxType::Deposit`].
    pub const fn is_deposit(&self) -> bool {
        matches!(self, Self::Deposit)
    }
}

#[cfg(feature = "arbitrary")]
impl arbitrary::Arbitrary<'_> for OpTxType {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let i = u.choose_index(Self::ALL.len())?;
        Ok(Self::ALL[i])
    }
}

impl PartialEq<u8> for OpTxType {
    fn eq(&self, other: &u8) -> bool {
        (*self as u8) == *other
    }
}

impl PartialEq<OpTxType> for u8 {
    fn eq(&self, other: &OpTxType) -> bool {
        *self == *other as Self
    }
}

impl Encodable for OpTxType {
    fn encode(&self, out: &mut dyn BufMut) {
        (*self as u8).encode(out);
    }

    fn length(&self) -> usize {
        1
    }
}

impl Decodable for OpTxType {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let ty = u8::decode(buf)?;

        Self::try_from(ty).map_err(|_| alloy_rlp::Error::Custom("invalid transaction type"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{vec, vec::Vec};

    #[test]
    fn test_all_tx_types() {
        assert_eq!(OpTxType::ALL.len(), 5);
        let all = vec![
            OpTxType::Legacy,
            OpTxType::Eip2930,
            OpTxType::Eip1559,
            OpTxType::Eip7702,
            OpTxType::Deposit,
        ];
        assert_eq!(OpTxType::ALL.to_vec(), all);
    }

    #[test]
    fn tx_type_roundtrip() {
        for &tx_type in &OpTxType::ALL {
            let mut buf = Vec::new();
            tx_type.encode(&mut buf);
            let decoded = OpTxType::decode(&mut &buf[..]).unwrap();
            assert_eq!(tx_type, decoded);
        }
    }
}
