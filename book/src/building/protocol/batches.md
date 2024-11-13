# Batches

A [Batch][batch] contains a list of transactions to be included in a specific
L2 block. Since the [Delta hardfork][delta], there are two Batch types or
variants: [`SingleBatch`][single-batch] and [`SpanBatch`][span-batch].


## Where Batches fit in the OP Stack

The [Batch][batch] is the highest-level data type in the OP Stack
derivation process that comes prior to building payload attributes.
A [Batch][batch] is constructed by taking the raw data from a
[Channel][channel], decompressing it, and decoding the [Batch][batch]
from this decompressed data.

Alternatively, when looking at the [Batch][batch] type from a batching
perspective, and not from the derivation perspective, the [Batch][batch]
type contains a list of L2 transactions and is compressed into the
[`Channel`][channel] type. In turn, the [`Channel`][channel] is split
into frames which are posted to the data availability layer through batcher
transactions.


## Contents of a `Batch`

A [`Batch`][batch] is either a [`SingleBatch`][single-batch] or a
[`SpanBatch`][span-batch], each with their own contents. Below,
these types are broken down in their respective sections.

### `SingleBatch` Type

The [`SingleBatch`][single-batch] type contains the following.
- A [`BlockHash`][block-hash] parent hash that represents the parent L2 block.
- A `u64` epoch number that identifies the [epoch][epoch] for this batch.
- A [`BlockHash`][block-hash] epoch hash.
- The timestamp for the batch as a `u64`.
- A list of EIP-2718 encoded transactions (represented as [`Bytes`][bytes]).

In order to validate the [`SingleBatch`][single-batch] once decoded,
the [`SingleBatch::check_batch`][check-batch-single] method should be used,
providing the rollup config, l1 blocks, l2 safe head, and inclusion block.

### `SpanBatch` Type

The [`SpanBatch`][span-batch] type (available since the [Delta hardfork][delta])
comprises the data needed to build a "span" of multiple L2 blocks. It contains
the following data.
- The parent check (the first 20 bytes of the block's parent hash).
- The l1 origin check (the first 20 bytes of the last block's l1 origin hash).
- The genesis timestamp.
- The chain id.
- A list of [`SpanBatchElement`][span-batch-element]s. These are similar to
  the [`SingleBatch`][single-batch] type but don't contain the parent hash
  and epoch hash for this L2 block.
- Origin bits.
- Block transaction counts.
- Span batch transactions which contain information for transactions in a span batch.

Similar to the `SingleBatch` type discussed above, the [`SpanBatch`][span-batch] type
must be validated once decoded. For this, the [`SpanBatch::check_batch`][check-batch-span]
method is available.

After the [Holocene hardfork][holocene] was introduced, span batch validation is greatly
simplified to be forwards-invalidating instead of backwards-invalidating, so a new
[`SpanBatch::check_batch_prefix`][check-batch-prefix] method provides a way to validate
each batch as it is loaded, in an iterative fashion.


## Batch Encoding

The first byte of the decompressed channel data is the
[`BatchType`][batch-type], which identifies whether the batch is a
[`SingleBatch`][single-batch] or a [`SpanBatch`][span-batch].
From there, the respective type is decoded, and [derived][derived]
in the case of the [`SpanBatch`][span-batch].

The `Batch` encoding format for the [`SingleBatch`][single-batch] is
broken down [in the specs][specs].


## The `Batch` Type

The [`Batch`][batch] type itself only provides two useful methods.
- [`timestamp`][timestamp] returns the timestamp of the [`Batch`][batch]
- [`deocde`][decode], constructs a new [`Batch`][batch] from the provided
  raw, decompressed batch data and rollup config.

Within each [`Batch`][batch] variant, the individual types contain
more functionality.


<!-- Links -->

[holocene]: https://specs.optimism.io/protocol/holocene/overview.html
[check-batch-prefix]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.SpanBatch.html#method.check_batch_prefix
[check-batch-span]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.SpanBatch.html#method.check_batch
[span-batch-element]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.SpanBatchElement.html
[check-batch-single]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.SingleBatch.html#method.check_batch

[bytes]: https://docs.rs/alloy-primitives/latest/alloy_primitives/struct.Bytes.html

[block-hash]: https://docs.rs/alloy-primitives/latest/alloy_primitives/aliases/type.BlockHash.html
[epoch]: https://specs.optimism.io/glossary.html?highlight=Epoch#sequencing-epoch

[decode]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/enum.Batch.html#method.decode
[timestamp]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/enum.Batch.html#method.timestamp

[specs]: https://specs.optimism.io/protocol/derivation.html#batch-format

[derived]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.RawSpanBatch.html#method.derive
[batch-type]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/enum.BatchType.html
[channel]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Channel.html
[batch]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/enum.Batch.html
[span-batch]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.SpanBatch.html
[single-batch]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.SingleBatch.html

[delta]: https://specs.optimism.io/protocol/delta/overview.html
