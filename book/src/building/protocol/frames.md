# Frames

[`Frame`][frame]s are the lowest level data format in the OP Stack protocol.


## Where Frames fit in the OP Stack

Transactions posted to the data availability layer of the rollup
contain one or multiple [Frame][frame]s. Frames are chunks of raw data that
belong to a given [Channel][channel], the next, higher up data format in the
OP Stack protocol. Importantly, a given transaction can contain
a variety of frames from _different_ channels, allowing maximum flexibility
when breaking up channels into batcher transactions.


## Contents of a Frame

A [Frame][frame] is comprised of the following items.
- A [`ChannelId`][cid] which is a 16 byte long identifier for the channel that
  the given frame belongs to.
- A `number` that identifies the index of the frame within the channel. Frames
  are 0-indexed and are bound to `u16` size limit.
- `data` contains the raw data within the frame.
- `is_last` marks if the frame is the last within the channel.


## Frame Encoding

When frames are posted through a batcher transaction, they are encoded as a
contiguous list with a single byte prefix denoting the derivation version.
The encoding can be represented as the following concatenated bytes.

```
encoded = DERIVATION_VERSION_0 ++ encoded_frame_0 ++ encoded_frame_1 ++ ..
```

Where `DERIVATION_VERSION_0` is a single byte (`0x00`) indicating the derivation
version including how the frames are encoded. Currently, the only supported
derivation version is `0`.


`encoded_frame_0`, `encoded_frame_1`, and so on, are all [`Frame`][frame]s encoded
as raw bytes. A single encoded [`Frame`][frame] can be represented by the following
concatenation of it's fields.

```
encoded_frame = channel_id ++ frame_number ++ frame_data_length ++ frame_data ++ is_last
```

Where `++` represents concatenation. The frame's fields map to it's encoding.
- `channel_id` is the 16 byte long [`Frame::id`][id].
- `frame_number` is the 2 byte long (or `u16`) [`Frame::number`][number].
- `frame_data_length` and `frame_data` provide the necessary details to decode
  the [`Frame::data`][data], where `frame_data_length` is 4 bytes long (or `u32`).
- `is_last` is a single byte [`Frame::is_last`][is_last].


## op-alloy's `Frame` Type

[`op-alloy-protocol`][protocol] provides the [`Frame`][frame] type with a few useful
methods. [`Frame`][frame]s can be encoded and decoded using the [`Frame::encode`][encode]
and [`Frame::decode`][decode] methods. Given the raw batcher transaction data or blob data
containing the concatenated derivation version and contiguous list of encoded frames,
the [`Frame::parse_frame`][parse_frame] and [`Frame::parse_frames`][parse_frames] methods
provide ways to decode single and multiple frames, respectively.


<!-- Links -->

[encode]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Frame.html#method.encode
[decode]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Frame.html#method.decode

[parse_frame]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Frame.html#method.parse_frame
[parse_frames]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Frame.html#method.parse_frames

[protocol]: https://crates.io/crate/op-alloy-protocol

[id]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Frame.html#structfield.id
[number]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Frame.html#structfield.number
[data]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Frame.html#structfield.data
[is_last]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Frame.html#structfield.is_last

[cid]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/type.ChannelId.html
[channel]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Channel.html
[frame]: https://docs.rs/op-alloy-protocol/latest/op_alloy_protocol/struct.Frame.html
