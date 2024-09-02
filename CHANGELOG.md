# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.6](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.2.6) - 2024-09-02

### Bug Fixes

- Derive_more dep ([#63](https://github.com/alloy-rs/op-alloy/issues/63))
- [rpc] Add l1 block info to OpTransactionReceipt ([#62](https://github.com/alloy-rs/op-alloy/issues/62))

### Features

- Workflow to validate no_std Compatibility ([#64](https://github.com/alloy-rs/op-alloy/issues/64))
- [consensus] Hardfork Transaction Builders ([#55](https://github.com/alloy-rs/op-alloy/issues/55))

### Miscellaneous Tasks

- Clean up components used in the feature form ([#60](https://github.com/alloy-rs/op-alloy/issues/60))
- Remove ethers-rs contact link ([#61](https://github.com/alloy-rs/op-alloy/issues/61))

## [0.2.2](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.2.2) - 2024-08-29

### Features

- [protocol] Core Protocol Types ([#56](https://github.com/alloy-rs/op-alloy/issues/56))

### Miscellaneous Tasks

- Release 0.2.2

### Other

- Add ecotone support to `op_alloy_rpc_types::OptimismTransactionReceiptFileds` ([#58](https://github.com/alloy-rs/op-alloy/issues/58))

## [0.2.1](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.2.1) - 2024-08-28

### Bug Fixes

- Some serde fixes ([#51](https://github.com/alloy-rs/op-alloy/issues/51))

### Miscellaneous Tasks

- Release 0.2.1
- Release 0.2.1
- Add missing envelope fns ([#52](https://github.com/alloy-rs/op-alloy/issues/52))

### Other

- Add emhane to CODEOWNERS ([#50](https://github.com/alloy-rs/op-alloy/issues/50))

## [0.2.0](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.2.0) - 2024-08-28

### Bug Fixes

- [rpc] Add Missing Safe Head Endpoint ([#47](https://github.com/alloy-rs/op-alloy/issues/47))

### Dependencies

- [deps] Use latest alloy ([#45](https://github.com/alloy-rs/op-alloy/issues/45))

### Features

- Op-alloy-rpc-types-engine ([#49](https://github.com/alloy-rs/op-alloy/issues/49))
- Add other op endpoints ([#46](https://github.com/alloy-rs/op-alloy/issues/46))
- [rpc-client] Introduce rpc-jsonrpsee Crate ([#37](https://github.com/alloy-rs/op-alloy/issues/37))
- Add rollup and other config types ([#42](https://github.com/alloy-rs/op-alloy/issues/42))
- Added sync file with types from reth ([#35](https://github.com/alloy-rs/op-alloy/issues/35))
- [rpc-types] P2p net types ([#39](https://github.com/alloy-rs/op-alloy/issues/39))

### Miscellaneous Tasks

- Release 0.2.0

### Other

- Set op_alloy_rpc_types::Transaction as Optimism::TransactionResponse ([#33](https://github.com/alloy-rs/op-alloy/issues/33))

## [0.1.5](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.1.5) - 2024-08-08

### Bug Fixes

- Fix arbitrary impl for OpTxType to include deposit tx

### Miscellaneous Tasks

- Release 0.1.5
- Clippy happy ([#30](https://github.com/alloy-rs/op-alloy/issues/30))
- Codeowners
- Downgrad clippy all

### Other

- Add granite_time to OptimismGenesisInfo ([#31](https://github.com/alloy-rs/op-alloy/issues/31))
- Merge pull request [#26](https://github.com/alloy-rs/op-alloy/issues/26) from alloy-rs/matt/codeowners1
- Merge pull request [#23](https://github.com/alloy-rs/op-alloy/issues/23) from alloy-rs/emhane/op-alloy-tx-type
- Replace TxType with OpTxType in Network impl for Optimism
- Implement display for OpTxType
- Merge pull request [#25](https://github.com/alloy-rs/op-alloy/issues/25) from alloy-rs/emhane/fix-arbitrary-op-tx-ty
- Merge pull request [#24](https://github.com/alloy-rs/op-alloy/issues/24) from alloy-rs/matt/downgrade-all-clippy

## [0.1.4](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.1.4) - 2024-07-16

### Dependencies

- Bump alloy

### Miscellaneous Tasks

- Release 0.1.4

## [0.1.3](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.1.3) - 2024-07-13

### Bug Fixes

- Op alloy rpc tx receipt

### Miscellaneous Tasks

- Release 0.1.3
- Use serde::quantity
- Rename mod

### Other

- Merge pull request [#21](https://github.com/alloy-rs/op-alloy/issues/21) from alloy-rs/matt/op-alloy-rpc-receipt
- Merge pull request [#20](https://github.com/alloy-rs/op-alloy/issues/20) from alloy-rs/matt/use-serde-quantity
- Merge pull request [#19](https://github.com/alloy-rs/op-alloy/issues/19) from alloy-rs/matt/rename-mod

## [0.1.2](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.1.2) - 2024-07-08

### Miscellaneous Tasks

- Release 0.1.2
- Update alloy
- Update changelog

## [0.1.1](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.1.1) - 2024-07-03

### Bug Fixes

- Cliff typo
- Fix test
- Fix identifier
- Fix feature
- U128 conversion
- Doc comments
- Receipt type name and flattening
- Receipt trait
- Receipt.rs imports are fixed.

### Dependencies

- Bump alloy version
- Bump alloy version.
- Add serde and alloy_primitives to the dependencies

### Documentation

- Remove outdated documentation.

### Features

- Extract optimism genesis info
- Add genesis types
- Add OP network
- [op-consensus] Trim and complete OP modifications
- [op-consensus] Add optimism deposit tx type
- [consensus] Op-consensus
- [consensus] Op-consensus
- Use generics, remove unnecessary types.
- Review changes.
- Re-export all eth types.
- Add filters.rs
- Fix imports, add TODO comments, organize the code.
- Add pubsub.rs
- Add call.rs and update visibility of transaction requests, types, and receipts.
- Add op-consensus and receiptEnvelope
- Add transaction, and request types. Adjust block to use the crate's transaction and alloy's header.
- Add txType as a separate file under transactions and update receipt.rs accordingly.
- Add block.
- Add txtype, deposit nonce, and receipt version.
- Add log
- Add transaction receipt type without tests + several dependencies.

### Miscellaneous Tasks

- Release 0.1.1
- Add cliff support
- Use alloy from crates
- Rename crate
- Rename crates

### Other

- Merge pull request [#17](https://github.com/alloy-rs/op-alloy/issues/17) from Vid201/feat/op_genesis
- Merge pull request [#16](https://github.com/alloy-rs/op-alloy/issues/16) from alloy-rs/matt/add-genesis-types
- Merge pull request [#15](https://github.com/alloy-rs/op-alloy/issues/15) from alloy-rs/matt/add-cliff-support
- Merge pull request [#14](https://github.com/alloy-rs/op-alloy/issues/14) from alloy-rs/matt/alloy-crates
- Merge pull request [#12](https://github.com/alloy-rs/op-alloy/issues/12) from alloy-rs/matt/add-network-crates
- Exclude wasm
- Merge pull request [#11](https://github.com/alloy-rs/op-alloy/issues/11) from alloy-rs/matt/rename-crates
- Merge pull request [#8](https://github.com/alloy-rs/op-alloy/issues/8) from alloy-rs/feat/op-alloy-consensus
- Reuse exiting receipt
- Make it compile
- Cleanup tx type
- Cleanup tx type
- Inherit `TxReceipt` trait
- Use upstream alloy
- `deposit` fn in `OpTypedTransaction`
- Use upstream Ethereum transaction types from `alloy-consensus`
- Add deposit receipt roundtrip RLP tests
- Use upstreamed `Signed` + `SignableTransaction`
- Merge pull request [#7](https://github.com/alloy-rs/op-alloy/issues/7) from alloy-rs/matt/add-default
- Add missing default
- Merge pull request [#6](https://github.com/alloy-rs/op-alloy/issues/6) from alloy-rs/matt/add-tx-rpc-type
- Allow
- Allow git
- Some cleanup
- Initial commit

### Refactor

- Use native types
- Re-import instead of redefining.
- Update optimism specific fields and their (de)serialization methods in receipt.rs

### Styling

- Fmt
- Cargo fmt
- Cargo fmt.

<!-- generated by git-cliff -->
