# op-alloy-network

<a href="https://crates.io/crates/op-alloy-network"><img src="https://img.shields.io/crates/v/op-alloy-network.svg" alt="op-alloy-network crate"></a>

Optimism blockchain RPC behavior abstraction.

This crate contains a simple abstraction of the RPC behavior of an
Op-stack blockchain. It is intended to be used by the Alloy client to
provide a consistent interface to the rest of the library, regardless of
changes the underlying blockchain makes to the RPC interface.
