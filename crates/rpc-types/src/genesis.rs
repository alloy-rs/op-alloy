//! OP types for genesis data.

use alloy_serde::OtherFields;
use serde::de::Error;

/// Info for Optimism chain.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimismChainInfo {
    /// Genesis info
    pub genesis_info: Option<OptimismGenesisInfo>,
    /// Base fee info
    pub base_fee_info: Option<OptimismBaseFeeInfo>,
}

impl OptimismChainInfo {
    /// Extract the Optimism chain info from `OtherFields` object.
    pub fn extract_from(others: &OtherFields) -> Option<Self> {
        let genesis_info = OptimismGenesisInfo::extract_from(others);
        let base_fee_info = OptimismBaseFeeInfo::extract_from(others);

        Some(OptimismChainInfo { genesis_info, base_fee_info })
    }
}

impl TryFrom<OtherFields> for OptimismChainInfo {
    type Error = serde_json::Error;

    fn try_from(others: OtherFields) -> Result<Self, Self::Error> {
        let genesis_info = OptimismGenesisInfo::try_from(others.clone())?;
        let base_fee_info = OptimismBaseFeeInfo::try_from(others)?;

        Ok(OptimismChainInfo {
            genesis_info: Some(genesis_info),
            base_fee_info: Some(base_fee_info),
        })
    }
}

/// Genesis info for Optimism.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimismGenesisInfo {
    /// Bedrock block number
    pub bedrock_block: Option<u64>,
    /// regolith hardfork timestamp
    pub regolith_time: Option<u64>,
    /// canyon hardfork timestamp
    pub canyon_time: Option<u64>,
    /// ecotone hardfork timestamp
    pub ecotone_time: Option<u64>,
    /// fjord hardfork timestamp
    pub fjord_time: Option<u64>,
}

impl OptimismGenesisInfo {
    /// Extract the Optimism genesis info from `OtherFields` object.
    pub fn extract_from(others: &OtherFields) -> Option<Self> {
        others.deserialize_as().ok()
    }
}

impl TryFrom<OtherFields> for OptimismGenesisInfo {
    type Error = serde_json::Error;

    fn try_from(others: OtherFields) -> Result<Self, Self::Error> {
        others.deserialize_as()
    }
}

/// Additional base fee info for Optimism.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimismBaseFeeInfo {
    /// EIP-1559 elasticity
    pub eip1559_elasticity: Option<u64>,
    /// EIP-1559 denominator
    pub eip1559_denominator: Option<u64>,
    /// EIP-1559 denominator after canyon
    pub eip1559_denominator_canyon: Option<u64>,
}

impl OptimismBaseFeeInfo {
    /// Extract the Optimism base fee info from `OtherFields` object.
    pub fn extract_from(others: &OtherFields) -> Option<Self> {
        if let Some(Ok(optimism_base_fee_info)) =
            others.get_deserialized::<OptimismBaseFeeInfo>("optimism")
        {
            return Some(optimism_base_fee_info);
        }
        None
    }
}

impl TryFrom<OtherFields> for OptimismBaseFeeInfo {
    type Error = serde_json::Error;

    fn try_from(others: OtherFields) -> Result<Self, Self::Error> {
        if let Some(Ok(optimism_base_fee_info)) =
            others.get_deserialized::<OptimismBaseFeeInfo>("optimism")
        {
            Ok(optimism_base_fee_info)
        } else {
            Err(serde_json::Error::missing_field("optimism"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_optimism_genesis_info() {
        let genesis_info = r#"
        {
          "bedrockBlock": 10,
          "regolithTime": 12,
          "canyonTime": 0,
          "ecotoneTime": 0
        }
        "#;

        let others: OtherFields = serde_json::from_str(genesis_info).unwrap();
        let genesis_info = OptimismGenesisInfo::extract_from(&others).unwrap();

        assert_eq!(
            genesis_info,
            OptimismGenesisInfo {
                bedrock_block: Some(10),
                regolith_time: Some(12),
                canyon_time: Some(0),
                ecotone_time: Some(0),
                fjord_time: None,
            }
        );
    }

    #[test]
    fn test_extract_optimism_base_fee_info() {
        let base_fee_info = r#"
        {
          "optimism": {
            "eip1559Elasticity": 0,
            "eip1559Denominator": 8,
            "eip1559DenominatorCanyon": 8
          }
        }
        "#;

        let others: OtherFields = serde_json::from_str(base_fee_info).unwrap();
        let base_fee_info = OptimismBaseFeeInfo::extract_from(&others).unwrap();

        assert_eq!(
            base_fee_info,
            OptimismBaseFeeInfo {
                eip1559_elasticity: Some(0),
                eip1559_denominator: Some(8),
                eip1559_denominator_canyon: Some(8),
            }
        );
    }

    #[test]
    fn test_extract_optimism_chain_info() {
        let chain_info = r#"
        {
          "bedrockBlock": 10,
          "regolithTime": 12,
          "canyonTime": 0,
          "ecotoneTime": 0,
          "optimism": {
            "eip1559Denominator": 8,
            "eip1559DenominatorCanyon": 8
          }
        }
        "#;

        let others: OtherFields = serde_json::from_str(chain_info).unwrap();
        let chain_info = OptimismChainInfo::extract_from(&others).unwrap();

        assert_eq!(
            chain_info,
            OptimismChainInfo {
                genesis_info: Some(OptimismGenesisInfo {
                    bedrock_block: Some(10),
                    regolith_time: Some(12),
                    canyon_time: Some(0),
                    ecotone_time: Some(0),
                    fjord_time: None,
                }),
                base_fee_info: Some(OptimismBaseFeeInfo {
                    eip1559_elasticity: None,
                    eip1559_denominator: Some(8),
                    eip1559_denominator_canyon: Some(8),
                }),
            }
        );

        let chain_info = OptimismChainInfo::try_from(others).unwrap();

        assert_eq!(
            chain_info,
            OptimismChainInfo {
                genesis_info: Some(OptimismGenesisInfo {
                    bedrock_block: Some(10),
                    regolith_time: Some(12),
                    canyon_time: Some(0),
                    ecotone_time: Some(0),
                    fjord_time: None,
                }),
                base_fee_info: Some(OptimismBaseFeeInfo {
                    eip1559_elasticity: None,
                    eip1559_denominator: Some(8),
                    eip1559_denominator_canyon: Some(8),
                }),
            }
        );
    }

    #[test]
    fn test_extract_optimism_chain_info_no_base_fee() {
        let chain_info = r#"
        {
          "bedrockBlock": 10,
          "regolithTime": 12,
          "canyonTime": 0,
          "ecotoneTime": 0,
          "fjordTime": 0
        }
        "#;

        let others: OtherFields = serde_json::from_str(chain_info).unwrap();
        let chain_info = OptimismChainInfo::extract_from(&others).unwrap();

        assert_eq!(
            chain_info,
            OptimismChainInfo {
                genesis_info: Some(OptimismGenesisInfo {
                    bedrock_block: Some(10),
                    regolith_time: Some(12),
                    canyon_time: Some(0),
                    ecotone_time: Some(0),
                    fjord_time: Some(0),
                }),
                base_fee_info: None,
            }
        );
    }
}
