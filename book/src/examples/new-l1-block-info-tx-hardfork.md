# Create a L1BlockInfoTx Variant for a new Hardfork

This example walks through creating a variant of the [`L1BlockInfoTx`][info-tx]
for a new Hardfork.

> [!NOTE]
>
> This example is very verbose.
> To grok required changes, view [this PR diff][pr-diff]
> which introduces Isthmus hardfork changes to the `L1BlockInfoTx` with a new variant.


## Required Genesis Updates

The first updates that need to be made are to [`op-alloy-genesis`][genesis]
types, namely the [`RollupConfig`][rc] and [`HardForkConfiguration`][hfc].

First, add a timestamp field to the [`RollupConfig`][rc]. Let's use the
hardfork name "Glacier" as an example.

```rust
pub struct RollupConfig {
   ...
   /// `glacier_time` sets the activation time for the Glacier network upgrade.
   /// Active if `glacier_time` != None && L2 block timestamp >= Some(glacier_time), inactive
   /// otherwise.
   #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
   pub glacier_time: Option<u64>,
   ...
}
```

Add an accessor on the [`RollupConfig`][rc] to provide a way of checking whether the
"Glacier" hardfork is active for a given timestamp. Also update the prior hardfork
accessor to call this method (let's use "Isthmus" as the prior hardfork).

```rust
    /// Returns true if Isthmus is active at the given timestamp.
    pub fn is_isthmus_active(&self, timestamp: u64) -> bool {
        self.isthmus_time.map_or(false, |t| timestamp >= t) || self.is_glacier_active(timestamp)
    }

    /// Returns true if Glacier is active at the given timestamp.
    pub fn is_glacier_active(&self, timestamp: u64) -> bool {
        self.glacier_time.map_or(false, |t| timestamp >= t)
    }
```

Lastly, add the "Glacier" timestamp to the [`HardForkConfiguration`][hfc].

```rust
pub struct HardForkConfiguration {
    ...
    /// Glacier hardfork activation time
    pub glacier_time: Option<u64>,
}
```


## Protocol Changes

Introduce a new `glacier.rs` module containing a `L1BlockInfoGlacier` type
in [`op_alloy_genesis::info` module][info-mod].

This should include a few methods used in the `L1BlockInfoTx` later.

```rust
    pub fn encode_calldata(&self) -> Bytes { ... }

    pub fn decode_calldata(r: &[u8]) -> Result<Self, DecodeError> { ... }
```

Use other hardfork variants like the [`L1BlockInfoEcotone`][ecotone]
for reference.

Next, add the new "Glacier" variant to the [`L1BlockInfoTx`][info-tx].

```rust
pub enum L1BlockInfoTx {
   ...
   Glacier(L1BlockInfoGlacier)
}
```

Update [`L1BlockInfoTx::try_new`][try-new] to construct the `L1BlockInfoGlacier`
if the hardfork is active using the `RollupConfig::is_glacier_active`.

Also, be sure to update [`L1BlockInfoTx::decode_calldata`][decode-calldata]
with the new variant decoding, as well as other [`L1BlockInfoTx`][info-tx]
methods.

Once some tests are added surrounding the decoding and encoding of the new
`L1BlockInfoGlacier` variant, all required changes are complete!

Now, [this example PR diff][pr-diff] introducing the Isthmus changes should
make sense, since it effectively implements the above changes for the Isthmus
hardfork (replacing "Glacier" with "Isthmus"). Notice, Isthmus introduces
some new "operator fee" fields as part of it's `L1BlockInfoIsthmus` type.
Some new error variants to the [`BlockInfoError`][bie] are needed as well.


<!-- Links -->

[bie]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/enum.BlockInfoError.html
[pr-diff]: https://github.com/alloy-rs/op-alloy/pull/130/files
[decode-calldata]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/enum.L1BlockInfoTx.html#method.decode_calldata
[try-new]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/enum.L1BlockInfoTx.html#method.try_new
[ecotone]: https://github.com/alloy-rs/op-alloy/blob/main/crates/protocol/src/info/ecotone.rs
[info-mod]: https://github.com/alloy-rs/op-alloy/blob/main/crates/protocol/src/info/mod.rs
[genesis]: https://docs.rs/op-alloy-genesis/latest/op_alloy_genesis/index.html
[rc]: https://docs.rs/op-alloy-genesis/latest/op_alloy_genesis/struct.RollupConfig.html
[hfc]: https://docs.rs/op-alloy-genesis/latest/op_alloy_genesis/struct.HardForkConfiguration.html
[info-tx]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/enum.L1BlockInfoTx.html
