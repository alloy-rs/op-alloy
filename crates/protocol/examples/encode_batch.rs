//! An example encoding and decoding a [SingleBatch].

use alloy_consensus::{SignableTransaction, TxEip1559};
use alloy_eips::eip2718::{Decodable2718, Encodable2718};
use alloy_primitives::{Address, BlockHash, Bytes, PrimitiveSignature, U256};
use alloy_rlp::{Decodable, Encodable};
use op_alloy_consensus::OpTxEnvelope;
use op_alloy_protocol::SingleBatch;

fn example_transactions() -> Vec<Bytes> {
    let mut transactions = Vec::new();

    // First Transaction in the batch.
    let tx = TxEip1559 {
        chain_id: 10u64,
        nonce: 2,
        max_fee_per_gas: 3,
        max_priority_fee_per_gas: 4,
        gas_limit: 5,
        to: Address::left_padding_from(&[6]).into(),
        value: U256::from(7_u64),
        input: vec![8].into(),
        access_list: Default::default(),
    };
    let sig = PrimitiveSignature::test_signature();
    let tx_signed = tx.into_signed(sig);
    let envelope: OpTxEnvelope = tx_signed.into();
    let encoded = envelope.encoded_2718();
    transactions.push(encoded.clone().into());
    let mut slice = encoded.as_slice();
    let decoded = OpTxEnvelope::decode_2718(&mut slice).unwrap();
    assert!(matches!(decoded, OpTxEnvelope::Eip1559(_)));

    // Second transaction in the batch.
    let tx = TxEip1559 {
        chain_id: 10u64,
        nonce: 2,
        max_fee_per_gas: 3,
        max_priority_fee_per_gas: 4,
        gas_limit: 5,
        to: Address::left_padding_from(&[7]).into(),
        value: U256::from(7_u64),
        input: vec![8].into(),
        access_list: Default::default(),
    };
    let sig = PrimitiveSignature::test_signature();
    let tx_signed = tx.into_signed(sig);
    let envelope: OpTxEnvelope = tx_signed.into();
    let encoded = envelope.encoded_2718();
    transactions.push(encoded.clone().into());
    let mut slice = encoded.as_slice();
    let decoded = OpTxEnvelope::decode_2718(&mut slice).unwrap();
    assert!(matches!(decoded, OpTxEnvelope::Eip1559(_)));

    transactions
}

fn main() {
    // Use the example transaction
    let transactions = example_transactions();

    // Construct a basic `SingleBatch`
    let parent_hash = BlockHash::ZERO;
    let epoch_num = 1;
    let epoch_hash = BlockHash::ZERO;
    let timestamp = 1;

    let single_batch = SingleBatch { parent_hash, epoch_num, epoch_hash, timestamp, transactions };

    // Encode the batch.
    let mut encoded = Vec::new();
    single_batch.encode(&mut encoded);
    println!("Encoded: {:?}", alloy_primitives::hex::encode(encoded.clone()));
    let decoded = SingleBatch::decode(&mut encoded.as_slice()).unwrap();
    assert_eq!(single_batch, decoded);

    println!("Successfully encoded and decoded the batch.");
}
