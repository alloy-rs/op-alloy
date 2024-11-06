# Loading a Rollup Config from a Chain ID

In this section, the code examples demonstrate loading the
rollup config for the given L2 Chain ID.

Let's load the Rollup Config for OP Mainnet which hash chain id 10.

```rust
use op_alloy_genesis::{OP_MAINNET_CONFIG, rollup_config_from_chain_id};

// The chain id for OP Mainnet
let op_mainnet_id = 10;

// Load a rollup config from the chain id.
let op_mainnet_config = rollup_config_from_chain_id(op_mainnet_id).expect("infallible");

// The chain id should match the hardcoded chain id.
assert_eq!(OP_MAINNET_CONFIG, op_mainnet_config);
```

> ⚠️ Available Configs
>
> The `rollup_config_from_chain_id` method in `op-alloy-genesis` uses hardcoded
> rollup configs. But, there are only a few of these hardcoded rollup configs in
> `op-alloy-genesis`. This method and these configs are provided for `no_std`
> environments where dynamic filesystem loading at runtime is not supported
> in `no_std` environments.
>
> In a `std` environment, the [superchain][superchain] crate may be used which
> dynamically provides all rollup configs from the [superchain-registry][registry]
> for their respective chain ids.

[superchain]: https://crates.io/crates/superchain
[registry]: https://github.com/ethereum-optimism/superchain-registry
