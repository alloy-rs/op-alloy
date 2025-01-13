# Building

This section offers in-depth documentation into the various `op-alloy` crates.
Some of the primary crates and their types are listed below.

  [`RollupConfig`][rollup-config] and [`SystemConfig`][system-config] types.
- [`op-alloy-consensus`][op-alloy-consensus] provides [`OpBlock`][op-block],
  [`OpTxEnvelope`][op-tx-envelope], [`OpReceiptEnvelope`][op-rx-envelope],
  [`Hardforks`][hardforks], and more.
- [`op-alloy-rpc-types-engine`][op-alloy-rpc-types-engine] provides the
  [`OpPayloadAttributes`][op-payload-attributes] and
  [`OpAttributesWithParent`][op-attributes-with-parent].
- [`op-alloy-protocol`][op-alloy-protocol] provides [`Frame`][frame],
  [`Channel`][channel], [`Batch`][batch] types and more.


<!-- Links -->

[op-block]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/type.OpBlock.html
[op-tx-envelope]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/enum.OpTxEnvelope.html
[op-rx-envelope]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/enum.OpReceiptEnvelope.html
[hardforks]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/hardforks/struct.Hardforks.html

[op-payload-attributes]: https://docs.rs/op-alloy-rpc-types-engine/latest/op_alloy_rpc_types_engine/struct.OpPayloadAttributes.html
[op-attributes-with-parent]: https://docs.rs/op-alloy-rpc-types-engine/latest/op_alloy_rpc_types_engine/struct.OpAttributesWithParent.html

[frame]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Frame.html
[channel]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Channel.html
[batch]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/enum.Batch.html

[op-alloy-consensus]: https://crates.io/crates/op-alloy-consensus
[op-alloy-protocol]: https://crates.io/crates/op-alloy-protocol
[op-alloy-rpc-types-engine]: https://crates.io/crates/op-alloy-rpc-types-engine
