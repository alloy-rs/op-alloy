//! Optimism-specific payload attributes.

use alloc::vec::Vec;
use alloy_consensus::transaction::Recovered;
use alloy_eips::{Decodable2718, eip1559::BaseFeeParams, eip2718::WithEncoded};
use alloy_primitives::{B64, Bytes, SignatureError};
use alloy_rlp::Result;
use alloy_rpc_types_engine::PayloadAttributes;
use op_alloy_consensus::{
    EIP1559ParamError, OpTxEnvelope, decode_eip_1559_params, encode_holocene_extra_data,
};

/// Optimism Payload Attributes
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct OpPayloadAttributes {
    /// The payload attributes
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub payload_attributes: PayloadAttributes,
    /// Transactions is a field for rollups: the transactions list is forced into the block
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub transactions: Option<Vec<Bytes>>,
    /// If true, the no transactions are taken out of the tx-pool, only transactions from the above
    /// Transactions list will be included.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub no_tx_pool: Option<bool>,
    /// If set, this sets the exact gas limit the block produced with.
    #[cfg_attr(
        feature = "serde",
        serde(skip_serializing_if = "Option::is_none", with = "alloy_serde::quantity::opt")
    )]
    pub gas_limit: Option<u64>,
    /// If set, this sets the EIP-1559 parameters for the block.
    ///
    /// Prior to Holocene activation, this field should always be [None].
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub eip_1559_params: Option<B64>,
}

impl OpPayloadAttributes {
    /// Encodes the `eip1559` parameters for the payload.
    pub fn get_holocene_extra_data(
        &self,
        default_base_fee_params: BaseFeeParams,
    ) -> Result<Bytes, EIP1559ParamError> {
        self.eip_1559_params
            .map(|params| encode_holocene_extra_data(params, default_base_fee_params))
            .ok_or(EIP1559ParamError::NoEIP1559Params)?
    }

    /// Extracts the Holocene 1599 parameters from the encoded form:
    /// <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/holocene/exec-engine.md#eip1559params-encoding>
    ///
    /// Returns (`elasticity`, `denominator`)
    pub fn decode_eip_1559_params(&self) -> Option<(u32, u32)> {
        self.eip_1559_params.map(decode_eip_1559_params)
    }

    /// Decode each transaction and return an iterator of decoded `OpTxEnvelope`s.
    /// Returns iterator over successfully decoded OpTxEnvelope.
    pub fn try_to_decoded(
        &self,
    ) -> impl Iterator<Item = Result<OpTxEnvelope, SignatureError>> + '_ {
        self.transactions.iter().flatten().map(|tx_bytes| {
            OpTxEnvelope::decode_2718(&mut tx_bytes.as_ref())
                .map_err(|_| SignatureError::FromBytes("Failed to decode OpTxEnvelope"))
        })
    }

    /// Returns iterator over decoded transactions with their original encoded bytes.
    pub fn decoded_transactions_with_encoded(
        &self,
    ) -> impl Iterator<Item = Result<WithEncoded<OpTxEnvelope>, SignatureError>> + '_ {
        self.transactions
            .iter()
            .flatten()
            .zip(self.try_to_decoded())
            .map(|(tx_bytes, result)| result.map(|op_tx| WithEncoded::new(tx_bytes.clone(), op_tx)))
    }

    /// Decode and recover signature from each transaction.
    pub fn try_to_recovered(
        &self,
    ) -> impl Iterator<Item = Result<Recovered<OpTxEnvelope>, SignatureError>> + '_ {
        self.decoded_transactions_with_encoded().map(|res| {
            res.and_then(|with| {
                let eth_env = with
                    .into_value()
                    .try_into_eth_envelope()
                    .map_err(|_| SignatureError::FromBytes("Failed to convert to eth envelope"))?;

                let recovered = eth_env
                    .try_into_recovered()
                    .map_err(|_| SignatureError::FromBytes("Failed to recover signature"))?;

                let recovered_op_tx =
                    OpTxEnvelope::try_from_eth_envelope(recovered.inner().clone()).map_err(
                        |_| SignatureError::FromBytes("Failed to convert back from eth envelope"),
                    )?;

                Ok(Recovered::new_unchecked(recovered_op_tx, recovered.signer()))
            })
        })
    }

    /// Returns iterator over successfully recovered transactions with encoded bytes.
    pub fn recovered_transactions_with_encoded(
        &self,
    ) -> impl Iterator<Item = Result<WithEncoded<Recovered<OpTxEnvelope>>, SignatureError>> + '_
    {
        self.decoded_transactions_with_encoded().zip(self.try_to_recovered()).map(
            |(decoded_result, recovered_result)| match (decoded_result, recovered_result) {
                (Ok(decoded), Ok(recovered)) => {
                    let encoded = decoded.encoded_bytes().clone();
                    Ok(WithEncoded::new(encoded, recovered))
                }
                (Err(e), _) | (_, Err(e)) => Err(e),
            },
        )
    }

    /// Filter only successfully recovered transactions
    pub fn recovered_transactions(&self) -> impl Iterator<Item = Recovered<OpTxEnvelope>> + '_ {
        self.try_to_recovered().filter_map(Result::ok)
    }
}

#[cfg(all(test, feature = "serde"))]
mod test {
    use super::*;
    use alloc::vec;
    use alloy_primitives::{Address, B256, b64};
    use alloy_rpc_types_engine::PayloadAttributes;
    use core::str::FromStr;

    #[test]
    fn test_serde_roundtrip_attributes_pre_holocene() {
        let attributes = OpPayloadAttributes {
            payload_attributes: PayloadAttributes {
                timestamp: 0x1337,
                prev_randao: B256::ZERO,
                suggested_fee_recipient: Address::ZERO,
                withdrawals: Default::default(),
                parent_beacon_block_root: Some(B256::ZERO),
            },
            transactions: Some(vec![b"hello".to_vec().into()]),
            no_tx_pool: Some(true),
            gas_limit: Some(42),
            eip_1559_params: None,
        };

        let ser = serde_json::to_string(&attributes).unwrap();
        let de: OpPayloadAttributes = serde_json::from_str(&ser).unwrap();

        assert_eq!(attributes, de);
    }

    #[test]
    fn test_serde_roundtrip_attributes_post_holocene() {
        let attributes = OpPayloadAttributes {
            payload_attributes: PayloadAttributes {
                timestamp: 0x1337,
                prev_randao: B256::ZERO,
                suggested_fee_recipient: Address::ZERO,
                withdrawals: Default::default(),
                parent_beacon_block_root: Some(B256::ZERO),
            },
            transactions: Some(vec![b"hello".to_vec().into()]),
            no_tx_pool: Some(true),
            gas_limit: Some(42),
            eip_1559_params: Some(b64!("0000dead0000beef")),
        };

        let ser = serde_json::to_string(&attributes).unwrap();
        let de: OpPayloadAttributes = serde_json::from_str(&ser).unwrap();

        assert_eq!(attributes, de);
    }

    #[test]
    fn test_get_extra_data_post_holocene() {
        let attributes = OpPayloadAttributes {
            eip_1559_params: Some(B64::from_str("0x0000000800000008").unwrap()),
            ..Default::default()
        };
        let extra_data = attributes.get_holocene_extra_data(BaseFeeParams::new(80, 60));
        assert_eq!(extra_data.unwrap(), Bytes::copy_from_slice(&[0, 0, 0, 0, 8, 0, 0, 0, 8]));
    }

    #[test]
    fn test_get_extra_data_post_holocene_default() {
        let attributes =
            OpPayloadAttributes { eip_1559_params: Some(B64::ZERO), ..Default::default() };
        let extra_data = attributes.get_holocene_extra_data(BaseFeeParams::new(80, 60));
        assert_eq!(extra_data.unwrap(), Bytes::copy_from_slice(&[0, 0, 0, 0, 80, 0, 0, 0, 60]));
    }
}
