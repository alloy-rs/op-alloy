//! OP transaction identifiers.

use alloy_eips::eip2718::Eip2718Error;
use core::mem;

/// Identifier for legacy transaction, however a legacy tx is technically not
/// typed.
pub const LEGACY_TX_TYPE_ID: u8 = 0;

/// Identifier for an EIP2930 transaction.
pub const EIP2930_TX_TYPE_ID: u8 = 1;

/// Identifier for an EIP1559 transaction.
pub const EIP1559_TX_TYPE_ID: u8 = 2;

/// Identifier for an EIP4844 transaction.
pub const EIP4844_TX_TYPE_ID: u8 = 3;

/// Identifier for an Optimism deposit transaction
pub const DEPOSIT_TX_TYPE_ID: u8 = 126;

/// Optimism `TransactionType` as specified in EIPs [2718], [1559], and
/// [2930], as well as the [deposit transaction spec][deposit-spec]
///
/// [2718]: https://eips.ethereum.org/EIPS/eip-2718
/// [1559]: https://eips.ethereum.org/EIPS/eip-1559
/// [2930]: https://eips.ethereum.org/EIPS/eip-2930
/// [4844]: https://eips.ethereum.org/EIPS/eip-4844
/// [deposit-spec]: https://specs.optimism.io/protocol/deposits.html
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TxType {
    /// Legacy transaction pre EIP-2929
    #[default]
    Legacy = 0_isize,
    /// AccessList transaction
    Eip2930 = 1_isize,
    /// Transaction with Priority fee
    Eip1559 = 2_isize,
    /// Shard Blob Transactions - EIP-4844
    Eip4844 = 3_isize,
    /// Optimism Deposit transaction.
    Deposit = 126_isize,
}

#[cfg(any(test, feature = "arbitrary"))]
impl<'a> arbitrary::Arbitrary<'a> for TxType {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        Ok(match u.int_in_range(0..=3)? {
            0 => TxType::Legacy,
            1 => TxType::Eip2930,
            2 => TxType::Eip1559,
            3 => TxType::Eip4844,
            0x7E => TxType::Deposit,
            _ => unreachable!(),
        })
    }
}

impl From<TxType> for u8 {
    fn from(value: TxType) -> Self {
        match value {
            TxType::Legacy => LEGACY_TX_TYPE_ID,
            TxType::Eip2930 => EIP2930_TX_TYPE_ID,
            TxType::Eip1559 => EIP1559_TX_TYPE_ID,
            TxType::Eip4844 => EIP4844_TX_TYPE_ID,
            TxType::Deposit => DEPOSIT_TX_TYPE_ID,
        }
    }
}

impl TryFrom<u8> for TxType {
    type Error = Eip2718Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            // SAFETY: repr(u8) with explicit discriminant
            0..=3 | 0x7E => Ok(unsafe { mem::transmute::<u8, TxType>(value) }),
            _ => Err(Eip2718Error::UnexpectedType(value)),
        }
    }
}
