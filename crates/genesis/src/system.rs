//! System Config Type

use crate::RollupConfig;
use alloy_consensus::{Eip658Value, Receipt};
use alloy_primitives::{address, b256, Address, Log, B256, U256, U64};
use alloy_sol_types::{sol, SolType};

/// `keccak256("ConfigUpdate(uint256,uint8,bytes)")`
pub const CONFIG_UPDATE_TOPIC: B256 =
    b256!("1d2b0bda21d56b8bd12d4f94ebacffdfb35f5e226f84b461103bb8beab6353be");

/// The initial version of the system config event log.
pub const CONFIG_UPDATE_EVENT_VERSION_0: B256 = B256::ZERO;

/// System configuration.
#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SystemConfig {
    /// Batcher address
    #[cfg_attr(feature = "serde", serde(rename = "batcherAddr"))]
    pub batcher_address: Address,
    /// Fee overhead value
    pub overhead: U256,
    /// Fee scalar value
    pub scalar: U256,
    /// Gas limit value
    pub gas_limit: u64,
    /// Base fee scalar value
    pub base_fee_scalar: Option<u64>,
    /// Blob base fee scalar value
    pub blob_base_fee_scalar: Option<u64>,
}

/// Represents type of update to the system config.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u64)]
pub enum SystemConfigUpdateType {
    /// Batcher update type
    Batcher = 0,
    /// Gas config update type
    GasConfig = 1,
    /// Gas limit update type
    GasLimit = 2,
    /// Unsafe block signer update type
    UnsafeBlockSigner = 3,
}

impl TryFrom<u64> for SystemConfigUpdateType {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SystemConfigUpdateType::Batcher),
            1 => Ok(SystemConfigUpdateType::GasConfig),
            2 => Ok(SystemConfigUpdateType::GasLimit),
            3 => Ok(SystemConfigUpdateType::UnsafeBlockSigner),
            _ => Err("Invalid SystemConfigUpdateType value"),
        }
    }
}

impl SystemConfig {
    /// Filters all L1 receipts to find config updates and applies the config updates.
    pub fn update_with_receipts(
        &mut self,
        receipts: &[Receipt],
        rollup_config: &RollupConfig,
        l1_time: u64,
    ) -> Result<(), &'static str> {
        for receipt in receipts {
            if Eip658Value::Eip658(false) == receipt.status {
                continue;
            }

            receipt.logs.iter().try_for_each(|log| {
                let topics = log.topics();
                if log.address == rollup_config.l1_system_config_address
                    && !topics.is_empty()
                    && topics[0] == CONFIG_UPDATE_TOPIC
                {
                    if let Err(_) = self.process_config_update_log(log, rollup_config, l1_time) {
                        return Err("Failed to process config update log");
                    }
                }
                Ok::<_, &'static str>(())
            })?;
        }
        Ok(())
    }

    /// Decodes an EVM log entry emitted by the system config contract and applies it as a
    /// [SystemConfig] change.
    ///
    /// Parse log data for:
    ///
    /// ```text
    /// event ConfigUpdate(
    ///    uint256 indexed version,
    ///    UpdateType indexed updateType,
    ///    bytes data
    /// );
    /// ```
    fn process_config_update_log(
        &mut self,
        log: &Log,
        rollup_config: &RollupConfig,
        l1_time: u64,
    ) -> Result<(), &'static str> {
        if log.topics().len() < 3 {
            return Err("Invalid config update log: not enough topics");
        }
        if log.topics()[0] != CONFIG_UPDATE_TOPIC {
            return Err("Invalid config update log: invalid topic");
        }

        // Parse the config update log
        let version = log.topics()[1];
        if version != CONFIG_UPDATE_EVENT_VERSION_0 {
            return Err("Invalid config update log: unsupported version");
        }
        let Ok(topic_bytes) = log.topics()[2].as_slice()[0..8].try_into() else {
            return Err("Invalid config update log: invalid update type");
        };
        let update_type = u64::from_be_bytes(topic_bytes);
        let log_data = log.data.data.as_ref();

        match update_type.try_into()? {
            SystemConfigUpdateType::Batcher => {
                if log_data.len() != 96 {
                    return Err("Invalid config update log: invalid data length");
                }

                let Ok(pointer) = <sol!(uint64)>::abi_decode(&log_data[0..32], true) else {
                    return Err("Failed to decode batcher update log");
                };
                if pointer != 32 {
                    return Err("Invalid config update log: invalid data pointer");
                }
                let Ok(length) = <sol!(uint64)>::abi_decode(&log_data[32..64], true) else {
                    return Err("Failed to decode batcher update log");
                };
                if length != 32 {
                    return Err("Invalid config update log: invalid data length");
                }

                let Ok(batcher_address) =
                    <sol!(address)>::abi_decode(&log.data.data.as_ref()[64..], true)
                else {
                    return Err("Failed to decode batcher update log");
                };
                self.batcher_address = batcher_address;
            }
            SystemConfigUpdateType::GasConfig => {
                if log_data.len() != 128 {
                    return Err("Invalid config update log: invalid data length");
                }

                let Ok(pointer) = <sol!(uint64)>::abi_decode(&log_data[0..32], true) else {
                    return Err("Invalid config update log: invalid data pointer");
                };

                if pointer != 32 {
                    return Err("Invalid config update log: invalid data pointer");
                }
                let Ok(length) = <sol!(uint64)>::abi_decode(&log_data[32..64], true) else {
                    return Err("Invalid config update log: invalid data length");
                };
                if length != 64 {
                    return Err("Invalid config update log: invalid data length");
                }

                let Ok(overhead) = <sol!(uint256)>::abi_decode(&log_data[64..96], true) else {
                    return Err("Invalid config update log: invalid overhead");
                };
                let Ok(scalar) = <sol!(uint256)>::abi_decode(&log_data[96..], true) else {
                    return Err("Invalid config update log: invalid scalar");
                };

                if rollup_config.is_ecotone_active(l1_time) {
                    if RollupConfig::check_ecotone_l1_system_config_scalar(scalar.to_be_bytes())
                        .is_err()
                    {
                        // ignore invalid scalars, retain the old system-config scalar
                        return Ok(());
                    }

                    // retain the scalar data in encoded form
                    self.scalar = scalar;
                    // zero out the overhead, it will not affect the state-transition after Ecotone
                    self.overhead = U256::ZERO;
                } else {
                    self.scalar = scalar;
                    self.overhead = overhead;
                }
            }
            SystemConfigUpdateType::GasLimit => {
                if log_data.len() != 96 {
                    return Err("Invalid config update log: invalid data length");
                }

                let Ok(pointer) = <sol!(uint64)>::abi_decode(&log_data[0..32], true) else {
                    return Err("Invalid config update log: invalid data pointer");
                };
                if pointer != 32 {
                    return Err("Invalid config update log: invalid data pointer");
                }
                let Ok(length) = <sol!(uint64)>::abi_decode(&log_data[32..64], true) else {
                    return Err("Invalid config update log: invalid data length");
                };
                if length != 32 {
                    return Err("Invalid config update log: invalid data length");
                }

                let Ok(gas_limit) = <sol!(uint256)>::abi_decode(&log_data[64..], true) else {
                    return Err("Invalid config update log: invalid gas limit");
                };
                self.gas_limit = U64::from(gas_limit).saturating_to::<u64>();
            }
            SystemConfigUpdateType::UnsafeBlockSigner => {
                // Ignored in derivation
            }
        }

        Ok(())
    }
}

/// System accounts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SystemAccounts {
    /// The address that can deposit attributes
    pub attributes_depositor: Address,
    /// The address of the attributes predeploy
    pub attributes_predeploy: Address,
    /// The address of the fee vault
    pub fee_vault: Address,
}

impl Default for SystemAccounts {
    fn default() -> Self {
        Self {
            attributes_depositor: address!("deaddeaddeaddeaddeaddeaddeaddeaddead0001"),
            attributes_predeploy: address!("4200000000000000000000000000000000000015"),
            fee_vault: address!("4200000000000000000000000000000000000011"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ChainGenesis;
    use alloc::vec;
    use alloy_primitives::{b256, hex, LogData, B256};

    fn mock_rollup_config(system_config: SystemConfig) -> RollupConfig {
        RollupConfig {
            genesis: ChainGenesis { system_config: Some(system_config), ..Default::default() },
            block_time: 2,
            l1_chain_id: 1,
            l2_chain_id: 10,
            regolith_time: Some(0),
            canyon_time: Some(0),
            delta_time: Some(0),
            ecotone_time: Some(10),
            fjord_time: Some(0),
            granite_time: Some(0),
            holocene_time: Some(0),
            blobs_enabled_l1_timestamp: Some(0),
            da_challenge_address: Some(Address::ZERO),
            ..Default::default()
        }
    }

    #[test]
    fn test_system_config_serde() {
        let sc_str = r#"{
          "batcherAddr": "0x6887246668a3b87F54DeB3b94Ba47a6f63F32985",
          "overhead": "0x00000000000000000000000000000000000000000000000000000000000000bc",
          "scalar": "0x00000000000000000000000000000000000000000000000000000000000a6fe0",
          "gasLimit": 30000000
        }"#;
        let system_config: SystemConfig = serde_json::from_str(sc_str).unwrap();
        assert_eq!(
            system_config.batcher_address,
            address!("6887246668a3b87F54DeB3b94Ba47a6f63F32985")
        );
        assert_eq!(system_config.overhead, U256::from(0xbc));
        assert_eq!(system_config.scalar, U256::from(0xa6fe0));
        assert_eq!(system_config.gas_limit, 30000000);
    }

    #[test]
    fn test_system_config_update_batcher_log() {
        const UPDATE_TYPE: B256 =
            b256!("0000000000000000000000000000000000000000000000000000000000000000");

        let mut system_config = SystemConfig::default();
        let rollup_config = mock_rollup_config(system_config.clone());

        let update_log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    UPDATE_TYPE,
                ],
                hex!("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000beef").into()
            )
        };

        // Update the batcher address.
        system_config.process_config_update_log(&update_log, &rollup_config, 0).unwrap();

        assert_eq!(
            system_config.batcher_address,
            address!("000000000000000000000000000000000000bEEF")
        );
    }

    #[test]
    fn test_system_config_update_gas_config_log() {
        const UPDATE_TYPE: B256 =
            b256!("0000000000000000000000000000000000000000000000000000000000000001");

        let mut system_config = SystemConfig::default();
        let rollup_config = mock_rollup_config(system_config.clone());

        let update_log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    UPDATE_TYPE,
                ],
                hex!("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000babe000000000000000000000000000000000000000000000000000000000000beef").into()
            )
        };

        // Update the batcher address.
        system_config.process_config_update_log(&update_log, &rollup_config, 0).unwrap();

        assert_eq!(system_config.overhead, U256::from(0xbabe));
        assert_eq!(system_config.scalar, U256::from(0xbeef));
    }

    #[test]
    fn test_system_config_update_gas_config_log_ecotone() {
        const UPDATE_TYPE: B256 =
            b256!("0000000000000000000000000000000000000000000000000000000000000001");

        let mut system_config = SystemConfig::default();
        let rollup_config = mock_rollup_config(system_config.clone());

        let update_log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    UPDATE_TYPE,
                ],
                hex!("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000babe000000000000000000000000000000000000000000000000000000000000beef").into()
            )
        };

        // Update the batcher address.
        system_config.process_config_update_log(&update_log, &rollup_config, 10).unwrap();

        assert_eq!(system_config.overhead, U256::from(0));
        assert_eq!(system_config.scalar, U256::from(0xbeef));
    }

    #[test]
    fn test_system_config_update_gas_limit_log() {
        const UPDATE_TYPE: B256 =
            b256!("0000000000000000000000000000000000000000000000000000000000000002");

        let mut system_config = SystemConfig::default();
        let rollup_config = mock_rollup_config(system_config.clone());

        let update_log = Log {
            address: Address::ZERO,
            data: LogData::new_unchecked(
                vec![
                    CONFIG_UPDATE_TOPIC,
                    CONFIG_UPDATE_EVENT_VERSION_0,
                    UPDATE_TYPE,
                ],
                hex!("00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000beef").into()
            )
        };

        // Update the batcher address.
        system_config.process_config_update_log(&update_log, &rollup_config, 0).unwrap();

        assert_eq!(system_config.gas_limit, 0xbeef_u64);
    }
}
