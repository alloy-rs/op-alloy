# Transactions

The [`op-alloy-consensus`][consensus] crate contains types for Optimism EL
consensus and communication. Most notably, Optimism extends the Ethereum
[EIP-2718][2718] transaction envelope to include a deposit variant.

This doc breaks down transaction and other types defined in the
[`op-alloy-consensus`][consensus] crate.

## [`OpTxEnvelope`][envelope]

The [`OpTxEnvelope`][envelope] type is based on [Alloy][alloy]'s
[`TxEnvelope`][tx-envelope] type.

Optimism modifies the `TxEnvelope` to the following.
- Legacy
- EIP-2930
- EIP-1559
- EIP-7702
- Deposit

Deposit is a custom transaction type that is either an L1 attributes
deposit transaction or a user-submitted deposit transaction. Read more
about deposit transactions in [the specs][specs].

## Transaction Types ([`OpTxType`][ty])

The [`OpTxType`][ty] enumerates the transaction types using their byte identifier,
represents as a `u8` in rust.

## [`OpBlock`][block]

[`op-alloy-consensus`][consensus] exports an Optimism block type.

## Receipt Types

Just like [`op-alloy-consensus`][consensus] defines transaction types,
it also defines associated receipt types.

[`OpReceiptEnvelope`][ore] defines an [Eip-2718][2718] receipt envelope type
modified for the OP Stack. It contains the following variants - mapping
directly to the `OpTxEnvelope` variants defined above.

- Legacy
- EIP-2930
- EIP-1559
- EIP-7702
- Deposit

There is also an [`OpDepositReceipt`][odr] type, extending the alloy receipt
type with a deposit nonce and deposit receipt version.

## Hardforks

Aside from transactions and receipts, [`op-alloy-consensus`][consensus] exports
one other core primitive called [`Hardforks`][hardforks].

Hardforks provides hardfork transaction constructors - that is, it provides
methods that return upgrade transactions for each hardfork. Some of these
are the following.

- [`Hardforks::ecotone_txs()`][ecotone]
- [`Hardforks::fjord_txs()`][fjord]


[fjord]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/hardforks/struct.Hardforks.html#method.fjord_txs
[ecotone]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/hardforks/struct.Hardforks.html#method.ecotone_txs
[hardforks]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/hardforks/struct.Hardforks.html
[odr]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/struct.OpDepositReceipt.html
[ore]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/enum.OpReceiptEnvelope.html
[block]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/type.OpBlock.html
[ty]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/enum.OpTxType.html
[specs]: https://specs.optimism.io/protocol/deposits.html
[tx-envelope]: https://docs.rs/alloy-consensus/latest/alloy_consensus/transaction/enum.TxEnvelope.html
[envelope]: https://docs.rs/op-alloy-consensus/latest/op_alloy_consensus/enum.OpTxEnvelope.html
[2718]: https://eips.ethereum.org/EIPS/eip-2718
[consensus]: https://crates.io/crates/op-alloy-consensus
