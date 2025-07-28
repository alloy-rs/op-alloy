//! Support for EIP-1559 parameters after holocene.

use alloy_eips::eip1559::BaseFeeParams;
use alloy_primitives::{B64, Bytes};

/// Encodes the `eip1559` parameters for the payload.
fn encode_eip_1559_params(
    eip_1559_params: B64,
    default_base_fee_params: BaseFeeParams,
    extra_data: &mut [u8],
) -> Result<(), EIP1559ParamError> {
    if eip_1559_params.is_zero() {
        let max_change_denominator: u32 = (default_base_fee_params.max_change_denominator)
            .try_into()
            .map_err(|_| EIP1559ParamError::DenominatorOverflow)?;

        let elasticity_multiplier: u32 = (default_base_fee_params.elasticity_multiplier)
            .try_into()
            .map_err(|_| EIP1559ParamError::ElasticityOverflow)?;

        extra_data[1..5].copy_from_slice(&max_change_denominator.to_be_bytes());
        extra_data[5..9].copy_from_slice(&elasticity_multiplier.to_be_bytes());
    } else {
        let (elasticity, denominator) = decode_eip_1559_params(eip_1559_params);
        extra_data[1..5].copy_from_slice(&denominator.to_be_bytes());
        extra_data[5..9].copy_from_slice(&elasticity.to_be_bytes());
    }
    Ok(())
}

/// Extracts the Holocene 1599 parameters from the encoded form:
/// <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/holocene/exec-engine.md#eip1559params-encoding>
///
/// Returns (`elasticity`, `denominator`)
pub fn decode_eip_1559_params(eip_1559_params: B64) -> (u32, u32) {
    let denominator: [u8; 4] = eip_1559_params.0[..4].try_into().expect("sufficient length");
    let elasticity: [u8; 4] = eip_1559_params.0[4..8].try_into().expect("sufficient length");

    (u32::from_be_bytes(elasticity), u32::from_be_bytes(denominator))
}

/// Decodes the `eip1559` parameters from the `extradata` bytes.
///
/// Returns (`elasticity`, `denominator`)
pub fn decode_holocene_extra_data(extra_data: &[u8]) -> Result<(u32, u32), EIP1559ParamError> {
    if extra_data.len() < 9 {
        return Err(EIP1559ParamError::NoEIP1559Params);
    }

    if extra_data[0] != 0 {
        // version must be 0: https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/holocene/exec-engine.md#eip-1559-parameters-in-block-header
        return Err(EIP1559ParamError::InvalidVersion(extra_data[0]));
    }
    // skip the first version byte
    Ok(decode_eip_1559_params(B64::from_slice(&extra_data[1..9])))
}

/// Encodes the `eip1559` parameters for the payload.
pub fn encode_holocene_extra_data(
    eip_1559_params: B64,
    default_base_fee_params: BaseFeeParams,
) -> Result<Bytes, EIP1559ParamError> {
    // 9 bytes: 1 byte for version (0) and 8 bytes for eip1559 params
    let mut extra_data = [0u8; 9];
    encode_eip_1559_params(eip_1559_params, default_base_fee_params, &mut extra_data)?;
    Ok(Bytes::copy_from_slice(&extra_data))
}

/// Decodes the EIP-1559 parameters from `extra_data`,
/// as well as the minimum base fee log2.
///
/// Returns (`elasticity`, `denominator`, `min_base_fee_log2`)
pub fn decode_min_base_fee_extra_data(
    extra_data: &[u8],
) -> Result<(u32, u32, u8), EIP1559ParamError> {
    if extra_data.len() < 10 {
        return Err(EIP1559ParamError::NoEIP1559Params);
    }

    if extra_data[0] != 1 {
        // version must be 1: <https://github.com/ethereum-optimism/design-docs/blob/main/protocol/minimum-base-fee.md#minimum-base-fee-in-block-header>
        return Err(EIP1559ParamError::InvalidVersion(extra_data[0]));
    }
    // skip the first version byte
    let denominator: [u8; 4] = extra_data[1..5].try_into().expect("sufficient length");
    let elasticity: [u8; 4] = extra_data[5..9].try_into().expect("sufficient length");
    let min_base_fee_log2: u8 = extra_data[9];

    Ok((u32::from_be_bytes(elasticity), u32::from_be_bytes(denominator), min_base_fee_log2))
}

/// Encodes the EIP-1559 parameters for the payload,
/// as well as the minimum base fee log2.
pub fn encode_min_base_fee_extra_data(
    eip_1559_params: B64,
    default_base_fee_params: BaseFeeParams,
    min_base_fee_log2: u8,
) -> Result<Bytes, EIP1559ParamError> {
    // 10 bytes: 1 byte for version (1), 8 bytes for eip1559 params, and 1 byte for the minimum base
    // fee log2
    let mut extra_data = [0u8; 10];
    extra_data[0] = 1;
    encode_eip_1559_params(eip_1559_params, default_base_fee_params, &mut extra_data)?;
    extra_data[9] = min_base_fee_log2;
    Ok(Bytes::copy_from_slice(&extra_data))
}

/// Error type for EIP-1559 parameters
#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
pub enum EIP1559ParamError {
    /// Thrown if the extra data begins with the wrong version byte.
    #[error("Invalid EIP1559 version byte: {0}")]
    InvalidVersion(u8),
    /// No EIP-1559 parameters provided.
    #[error("No EIP1559 parameters provided")]
    NoEIP1559Params,
    /// Denominator overflow.
    #[error("Denominator overflow")]
    DenominatorOverflow,
    /// Elasticity overflow.
    #[error("Elasticity overflow")]
    ElasticityOverflow,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn test_get_extra_data_post_holocene() {
        let eip_1559_params = B64::from_str("0x0000000800000008").unwrap();
        let extra_data = encode_holocene_extra_data(eip_1559_params, BaseFeeParams::new(80, 60));
        assert_eq!(extra_data.unwrap(), Bytes::copy_from_slice(&[0, 0, 0, 0, 8, 0, 0, 0, 8]));
    }

    #[test]
    fn test_get_extra_data_post_holocene_default() {
        let eip_1559_params = B64::ZERO;
        let extra_data = encode_holocene_extra_data(eip_1559_params, BaseFeeParams::new(80, 60));
        assert_eq!(extra_data.unwrap(), Bytes::copy_from_slice(&[0, 0, 0, 0, 80, 0, 0, 0, 60]));
    }

    #[test]
    fn test_get_extra_data_min_base_fee() {
        let eip_1559_params = B64::from_str("0x0000000800000008").unwrap();
        let extra_data =
            encode_min_base_fee_extra_data(eip_1559_params, BaseFeeParams::new(80, 60), 20);
        // check the version byte is 1 and the min_base_fee_log2 is 20
        assert_eq!(extra_data.unwrap(), Bytes::copy_from_slice(&[1, 0, 0, 0, 8, 0, 0, 0, 8, 20]));
    }

    #[test]
    fn test_get_extra_data_min_base_fee_default() {
        let eip_1559_params = B64::ZERO;
        let extra_data =
            encode_min_base_fee_extra_data(eip_1559_params, BaseFeeParams::new(80, 60), 0);
        // check the version byte is 1 and the min_base_fee_log2 is 0
        assert_eq!(extra_data.unwrap(), Bytes::copy_from_slice(&[1, 0, 0, 0, 80, 0, 0, 0, 60, 0]));
    }
}
