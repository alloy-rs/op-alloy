//! This example demonstrates encoding a [Channel] using an EIP-2718 encoded batch.
//!
//! Notice, the raw batch is _encoded_ and **not** compressed.
//! In order to construct a valid [Channel], the encoded batch must be compressed
//! prior to splitting the data into frames as part of the [Channel] construction.

use alloy_primitives::hex;
use brotli::enc::{BrotliCompress, BrotliEncoderParams};
use op_alloy_genesis::RollupConfig;
use op_alloy_protocol::{ChannelId, ChannelOut};

/// Compresses the given bytes data using the Brotli compressor implemented
/// in the [`brotli`](https://crates.io/crates/brotli) crate.
pub fn compress_brotli(mut input: &[u8]) -> Vec<u8> {
    let mut output = vec![];
    BrotliCompress(&mut input, &mut output, &BrotliEncoderParams::default()).expect("succeeds");
    output
}

fn main() {
    // An example encoded batch taken from the `encode_batch.rs` example.
    let encoded_batch = hex!("f90110a0000000000000000000000000000000000000000000000000000000000000000001a0000000000000000000000000000000000000000000000000000000000000000001f8cab86302f8600a020403059400000000000000000000000000000000000000060708c080a0840cfc572845f5786e702984c2a582528cad4b49b2a10b9db1be7fca90058565a025e7109ceb98168d95b09b18bbf6b685130e0562f233877d492b94eee0c5b6d1b86302f8600a020403059400000000000000000000000000000000000000070708c080a0840cfc572845f5786e702984c2a582528cad4b49b2a10b9db1be7fca90058565a025e7109ceb98168d95b09b18bbf6b685130e0562f233877d492b94eee0c5b6d1");

    // Compress the encoded batch.
    let compressed = compress_brotli(&encoded_batch);
    let expected = hex!("1b1201f82f0f6c3734f4821cd090ef3979d71a98e7e483b1dccdd525024c0ef16f425c7b4976a7acc0c94a0514b72c096d4dcc52f0b22dae193c70c86d0790a304a08152c8250031d011fe80c23600004009b67bf33d17f4b6831018ad78018613b3403bc2fc6da91e8fc8a29031b3417774a33bf1f30534ea695b09eb3bf26cb553530e9fa2120e755ec5bd3a2bc75b2ee300");
    assert_eq!(compressed, expected);
    println!("Successfully compressed the batch using brotli.");

    // Create a new channel.
    let id = ChannelId::random();
    let config = RollupConfig::default();
    let channel = ChannelOut::new(id, config);

    // Add the compressed batch to the `ChannelOut`.
    channel.add_raw_compressed_batch(compressed);

    // Output frames
    while let Ok(frame) = channel_out.output_frame(100) {
        println!("Frame: {:?}", frame);
    }
}
