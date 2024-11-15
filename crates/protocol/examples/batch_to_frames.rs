//! An example encoding and decoding a [SingleBatch].
//!
//! This example demonstrates EIP-2718 encoding a [SingleBatch]
//! through a [ChannelOut] and into individual [Frame]s.
//!
//! Notice, the raw batch is first _encoded_.
//! Once encoded, it is compressed into raw data that the channel is constructed with.
//!
//! The [ChannelOut] then outputs frames individually using the maximum frame size,
//! in this case hardcoded to 100, to construct the frames.
//!
//! Finally, once [Frame]s are built from the [ChannelOut], they are encoded and ready
//! to be batch-submitted to the data availability layer.

use op_alloy_genesis::RollupConfig;
use op_alloy_consensus::OpTxEnvelope;
use alloy_rlp::{Decodable, Encodable};
use alloy_consensus::{SignableTransaction, TxEip1559};
use brotli::enc::{BrotliCompress, BrotliEncoderParams};
use alloy_eips::eip2718::{Decodable2718, Encodable2718};
use op_alloy_protocol::{random_channel_id, ChannelOut, SingleBatch};
use alloy_primitives::{hex, Address, BlockHash, Bytes, PrimitiveSignature, U256};

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
    let decoded = SingleBatch::decode(&mut encoded.as_slice()).unwrap();
    assert_eq!(single_batch, decoded);
    println!("Encoded Batch: {}", hex::encode(&encoded));

    // Compress the encoded batch.
    let compressed = compress_brotli(&encoded);
    let expected = hex!("1b1201f82f0f6c3734f4821cd090ef3979d71a98e7e483b1dccdd525024c0ef16f425c7b4976a7acc0c94a0514b72c096d4dcc52f0b22dae193c70c86d0790a304a08152c8250031d011fe80c23600004009b67bf33d17f4b6831018ad78018613b3403bc2fc6da91e8fc8a29031b3417774a33bf1f30534ea695b09eb3bf26cb553530e9fa2120e755ec5bd3a2bc75b2ee300");
    assert_eq!(compressed, expected);
    println!("Brotli-compressed batch: {}", hex::encode(&compressed));

    // Create a new channel.
    let id = random_channel_id();
    let config = RollupConfig::default();
    let mut channel_out = ChannelOut::new(id, &config);

    // Add the compressed batch to the `ChannelOut`.
    channel_out.add_raw_compressed_batch(compressed.into());

    // Output frames
    while channel_out.ready_bytes() > 0 {
        let frame = channel_out.output_frame(100).expect("outputs frame");
        println!("Frame: {}", alloy_primitives::hex::encode(&frame.encode()));
        if channel_out.ready_bytes() <= 100 {
            channel_out.close();
        }
    }

    assert!(channel_out.closed);
}

/// Compresses the given bytes data using the Brotli compressor implemented
/// in the [`brotli`](https://crates.io/crates/brotli) crate.
pub fn compress_brotli(mut input: &[u8]) -> Vec<u8> {
    let mut output = vec![];
    BrotliCompress(&mut input, &mut output, &BrotliEncoderParams::default()).expect("succeeds");
    output
}

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
