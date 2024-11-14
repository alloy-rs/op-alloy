# Channels

Taken from the [OP Stack specs][specs], [`Channel`][channel]s are a set of
sequencer batches (for any L2 blocks) compressed together.


## Where Channels fit in the OP Stack

L2 transactions are grouped into what are called [sequencer batches][seq-batch].
In order to obtain a better compression ratio when posting these L2 transactions
to the data availability layer, [sequencer batches][seq-batch] are compressed
together into what is called a [Channel][channel]. This ultimately reduces
data availability costs. As previously noted in the [Frame][frame-docs] section,
[Channel][channel]s may not "fit" in a single batcher transaction, posting the
data to the data availability layer. In order to accommodate large
[Channel][channel]s, a tertiary [Frame][frame] data type breaks the
[Channel][channel] up into multiple [Frame][frame]s where a batcher transaction
then consists of one or multiple [Frame][frame]s.


## Contents of a Channel

A [Channel][channel] is comprised of the following items.
- A [`ChannelId`][cid] which is a 16 byte long identifier for the channel.
  Notice, [Frame][frame]s also contain a [`ChannelId`][cid], which is the
  identical to this identifier, since frames "belong" to a given channel.
- A [`BlockInfo`][block-info] that marks the L1 block at which the channel
  is "opened" at.
- The estimated size of the channel (as a `usize`) used to drop the channel
  if there is a data overflow.
- A `boolean` if the channel is "closed". This indicates if the last frame
  has been buffered, and added to the channel.
- A `u16` indicating the highest frame number within the channel.
- The frame number of the last frame (where `is_last` set to `true`).
- A mapping from Frame number to the [`Frame`][frame] itself.
- A [`BlockInfo`][block-info] for highest L1 inclusion block that a frame
  was included in.


## Channel Encoding

[`Channel`][channel] encoding is even more straightforward than that of a
[`Frame`][frame]. Simply, a [`Channel`][channel] is the concatenated list
of encoded [`Frame`][frame]s.

Since each [`Frame`][frame] contains the [`ChannelId`][cid] that corresponds
to the given [`Channel`][channel], constructing a [`Channel`][channel] is as
simple as calling the [`Channel::add_frame`][add-frame] method for each of
its [`Frame`][frame]s.

Once the [`Channel`][channel] has ingested all of it's [`Frame`][frame]s,
it will be marked as "ready", with the [`Channel::is_ready`][is-ready]
method returning `true`.


## The `Channel` Type

As discussed [above](#-channel-encoding), the [`Channel`][channel] type is
expected to be populated with [`Frame`][frame]s using its
[`Channel::add_frame`][add-frame] method. Below we demonstrate constructing
a minimal [`Channel`][channel] using a few frames.

```rust
use op_alloy_protocol::{Channel, Frame};

// Construct a channel at the given L1 block.
let id = [0xee; 16];
let block = BlockInfo::default();
let mut channel = Channel::new(id, block);

// The channel will consist of 3 frames.
let frame_0 = Frame { id: [0xee; 16], number: 0, ..Default::default() };
let frame_1 = Frame { id: [0xee; 16], number: 1, ..Default::default() };
let frame_2 = Frame { id: [0xee; 16], number: 2, is_last: true, ..Default::default() };

// Add the frames to the channel.
channel.add_frame(frame_0);
channel.add_frame(frame_1);
channel.add_frame(frame_2);

// Since the last frame was ingested,
// the channel should be ready.
assert!(channel.is_ready());
```

There are a few rules when adding a [`Frame`][frame] to a [`Channel`][channel].
- The [`Frame`][frame]'s id must be the same [`ChannelId`][cid] as the [`Channel`][channel]s.
- [`Frame`][frame]s cannot be added once a [`Channel`][channel] is closed.
- [`Frame`][frame]s within a [`Channel`][channel] must have distinct numbers.

Notice, [`Frame`][frame]s can be added out-of-order so long as the [`Channel`][channel] is
still open, and the frame hasn't already been added.


<!-- Links -->

[is-ready]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Channel.html#method.is_ready
[add-frame]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Channel.html#method.add_frame

[block-info]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.BlockInfo.html

[frame-docs]: ./frames.md
[specs]: https://specs.optimism.io/protocol/derivation.html#batch-submission-wire-format
[seq-batch]: https://specs.optimism.io/glossary.html#sequencer-batch


[channel]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Channel.html
[cid]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/type.ChannelId.html
[frame]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Frame.html
