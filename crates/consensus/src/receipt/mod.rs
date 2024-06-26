use alloy_consensus::TxReceipt;

mod envelope;
pub use envelope::OpReceiptEnvelope;

mod receipts;
pub use receipts::{OpDepositReceipt, OpDepositReceiptWithBloom};

/// Receipt is the result of a transaction execution.
pub trait OpTxReceipt: TxReceipt {
    /// Returns the deposit nonce of the transaction.
    fn deposit_nonce(&self) -> Option<u64>;

    /// Returns the deposit receipt version of the transaction.
    fn deposit_receipt_version(&self) -> Option<u64>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_consensus::{Receipt, ReceiptWithBloom};
    use alloy_eips::eip2718::Encodable2718;
    use alloy_primitives::{address, b256, bytes, hex, Bytes, Log, LogData};
    use alloy_rlp::{Decodable, Encodable};

    #[cfg(not(feature = "std"))]
    use alloc::{vec, vec::Vec};

    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    #[test]
    fn encode_legacy_receipt() {
        let expected = hex!("f901668001b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f85ff85d940000000000000000000000000000000000000011f842a0000000000000000000000000000000000000000000000000000000000000deada0000000000000000000000000000000000000000000000000000000000000beef830100ff");

        let mut data = vec![];
        let receipt =
            OpReceiptEnvelope::Legacy(ReceiptWithBloom {
                receipt: Receipt {
                    status: false.into(),
                    cumulative_gas_used: 0x1u128,
                    logs: vec![Log {
                        address: address!("0000000000000000000000000000000000000011"),
                        data: LogData::new_unchecked(
                            vec![
                        b256!("000000000000000000000000000000000000000000000000000000000000dead"),
                        b256!("000000000000000000000000000000000000000000000000000000000000beef"),
                    ],
                            bytes!("0100ff"),
                        ),
                    }],
                },
                logs_bloom: [0; 256].into(),
            });

        receipt.network_encode(&mut data);

        // check that the rlp length equals the length of the expected rlp
        assert_eq!(receipt.length(), expected.len());
        assert_eq!(data, expected);
    }

    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    #[test]
    fn decode_legacy_receipt() {
        let data = hex!("f901668001b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000f85ff85d940000000000000000000000000000000000000011f842a0000000000000000000000000000000000000000000000000000000000000deada0000000000000000000000000000000000000000000000000000000000000beef830100ff");

        // EIP658Receipt
        let expected =
            OpDepositReceiptWithBloom {
                receipt: OpDepositReceipt {
                    inner: Receipt {
                        status: false.into(),
                        cumulative_gas_used: 0x1u128,
                        logs: vec![Log {
                            address: address!("0000000000000000000000000000000000000011"),
                            data: LogData::new_unchecked(
                                vec![
                            b256!("000000000000000000000000000000000000000000000000000000000000dead"),
                            b256!("000000000000000000000000000000000000000000000000000000000000beef"),
                        ],
                                bytes!("0100ff"),
                            ),
                        }],
                    },
                    deposit_nonce: None,
                    deposit_receipt_version: None,
                },
                logs_bloom: [0; 256].into(),
            };

        let receipt = OpDepositReceiptWithBloom::decode(&mut &data[..]).unwrap();
        assert_eq!(receipt, expected);
    }

    #[test]
    fn gigantic_receipt() {
        let receipt = OpDepositReceipt {
            inner: Receipt {
                cumulative_gas_used: 16747627,
                status: true.into(),
                logs: vec![
                    Log {
                        address: address!("4bf56695415f725e43c3e04354b604bcfb6dfb6e"),
                        data: LogData::new_unchecked(
                            vec![b256!(
                                "c69dc3d7ebff79e41f525be431d5cd3cc08f80eaf0f7819054a726eeb7086eb9"
                            )],
                            Bytes::from(vec![1; 0xffffff]),
                        ),
                    },
                    Log {
                        address: address!("faca325c86bf9c2d5b413cd7b90b209be92229c2"),
                        data: LogData::new_unchecked(
                            vec![b256!(
                                "8cca58667b1e9ffa004720ac99a3d61a138181963b294d270d91c53d36402ae2"
                            )],
                            Bytes::from(vec![1; 0xffffff]),
                        ),
                    },
                ],
            },
            deposit_nonce: None,
            deposit_receipt_version: None,
        }
        .with_bloom();

        let mut data = vec![];

        receipt.encode(&mut data);
        let decoded = OpDepositReceiptWithBloom::decode(&mut &data[..]).unwrap();

        // receipt.clone().to_compact(&mut data);
        // let (decoded, _) = Receipt::from_compact(&data[..], data.len());
        assert_eq!(decoded, receipt);
    }

    #[test]
    fn regolith_receipt_roundtrip() {
        let data = hex!("f9010c0182b741b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c0833d3bbf");

        // Deposit Receipt (post-regolith)
        let expected = OpDepositReceiptWithBloom {
            receipt: OpDepositReceipt {
                inner: Receipt { cumulative_gas_used: 46913, logs: vec![], status: true.into() },
                deposit_nonce: Some(4012991),
                deposit_receipt_version: None,
            },
            logs_bloom: [0; 256].into(),
        };

        let receipt = OpDepositReceiptWithBloom::decode(&mut &data[..]).unwrap();
        assert_eq!(receipt, expected);

        let mut buf = Vec::new();
        receipt.encode(&mut buf);
        assert_eq!(buf, &data[..]);
    }

    #[test]
    fn post_canyon_receipt_roundtrip() {
        let data = hex!("f9010d0182b741b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000c0833d3bbf01");

        // Deposit Receipt (post-regolith)
        let expected = OpDepositReceiptWithBloom {
            receipt: OpDepositReceipt {
                inner: Receipt { cumulative_gas_used: 46913, logs: vec![], status: true.into() },
                deposit_nonce: Some(4012991),
                deposit_receipt_version: Some(1),
            },
            logs_bloom: [0; 256].into(),
        };

        let receipt = OpDepositReceiptWithBloom::decode(&mut &data[..]).unwrap();
        assert_eq!(receipt, expected);

        let mut buf = Vec::new();
        expected.encode(&mut buf);
        assert_eq!(buf, &data[..]);
    }
}
