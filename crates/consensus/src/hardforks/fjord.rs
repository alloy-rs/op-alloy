//! Module containing a [Transaction] builder for the Fjord network upgrade transactions.
//!
//! [Transaction]: alloy_consensus::Transaction

use crate::{OpTxEnvelope, TxDeposit, UpgradeDepositSource};
use alloc::{string::String, vec::Vec};
use alloy_consensus::Sealable;
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{address, hex, Address, Bytes, TxKind, B256, U256};

/// The Fjord network upgrade transactions.
#[derive(Debug, Default, Clone, Copy)]
pub struct Fjord;

impl Fjord {
    /// The Gas Price Oracle Address
    /// This is computed by using go-ethereum's `crypto.CreateAddress` function,
    /// with the Gas Price Oracle Deployer Address and nonce 0.
    pub const GAS_PRICE_ORACLE: Address = address!("b528d11cc114e026f138fe568744c6d45ce6da7a");

    /// The L1 Info Depositer Address.
    pub const L1_INFO_DEPOSITER: Address = address!("deaddeaddeaddeaddeaddeaddeaddeaddead0001");

    /// Fjord Gas Price Oracle Deployer Address.
    pub const GAS_PRICE_ORACLE_FJORD_DEPLOYER: Address =
        address!("4210000000000000000000000000000000000002");

    /// Fjord Gas Price Oracle address.
    pub const FJORD_GAS_PRICE_ORACLE: Address =
        address!("a919894851548179a0750865e7974da599c0fac7");

    /// The Set Fjord Four Byte Method Signature.
    pub const SET_FJORD_METHOD_SIGNATURE: [u8; 4] = hex!("8e98b106");

    /// Returns the source hash for the deployment of the Fjord Gas Price Oracle.
    pub fn deploy_fjord_gas_price_oracle_source() -> B256 {
        UpgradeDepositSource { intent: String::from("Fjord: Gas Price Oracle Deployment") }
            .source_hash()
    }

    /// Returns the source hash for the update of the Fjord Gas Price Oracle.
    pub fn update_fjord_gas_price_oracle_source() -> B256 {
        UpgradeDepositSource { intent: String::from("Fjord: Gas Price Oracle Proxy Update") }
            .source_hash()
    }

    /// [UpgradeDepositSource] for setting the Fjord Gas Price Oracle.
    pub fn enable_fjord_source() -> B256 {
        UpgradeDepositSource { intent: String::from("Fjord: Gas Price Oracle Set Fjord") }
            .source_hash()
    }

    /// Returns the fjord gas price oracle deployment bytecode.
    pub fn gas_price_oracle_deployment_bytecode() -> alloy_primitives::Bytes {
        include_bytes!("./bytecode/gpo_fjord.hex").into()
    }
}

impl super::Hardfork for Fjord {
    /// Constructs the Fjord network upgrade transactions.
    fn txs(&self) -> impl Iterator<Item = Bytes> + '_ {
        let mut tx_1 = Vec::new();
        OpTxEnvelope::Deposit(
            TxDeposit {
                source_hash: Self::deploy_fjord_gas_price_oracle_source(),
                from: Self::GAS_PRICE_ORACLE_FJORD_DEPLOYER,
                to: TxKind::Create,
                mint: 0.into(),
                value: U256::ZERO,
                gas_limit: 1_450_000,
                is_system_transaction: false,
                input: Self::gas_price_oracle_deployment_bytecode(),
            }
            .seal_slow(),
        )
        .encode_2718(&mut tx_1);

        // Update the gas price oracle proxy.
        let mut tx_2 = Vec::new();
        OpTxEnvelope::Deposit(
            TxDeposit {
                source_hash: Self::update_fjord_gas_price_oracle_source(),
                from: Address::ZERO,
                to: TxKind::Call(Self::GAS_PRICE_ORACLE),
                mint: 0.into(),
                value: U256::ZERO,
                gas_limit: 50_000,
                is_system_transaction: false,
                input: super::upgrade_to_calldata(Self::FJORD_GAS_PRICE_ORACLE),
            }
            .seal_slow(),
        )
        .encode_2718(&mut tx_2);

        // Enable Fjord
        let mut tx_3 = Vec::new();
        OpTxEnvelope::Deposit(
            TxDeposit {
                source_hash: Self::enable_fjord_source(),
                from: Self::L1_INFO_DEPOSITER,
                to: TxKind::Call(Self::GAS_PRICE_ORACLE),
                mint: 0.into(),
                value: U256::ZERO,
                gas_limit: 90_000,
                is_system_transaction: false,
                input: Self::SET_FJORD_METHOD_SIGNATURE.into(),
            }
            .seal_slow(),
        )
        .encode_2718(&mut tx_3);

        [tx_1, tx_2, tx_3].into_iter().map(Bytes::from)
    }
}
