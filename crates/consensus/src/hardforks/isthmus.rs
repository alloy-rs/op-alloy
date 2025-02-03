//! Module containing a [Transaction] builder for the Isthmus network upgrade transactions.
//!
//! Isthmus network upgrade transactions are defined in the [OP Stack Specs][specs].
//!
//! [specs]: https://specs.optimism.io/protocol/isthmus/derivation.html#network-upgrade-automation-transactions
//! [Transaction]: alloy_consensus::Transaction

use crate::UpgradeDepositSource;
use alloc::{string::String, vec::Vec};
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{address, hex, Address, Bytes, TxKind, B256, U256};

use crate::{Hardfork, TxDeposit};

/// The Isthmus network upgrade transactions.
#[derive(Debug, Default, Clone, Copy)]
pub struct Isthmus;

impl Isthmus {
    /// EIP-2935 From Address
    pub const EIP2935_FROM: Address = address!("3462413Af4609098e1E27A490f554f260213D685");

    /// L1 Block Deployer Address
    pub const L1_BLOCK_DEPLOYER: Address = address!("4210000000000000000000000000000000000003");

    /// The Gas Price Oracle Deployer Address
    pub const GAS_PRICE_ORACLE_DEPLOYER: Address =
        address!("4210000000000000000000000000000000000004");

    /// The Operator Fee Vault Deployer Address
    pub const OPERATOR_FEE_VAULT_DEPLOYER: Address =
        address!("4210000000000000000000000000000000000005");


    /// Returns the source hash for the Isthmus Deposit Contract deployment.
    pub fn deposit_contract_source() -> B256 {
        UpgradeDepositSource { intent: String::from("Isthmus: deposit contract deployment") }
            .source_hash()
    }

    /// Returns the source hash for the deployment of the gas price oracle contract.
    pub fn deploy_gas_price_oracle_source() -> B256 {
        UpgradeDepositSource { intent: String::from("Isthmus: Gas Price Oracle Deployment") }
            .source_hash()
    }

    /// Returns the source hash for the deployment of the l1 block contract.
    pub fn deploy_l1_block_source() -> B256 {
        UpgradeDepositSource { intent: String::from("Isthmus: L1 Block Deployment") }.source_hash()
    }

    /// Returns the source hash for the deployment of the operator fee vault contract.
    pub fn deploy_operator_fee_vault_source() -> B256 {
        UpgradeDepositSource { intent: String::from("Isthmus: Operator Fee Vault Deployment") }.source_hash()
    }

    /// Returns the raw bytecode for the L1 Block deployment.
    pub fn l1_block_deployment_bytecode() -> Bytes {
        hex::decode(include_str!("./bytecode/l1_block_isthmus.hex").replace("\n", ""))
            .expect("Expected hex byte string")
            .into()
    }

    /// Returns the gas price oracle deployment bytecode.
    pub fn gas_price_oracle_deployment_bytecode() -> Bytes {
        hex::decode(include_str!("./bytecode/gpo_isthmus.hex").replace("\n", ""))
            .expect("Expected hex byte string")
            .into()
    }

    /// Returns the gas price oracle deployment bytecode.
    pub fn operator_fee_vault_deployment_bytecode() -> Bytes {
        hex::decode(include_str!("./bytecode/ofv_isthmus.hex").replace("\n", ""))
            .expect("Expected hex byte string")
            .into()
    }


    /// Returns the EIP-2935 creation data.
    pub fn eip2935_creation_data() -> Bytes {
        hex::decode(include_str!("./bytecode/eip2935_isthmus.hex").replace("\n", ""))
            .expect("Expected hex byte string")
            .into()
    }

    /// Returns the list of [TxDeposit]s for the network upgrade.
    pub fn deposits() -> impl Iterator<Item = TxDeposit> {
        ([
            TxDeposit {
                source_hash: Self::deploy_l1_block_source(),
                from: Self::L1_BLOCK_DEPLOYER,
                to: TxKind::Create,
                mint: 0.into(),
                value: U256::ZERO,
                gas_limit: 425_000,
                is_system_transaction: false,
                input: Self::l1_block_deployment_bytecode(),
            },
            TxDeposit {
                source_hash: Self::deploy_gas_price_oracle_source(),
                from: Self::GAS_PRICE_ORACLE_DEPLOYER,
                to: TxKind::Create,
                mint: 0.into(),
                value: U256::ZERO,
                gas_limit: 1_625_000,
                is_system_transaction: false,
                input: Self::gas_price_oracle_deployment_bytecode(),
            },
            TxDeposit {
                source_hash: Self::deploy_operator_fee_vault_source(),
                from: Self::OPERATOR_FEE_VAULT_DEPLOYER,
                to: TxKind::Create,
                mint: 0.into(),
                value: U256::ZERO,
                gas_limit: 500_000,
                is_system_transaction: false,
                input: Self::operator_fee_vault_deployment_bytecode(),
            },
            TxDeposit {
                source_hash: Self::deposit_contract_source(),
                from: Self::EIP2935_FROM,
                to: TxKind::Create,
                mint: 0.into(),
                value: U256::ZERO,
                gas_limit: 250_000,
                is_system_transaction: false,
                input: Self::eip2935_creation_data(),
            }
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
mod tests {
    use super::*;
    use alloc::vec;
    use alloy_primitives::b256;

    #[test]
    fn test_l1_block_source_hash() {
        let expected = b256!("3b2d0821ca2411ad5cd3595804d1213d15737188ae4cbd58aa19c821a6c211bf");
        assert_eq!(Isthmus::deploy_l1_block_source(), expected);
    }

    #[test]
    fn test_gas_price_oracle_source_hash() {
        let expected = b256!("fc70b48424763fa3fab9844253b4f8d508f91eb1f7cb11a247c9baec0afb8035");
        assert_eq!(Isthmus::deploy_gas_price_oracle_source(), expected);
    }

    #[test]
    fn test_isthmus_txs_encoded() {
        let isthmus_upgrade_tx = Isthmus.txs().collect::<Vec<_>>();
        assert_eq!(isthmus_upgrade_tx.len(), 1);

        let expected_txs: Vec<Bytes> =
            vec![hex::decode(include_str!("./bytecode/isthmus_tx_1.hex").replace("\n", ""))
                .unwrap()
                .into()];
        for (i, expected) in expected_txs.iter().enumerate() {
            assert_eq!(isthmus_upgrade_tx[i], *expected);
        }
    }
}
