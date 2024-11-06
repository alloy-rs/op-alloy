# Rollup Configs

Rollup configurations are a consensus construct used to configure an Optimism Consensus client.
When an OP Stack chain is deployed into production or consensus nodes are configured to sync the chain,
certain consensus parameters can be configured. These parameters are defined in the
[OP Stack specs][spec-configurability].

Consensus parameters are consumed by OP Stack software through the `RollupConfig` type defined in the
[`op-alloy-genesis`][genesis] crate.

## `RollupConfig` Type

The [`RollupConfig`][rc] type is defined in [`op-alloy-genesis`][genesis].

A predefined rollup config can be loaded from a given L2 chain id using
the [`rollup_config_from_chain_id`][rcid] method. An example is shown below.

```rust
use op_alloy_genesis::{OP_MAINNET_CONFIG, rollup_config_from_chain_id};

let op_mainnet_config = rollup_config_from_chain_id(10).expect("infallible");
assert_eq!(OP_MAINNET_CONFIG, op_mainnet_config);
```

The `OP_MAINNET_CONFIG` is one of the predefined rollup configs exported by
the [`op-alloy-genesis`][genesis] crate. Other predefined configs include
the following.

- `OP_MAINNET_CONFIG`
- `OP_SEPOLIA_CONFIG`
- `BASE_MAINNET_CONFIG`
- `BASE_SEPOLIA_CONFIG`


<!-- Links -->

{{#include ../../links.md}}
