use super::OpTxType;
use alloy_consensus::Transaction;
use alloy_eips::eip2930::AccessList;
use alloy_primitives::{Address, Bytes, ChainId, TxKind, B256, U256};
use alloy_rlp::{
    Buf, BufMut, Decodable, Encodable, Error as DecodeError, Header, EMPTY_STRING_CODE,
};
use core::mem;

/// Deposit transactions, also known as deposits are initiated on L1, and executed on L2.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TxDeposit {
    /// Hash that uniquely identifies the source of the deposit.
    pub source_hash: B256,
    /// The address of the sender account.
    pub from: Address,
    /// The address of the recipient account, or the null (zero-length) address if the deposited
    /// transaction is a contract creation.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "TxKind::is_create"))]
    pub to: TxKind,
    /// The ETH value to mint on L2.
    #[cfg_attr(feature = "serde", serde(default, with = "alloy_serde::quantity::opt"))]
    pub mint: Option<u128>,
    ///  The ETH value to send to the recipient account.
    pub value: U256,
    /// The gas limit for the L2 transaction.
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity", rename = "gas"))]
    pub gas_limit: u128,
    /// Field indicating if this transaction is exempt from the L2 gas limit.
    #[cfg_attr(feature = "serde", serde(with = "alloy_serde::quantity", rename = "isSystemTx"))]
    pub is_system_transaction: bool,
    /// Input has two uses depending if transaction is Create or Call (if `to` field is None or
    /// Some).
    pub input: Bytes,
}

impl TxDeposit {
    /// Decodes the inner [TxDeposit] fields from RLP bytes.
    ///
    /// NOTE: This assumes a RLP header has already been decoded, and _just_ decodes the following
    /// RLP fields in the following order:
    ///
    /// - `source_hash`
    /// - `from`
    /// - `to`
    /// - `mint`
    /// - `value`
    /// - `gas_limit`
    /// - `is_system_transaction`
    /// - `input`
    pub(crate) fn decode_fields(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        Ok(Self {
            source_hash: Decodable::decode(buf)?,
            from: Decodable::decode(buf)?,
            to: Decodable::decode(buf)?,
            mint: if *buf.first().ok_or(DecodeError::InputTooShort)? == EMPTY_STRING_CODE {
                buf.advance(1);
                None
            } else {
                Some(Decodable::decode(buf)?)
            },
            value: Decodable::decode(buf)?,
            gas_limit: Decodable::decode(buf)?,
            is_system_transaction: Decodable::decode(buf)?,
            input: Decodable::decode(buf)?,
        })
    }

    /// Outputs the length of the transaction's fields, without a RLP header or length of the
    /// eip155 fields.
    pub(crate) fn fields_len(&self) -> usize {
        self.source_hash.length()
            + self.from.length()
            + self.to.length()
            + self.mint.map_or(1, |mint| mint.length())
            + self.value.length()
            + self.gas_limit.length()
            + self.is_system_transaction.length()
            + self.input.0.length()
    }

    /// Encodes only the transaction's fields into the desired buffer, without a RLP header.
    /// <https://github.com/ethereum-optimism/specs/blob/main/specs/protocol/deposits.md#the-deposited-transaction-type>
    pub(crate) fn encode_fields(&self, out: &mut dyn alloy_rlp::BufMut) {
        self.source_hash.encode(out);
        self.from.encode(out);
        self.to.encode(out);
        if let Some(mint) = self.mint {
            mint.encode(out);
        } else {
            out.put_u8(EMPTY_STRING_CODE);
        }
        self.value.encode(out);
        self.gas_limit.encode(out);
        self.is_system_transaction.encode(out);
        self.input.encode(out);
    }

    /// Calculates a heuristic for the in-memory size of the [TxDeposit] transaction.
    #[inline]
    pub fn size(&self) -> usize {
        mem::size_of::<B256>() + // source_hash
        mem::size_of::<Address>() + // from
        self.to.size() + // to
        mem::size_of::<Option<u128>>() + // mint
        mem::size_of::<U256>() + // value
        mem::size_of::<u128>() + // gas_limit
        mem::size_of::<bool>() + // is_system_transaction
        self.input.len() // input
    }

    /// Get the transaction type
    pub(crate) const fn tx_type(&self) -> OpTxType {
        OpTxType::Deposit
    }

    /// Inner encoding function that is used for both rlp [`Encodable`] trait and for calculating
    /// hash that for eip2718 does not require rlp header
    pub fn encode_inner(&self, out: &mut dyn BufMut, with_header: bool) {
        let payload_length = self.fields_len();
        if with_header {
            Header {
                list: false,
                payload_length: 1 + Header { list: true, payload_length }.length() + payload_length,
            }
            .encode(out);
        }
        out.put_u8(self.tx_type() as u8);
        let header = Header { list: true, payload_length };
        header.encode(out);
        self.encode_fields(out);
    }

    /// Output the length of the RLP signed transaction encoding.
    ///
    /// If `with_header` is true, the length includes the RLP header.
    pub fn encoded_len(&self, with_header: bool) -> usize {
        // Count the length of the payload
        let payload_length = self.fields_len();

        // 'transaction type byte length' + 'header length' + 'payload length'
        let inner_payload_length =
            1 + Header { list: true, payload_length }.length() + payload_length;

        if with_header {
            Header { list: true, payload_length: inner_payload_length }.length()
                + inner_payload_length
        } else {
            inner_payload_length
        }
    }
}

impl Transaction for TxDeposit {
    fn chain_id(&self) -> Option<ChainId> {
        None
    }

    fn nonce(&self) -> u64 {
        0u64
    }

    fn gas_limit(&self) -> u128 {
        self.gas_limit
    }

    fn gas_price(&self) -> Option<u128> {
        None
    }

    fn to(&self) -> TxKind {
        self.to
    }

    fn value(&self) -> U256 {
        self.value
    }

    fn input(&self) -> &[u8] {
        &self.input
    }

    fn access_list(&self) -> Option<&AccessList> {
        None
    }

    fn blob_versioned_hashes(&self) -> Option<&[B256]> {
        None
    }

    fn max_fee_per_gas(&self) -> u128 {
        0
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        None
    }

    fn priority_fee_or_price(&self) -> u128 {
        0
    }

    fn ty(&self) -> u8 {
        OpTxType::Deposit as u8
    }

    fn max_fee_per_blob_gas(&self) -> Option<u128> {
        None
    }

    fn authorization_list(&self) -> Option<&[alloy_eips::eip7702::SignedAuthorization]> {
        None
    }
}

impl Encodable for TxDeposit {
    fn encode(&self, out: &mut dyn BufMut) {
        Header { list: true, payload_length: self.fields_len() }.encode(out);
        self.encode_fields(out);
    }

    fn length(&self) -> usize {
        let payload_length = self.fields_len();
        Header { list: true, payload_length }.length() + payload_length
    }
}

impl Decodable for TxDeposit {
    fn decode(data: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let header = Header::decode(data)?;
        let remaining_len = data.len();

        if header.payload_length > remaining_len {
            return Err(alloy_rlp::Error::InputTooShort);
        }

        Self::decode_fields(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::TxEnvelope;
    use alloy_primitives::hex;
    use alloy_rlp::BytesMut;

    #[test]
    fn test_rlp_roundtrip() {
        let bytes = Bytes::from_static(&hex!("7ef9015aa044bae9d41b8380d781187b426c6fe43df5fb2fb57bd4466ef6a701e1f01e015694deaddeaddeaddeaddeaddeaddeaddeaddead000194420000000000000000000000000000000000001580808408f0d18001b90104015d8eb900000000000000000000000000000000000000000000000000000000008057650000000000000000000000000000000000000000000000000000000063d96d10000000000000000000000000000000000000000000000000000000000009f35273d89754a1e0387b89520d989d3be9c37c1f32495a88faf1ea05c61121ab0d1900000000000000000000000000000000000000000000000000000000000000010000000000000000000000002d679b567db6187c0c8323fa982cfb88b74dbcc7000000000000000000000000000000000000000000000000000000000000083400000000000000000000000000000000000000000000000000000000000f4240"));
        let tx_a = TxDeposit::decode(&mut bytes[1..].as_ref()).unwrap();
        let mut buf_a = BytesMut::default();
        tx_a.encode(&mut buf_a);
        assert_eq!(&buf_a[..], &bytes[1..]);
    }

    #[test]
    fn test_encode_decode_fields() {
        let original = TxDeposit {
            source_hash: B256::default(),
            from: Address::default(),
            to: TxKind::default(),
            mint: Some(100),
            value: U256::default(),
            gas_limit: 50000,
            is_system_transaction: true,
            input: Bytes::default(),
        };

        let mut buffer = BytesMut::new();
        original.encode_fields(&mut buffer);
        let decoded = TxDeposit::decode_fields(&mut &buffer[..]).expect("Failed to decode");

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_encode_with_and_without_header() {
        let tx_deposit = TxDeposit {
            source_hash: B256::default(),
            from: Address::default(),
            to: TxKind::default(),
            mint: Some(100),
            value: U256::default(),
            gas_limit: 50000,
            is_system_transaction: true,
            input: Bytes::default(),
        };

        let mut buffer_with_header = BytesMut::new();
        tx_deposit.encode(&mut buffer_with_header);

        let mut buffer_without_header = BytesMut::new();
        tx_deposit.encode_fields(&mut buffer_without_header);

        assert!(buffer_with_header.len() > buffer_without_header.len());
    }

    #[test]
    fn test_payload_length() {
        let tx_deposit = TxDeposit {
            source_hash: B256::default(),
            from: Address::default(),
            to: TxKind::default(),
            mint: Some(100),
            value: U256::default(),
            gas_limit: 50000,
            is_system_transaction: true,
            input: Bytes::default(),
        };

        assert!(tx_deposit.size() > tx_deposit.fields_len());
    }

    #[test]
    fn test_encode_inner_with_and_without_header() {
        let tx_deposit = TxDeposit {
            source_hash: B256::default(),
            from: Address::default(),
            to: TxKind::default(),
            mint: Some(100),
            value: U256::default(),
            gas_limit: 50000,
            is_system_transaction: true,
            input: Bytes::default(),
        };

        let mut buffer_with_header = BytesMut::new();
        tx_deposit.encode_inner(&mut buffer_with_header, true);

        let mut buffer_without_header = BytesMut::new();
        tx_deposit.encode_inner(&mut buffer_without_header, false);

        println!("buffer_with_header: {:?}", buffer_with_header);
        println!("buffer_without_header: {:?}", buffer_without_header);

        assert!(buffer_with_header.len() > buffer_without_header.len());
    }

    #[test]
    fn test_payload_length_header() {
        let tx_deposit = TxDeposit {
            source_hash: B256::default(),
            from: Address::default(),
            to: TxKind::default(),
            mint: Some(100),
            value: U256::default(),
            gas_limit: 50000,
            is_system_transaction: true,
            input: Bytes::default(),
        };

        let total_len = tx_deposit.encoded_len(true);
        let len_without_header = tx_deposit.encoded_len(false);

        assert!(total_len > len_without_header);
    }
}
