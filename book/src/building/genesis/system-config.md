# System Config

The system configuration is a set of configurable chain parameters
defined in a contract on L1. These parameters can be changed through
the system config contract, emitting events that are picked up by
the [rollup node derivation process][derivation]. To dive deeper
into the System Config, visit the [OP Stack Specifications][specs].

## `SystemConfig` Type

The [`SystemConfig`][sc] type is defined in [`op-alloy-genesis`][genesis].

Parameters defined in the [`SystemConfig`][sc] are expected to be updated
through L1 receipts, using the [`update_with_receipts`][update] method.

## Holocene Updates

The [Holocene Hardfork][holocene] introduced an update to the [`SystemConfig`][sc]
type, adding EIP-1559 parameters to the config.

The [`SystemConfig`][sc] type in [`op-alloy-genesis`][genesis] provides a method
called [`eip_1559_params`][eip] that returns the EIP-1559 parameters encoded as
a [`B64`][b64].

[holocene]: https://specs.optimism.io/protocol/holocene/overview.html
[b64]: https://docs.rs/alloy-primitives/latest/alloy_primitives/aliases/type.B64.html
[eip]: https://docs.rs/op-alloy-genesis/latest/op_alloy_genesis/system/struct.SystemConfig.html#method.eip_1559_params
[update]: https://docs.rs/op-alloy-genesis/latest/op_alloy_genesis/system/struct.SystemConfig.html#method.update_with_receipts
[sc]: https://docs.rs/op-alloy-genesis/latest/op_alloy_genesis/system/struct.SystemConfig.html
[specs]: https://specs.optimism.io/protocol/system-config.html#system-config
[derivation]: https://specs.optimism.io/protocol/derivation.html
[genesis]: https://crates.io/crates/op-alloy-genesis
