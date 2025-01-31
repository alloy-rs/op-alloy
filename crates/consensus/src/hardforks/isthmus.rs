//! Module containing a [Transaction] builder for the Isthmus network upgrade transactions.
//!
//! [Transaction]: alloy_consensus::Transaction

use crate::UpgradeDepositSource;
use alloc::{string::String, vec::Vec};
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{address, Address, Bytes, TxKind, B256, U256};

use crate::{Hardfork, TxDeposit};

/// The Isthmus network upgrade transactions.
#[derive(Debug, Default, Clone, Copy)]
pub struct Isthmus;

impl Isthmus {
    /// EIP-2935 From Address
    pub const EIP2935_FROM: Address = address!("E9f0662359Bb2c8111840eFFD73B9AFA77CbDE10");

    /// EIP-7002 From Address
    pub const EIP7002_FROM: Address = address!("a05D9eED37862daB22b002b8F6668B8Fb0c4D798");

    /// Returns the source hash for the Isthmus Deposit Contract deployment.
    pub fn deposit_contract_source() -> B256 {
        UpgradeDepositSource { intent: String::from("Isthmus: deposit contract deployment") }
            .source_hash()
    }

    /// Returns the source hash for the Isthmus Withdrawals Request Contract deployment.
    pub fn withdrawals_request_contract_source() -> B256 {
        UpgradeDepositSource {
            intent: String::from("Isthmus: withdrawals request contract deployment"),
        }
        .source_hash()
    }

    /// Returns the EIP-2935 creation data.
    pub fn eip2935_creation_data() -> Bytes {
        include_bytes!("./bytecode/eip2935_isthmus.hex").into()
    }

    /// Returns the EIP-7002 creation data.
    pub fn eip7002_creation_data() -> Bytes {
        include_bytes!("./bytecode/eip7002_isthmus.hex").into()
    }

    /// Returns the list of [TxDeposit]s for the network upgrade.
    pub fn deposits() -> impl Iterator<Item = TxDeposit> {
        ([
            TxDeposit {
                source_hash: Self::deposit_contract_source(),
                from: Self::EIP2935_FROM,
                to: TxKind::Create,
                mint: 0.into(),
                value: U256::ZERO,
                gas_limit: 250_000,
                is_system_transaction: false,
                input: Self::eip2935_creation_data(),
            },
            TxDeposit {
                source_hash: Self::withdrawals_request_contract_source(),
                from: Self::EIP7002_FROM,
                to: TxKind::Create,
                mint: 0.into(),
                value: U256::ZERO,
                gas_limit: 250_000,
                is_system_transaction: false,
                input: Self::eip7002_creation_data(),
            },
        ])
        .into_iter()
    }
}

impl Hardfork for Isthmus {
    /// Constructs the network upgrade transactions.
    fn txs(&self) -> impl Iterator<Item = Bytes> + '_ {
        Self::deposits().map(|tx| {
            let mut encoded = Vec::new();
            tx.encode_2718(&mut encoded);
            Bytes::from(encoded)
        })
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use super::*;
    use alloc::vec;
    use alloy_primitives::hex;

    #[test]
    fn test_isthmus_txs_encoded() {
        let isthmus_upgrade_tx = Isthmus.txs().collect::<Vec<_>>();
        assert_eq!(isthmus_upgrade_tx.len(), 2);

        // let expected_tx = include_bytes!("./bytecode/isthmus_tx_2.hex");
        // // Parse as a string.
        // let expected_tx = String::from_utf8_lossy(expected_tx);
        // // Remove newlines
        // let expected_tx = expected_tx.replace("\n", "");
        //
        // // Write the expected tx as a string to the file
        // std::fs::write("isthmus_tx_2.hex", expected_tx).unwrap();

        let expected_txs: Vec<Bytes> = vec![
            hex::decode(include_bytes!("./bytecode/isthmus_tx_1.hex")).unwrap().into(),
            hex::decode(include_bytes!("./bytecode/isthmus_tx_2.hex")).unwrap().into(),
        ];
        for (i, expected) in expected_txs.iter().enumerate() {
            assert_eq!(isthmus_upgrade_tx[i], *expected);
        }
    }
}
