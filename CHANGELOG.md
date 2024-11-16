# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.6](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.6.6) - 2024-11-16

### Bug Fixes

- Protected bits handling ([#270](https://github.com/alloy-rs/op-alloy/issues/270))
- [book] Batch over SingleBatch ([#260](https://github.com/alloy-rs/op-alloy/issues/260))
- [book] Getting Start Links ([#256](https://github.com/alloy-rs/op-alloy/issues/256))
- [book] Broken Mdbook Version ([#250](https://github.com/alloy-rs/op-alloy/issues/250))

### Features

- [protocol] Brotli Compression behind `std` ([#263](https://github.com/alloy-rs/op-alloy/issues/263))
- [protocol] Batch Encoding ([#259](https://github.com/alloy-rs/op-alloy/issues/259))
- Add missing OpTxType trait impls ([#258](https://github.com/alloy-rs/op-alloy/issues/258))
- [book] Frames ([#226](https://github.com/alloy-rs/op-alloy/issues/226))
- [book] Add Badges for Crates ([#253](https://github.com/alloy-rs/op-alloy/issues/253))

### Miscellaneous Tasks

- [protocol] Re-organizes Modules and Errors ([#261](https://github.com/alloy-rs/op-alloy/issues/261))
- [book] Building Docs ([#257](https://github.com/alloy-rs/op-alloy/issues/257))
- [book] Frames to Batches Example ([#232](https://github.com/alloy-rs/op-alloy/issues/232))
- [book] Missing Sections and Enhancements ([#255](https://github.com/alloy-rs/op-alloy/issues/255))
- [book] Touchup Introduction ([#254](https://github.com/alloy-rs/op-alloy/issues/254))

## [0.6.5](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.6.5) - 2024-11-12

### Dependencies

- Bump alloy 064 ([#249](https://github.com/alloy-rs/op-alloy/issues/249))

### Features

- Wrap `TxDeposit` into `Sealed` in `OpTxEnvelope` ([#247](https://github.com/alloy-rs/op-alloy/issues/247))
- Add nonce to RPC transaction ([#246](https://github.com/alloy-rs/op-alloy/issues/246))

### Miscellaneous Tasks

- Release 0.6.5
- Add deserde test ([#248](https://github.com/alloy-rs/op-alloy/issues/248))

## [0.6.4](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.6.4) - 2024-11-12

### Bug Fixes

- [consensus] Add conversion for `OpTxType::Eip7702` ([#244](https://github.com/alloy-rs/op-alloy/issues/244))
- [consensus] Fix arbitrary impl for `OpTxType` ([#242](https://github.com/alloy-rs/op-alloy/issues/242))

### Miscellaneous Tasks

- Release 0.6.4
- Add is dynamic fee ([#245](https://github.com/alloy-rs/op-alloy/issues/245))

## [0.6.3](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.6.3) - 2024-11-08

### Dependencies

- Bump Alloy Deps ([#239](https://github.com/alloy-rs/op-alloy/issues/239))

### Features

- Bump alloy ([#240](https://github.com/alloy-rs/op-alloy/issues/240))

### Miscellaneous Tasks

- Release 0.6.3

## [0.6.2](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.6.2) - 2024-11-06

### Bug Fixes

- [protocol] Batch Decoding ([#235](https://github.com/alloy-rs/op-alloy/issues/235))
- [book] Links Imports ([#227](https://github.com/alloy-rs/op-alloy/issues/227))

### Features

- Add fn for decoded 1559 params ([#236](https://github.com/alloy-rs/op-alloy/issues/236))
- [book] Engine RPC Types ([#229](https://github.com/alloy-rs/op-alloy/issues/229))

### Miscellaneous Tasks

- Release 0.6.2
- Move eip1559 impls ([#237](https://github.com/alloy-rs/op-alloy/issues/237))
- [rpc-types] Clean up Exports ([#231](https://github.com/alloy-rs/op-alloy/issues/231))
- [book] Consolidate Links ([#230](https://github.com/alloy-rs/op-alloy/issues/230))
- [book] RPC Types ([#228](https://github.com/alloy-rs/op-alloy/issues/228))
- [book] Protocol Docs ([#225](https://github.com/alloy-rs/op-alloy/issues/225))

### Other

- V0.6.1 ([#238](https://github.com/alloy-rs/op-alloy/issues/238))

## [0.6.0](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.6.0) - 2024-11-06

### Bug Fixes

- [book] Small Book Touchups ([#220](https://github.com/alloy-rs/op-alloy/issues/220))
- [ci] Remove Docs gh-page publish ([#216](https://github.com/alloy-rs/op-alloy/issues/216))
- Ci powerset ([#214](https://github.com/alloy-rs/op-alloy/issues/214))
- [book] Missing READMEs ([#213](https://github.com/alloy-rs/op-alloy/issues/213))

### Dependencies

- [wip] feat: bump alloy ([#205](https://github.com/alloy-rs/op-alloy/issues/205))
- [workspace] Import Touchups ([#199](https://github.com/alloy-rs/op-alloy/issues/199))
- Bump alloy ([#178](https://github.com/alloy-rs/op-alloy/issues/178))

### Features

- Add holocene extradata fn ([#233](https://github.com/alloy-rs/op-alloy/issues/233))
- Add jsonrpsee trait for SuperchainSignal ([#217](https://github.com/alloy-rs/op-alloy/issues/217))
- `OpTransactionRequest` ([#215](https://github.com/alloy-rs/op-alloy/issues/215))
- [book] Consensus ([#212](https://github.com/alloy-rs/op-alloy/issues/212))
- [book] Genesis - System Config ([#211](https://github.com/alloy-rs/op-alloy/issues/211))
- `op-alloy` meta crate ([#210](https://github.com/alloy-rs/op-alloy/issues/210))
- [book] Genesis - Rollup Config ([#209](https://github.com/alloy-rs/op-alloy/issues/209))
- Book Setup ([#208](https://github.com/alloy-rs/op-alloy/issues/208))
- README ([#207](https://github.com/alloy-rs/op-alloy/issues/207))
- Book ([#206](https://github.com/alloy-rs/op-alloy/issues/206))
- [protocol] Batch ([#200](https://github.com/alloy-rs/op-alloy/issues/200))
- [protocol] Span Batch Validity Checks ([#198](https://github.com/alloy-rs/op-alloy/issues/198))
- [protocol] Span Batch Type ([#197](https://github.com/alloy-rs/op-alloy/issues/197))
- [protocol] Span Batch Transactions ([#196](https://github.com/alloy-rs/op-alloy/issues/196))
- [protocol] Batch TX Data ([#195](https://github.com/alloy-rs/op-alloy/issues/195))
- [protocol] Span Batch Bits ([#194](https://github.com/alloy-rs/op-alloy/issues/194))
- [protocol] Span Batch Element ([#193](https://github.com/alloy-rs/op-alloy/issues/193))
- [protocol] Batch Utilities ([#191](https://github.com/alloy-rs/op-alloy/issues/191))
- [protocol] Batch Error Types ([#190](https://github.com/alloy-rs/op-alloy/issues/190))
- [protocol] BatchValidationProvider ([#189](https://github.com/alloy-rs/op-alloy/issues/189))
- [protocol] SingleBatch Type ([#188](https://github.com/alloy-rs/op-alloy/issues/188))
- [protocol] Batch Validity ([#187](https://github.com/alloy-rs/op-alloy/issues/187))
- [protocol] Batch Type ([#186](https://github.com/alloy-rs/op-alloy/issues/186))
- [rpc-types] `{Try}From` impl for `OpTransactionReceipt` + `Transaction` -> consensus types ([#183](https://github.com/alloy-rs/op-alloy/issues/183))
- [genesis] EIP 1559 System Config Accessor ([#179](https://github.com/alloy-rs/op-alloy/issues/179))

### Miscellaneous Tasks

- Release 0.6.0
- [book] Load Rollup Config Example ([#224](https://github.com/alloy-rs/op-alloy/issues/224))
- [book] Genesis Docs ([#223](https://github.com/alloy-rs/op-alloy/issues/223))
- [book] Consensus Docs ([#222](https://github.com/alloy-rs/op-alloy/issues/222))
- [ci] Use Justfile Targets in Github Actions ([#219](https://github.com/alloy-rs/op-alloy/issues/219))
- [book] Fix Doc Links ([#218](https://github.com/alloy-rs/op-alloy/issues/218))
- Release 0.5.2 ([#201](https://github.com/alloy-rs/op-alloy/issues/201))
- [consensus] Upstream Receipt Constructor ([#165](https://github.com/alloy-rs/op-alloy/issues/165))
- Release 0.5.1 ([#184](https://github.com/alloy-rs/op-alloy/issues/184))
- [consensus] Small Cleanup ([#180](https://github.com/alloy-rs/op-alloy/issues/180))
- Dependency Updates ([#177](https://github.com/alloy-rs/op-alloy/issues/177))

### Other

- Add arbitrary attr ([#182](https://github.com/alloy-rs/op-alloy/issues/182))

## [0.5.0](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.5.0) - 2024-10-18

### Dependencies

- Bump alloy and remove `OpExecutionPayloadV4` ([#176](https://github.com/alloy-rs/op-alloy/issues/176))

### Features

- Add signature function to TxDeposit ([#174](https://github.com/alloy-rs/op-alloy/issues/174))
- Add depositTransaction trait ([#171](https://github.com/alloy-rs/op-alloy/issues/171))
- Op network execution payload envelope decoding ([#149](https://github.com/alloy-rs/op-alloy/issues/149))
- [rollup] Backward-activate forks in `RollupConfig` ([#170](https://github.com/alloy-rs/op-alloy/issues/170))
- [envelope] Add missing `From<Signed<TxEip7702>>` ([#168](https://github.com/alloy-rs/op-alloy/issues/168))

### Miscellaneous Tasks

- Release 0.5.0

## [0.4.0](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.4.0) - 2024-10-09

### Bug Fixes

- Alloy Updates ([#166](https://github.com/alloy-rs/op-alloy/issues/166))
- Op Prefix ([#164](https://github.com/alloy-rs/op-alloy/issues/164))
- [genesis] Op Prefix Naming Convention ([#161](https://github.com/alloy-rs/op-alloy/issues/161))
- [rpc-types-engine] Op Prefix Naming Convention ([#163](https://github.com/alloy-rs/op-alloy/issues/163))
- [rpc-types] Op Prefix Naming Convention ([#162](https://github.com/alloy-rs/op-alloy/issues/162))
- Elide Lifetimes ([#160](https://github.com/alloy-rs/op-alloy/issues/160))
- Safeheadresponse field types ([#156](https://github.com/alloy-rs/op-alloy/issues/156))
- Genesis l1 l2 field types ([#157](https://github.com/alloy-rs/op-alloy/issues/157))
- Remove 4844 transaction type ([#151](https://github.com/alloy-rs/op-alloy/issues/151))
- Reverts 13d0c2 - impl SignableTransaction for Deposit ([#153](https://github.com/alloy-rs/op-alloy/issues/153))
- [genesis] BaseFeeParams Arbitrary Bounds ([#147](https://github.com/alloy-rs/op-alloy/issues/147))

### Features

- Add 7702 ([#167](https://github.com/alloy-rs/op-alloy/issues/167))
- [consensus] Transaction for OpTxEnvelope ([#159](https://github.com/alloy-rs/op-alloy/issues/159))
- [consensus] System Transaction ([#154](https://github.com/alloy-rs/op-alloy/issues/154))
- [`consensus`] Impl `SignableTx` for `TxDeposit` ([#152](https://github.com/alloy-rs/op-alloy/issues/152))
- Codeowner Updates ([#148](https://github.com/alloy-rs/op-alloy/issues/148))
- [protocol] Arbitrary Block Info Types ([#145](https://github.com/alloy-rs/op-alloy/issues/145))
- [genesis] Arbitrary Support ([#144](https://github.com/alloy-rs/op-alloy/issues/144))
- [protocol] Add Frame Iterator ([#141](https://github.com/alloy-rs/op-alloy/issues/141))
- Justfile for my sanity ([#142](https://github.com/alloy-rs/op-alloy/issues/142))
- [rpc-types-engine] EIP-1559 parameters in `OptimismPayloadAttributes` ([#138](https://github.com/alloy-rs/op-alloy/issues/138))
- [genesis] `SystemConfig` holocene updates ([#139](https://github.com/alloy-rs/op-alloy/issues/139))
- [protocol] SystemConfig Conversion Utility ([#135](https://github.com/alloy-rs/op-alloy/issues/135))

### Miscellaneous Tasks

- Release 0.4.0
- Cleanup Arbitrary Tests ([#146](https://github.com/alloy-rs/op-alloy/issues/146))
- Cleanup Workspace Manifest ([#143](https://github.com/alloy-rs/op-alloy/issues/143))
- V0.3.3 ([#140](https://github.com/alloy-rs/op-alloy/issues/140))
- Cleanup Workspace Documentation ([#129](https://github.com/alloy-rs/op-alloy/issues/129))
- [protocol] Remove `L1BlockInfoTx::Holocene` variant ([#137](https://github.com/alloy-rs/op-alloy/issues/137))
- [protocol] Payload Conversion Utilities ([#136](https://github.com/alloy-rs/op-alloy/issues/136))

### Other

- Adding fee computation functions to l1BlockInfoTx ([#134](https://github.com/alloy-rs/op-alloy/issues/134))

## [0.3.2](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.3.2) - 2024-09-30

### Features

- [consensus] Bincode compatibility ([#131](https://github.com/alloy-rs/op-alloy/issues/131))

### Miscellaneous Tasks

- Release 0.3.2 ([#133](https://github.com/alloy-rs/op-alloy/issues/133))
- [genesis] Small README Update ([#128](https://github.com/alloy-rs/op-alloy/issues/128))

## [0.3.1](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.3.1) - 2024-09-30

### Bug Fixes

- HashMap default

### Miscellaneous Tasks

- Release 0.3.1

## [0.3.0](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.3.0) - 2024-09-30

### Bug Fixes

- Clean up protocol std feat flagging ([#119](https://github.com/alloy-rs/op-alloy/issues/119))
- [protocol] Functional Batch Transaction ([#88](https://github.com/alloy-rs/op-alloy/issues/88))
- L1Origin -> l1origin during deser of L2BlockRef ([#116](https://github.com/alloy-rs/op-alloy/issues/116))
- [engine] Missing Error Source ([#114](https://github.com/alloy-rs/op-alloy/issues/114))

### Dependencies

- Bump alloy 0.4 ([#127](https://github.com/alloy-rs/op-alloy/issues/127))
- Use alloy map ([#126](https://github.com/alloy-rs/op-alloy/issues/126))

### Features

- [consensus] OpBlock Type ([#105](https://github.com/alloy-rs/op-alloy/issues/105))
- [workspace] Use Workspace Level Lints ([#125](https://github.com/alloy-rs/op-alloy/issues/125))
- [genesis] Simplify Log Updates in System Config ([#123](https://github.com/alloy-rs/op-alloy/issues/123))
- [genesis] Optimism Base Fee Params ([#122](https://github.com/alloy-rs/op-alloy/issues/122))
- [protocol] Holocene Support ([#118](https://github.com/alloy-rs/op-alloy/issues/118))
- [provider] OP engine api trait ext + superchain signal type ([#117](https://github.com/alloy-rs/op-alloy/issues/117))
- [engine] Deprecate RollupConfig Argument ([#112](https://github.com/alloy-rs/op-alloy/issues/112))
- Exec payload v4 serde test ([#113](https://github.com/alloy-rs/op-alloy/issues/113))

### Miscellaneous Tasks

- Release 0.3.0
- [protocol] Cleanup block info block hash retrieval ([#120](https://github.com/alloy-rs/op-alloy/issues/120))

### Other

- Replace u8 direction field with Direction type ([#90](https://github.com/alloy-rs/op-alloy/issues/90))
- Add holocene time to genesis ([#115](https://github.com/alloy-rs/op-alloy/issues/115))

## [0.2.12](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.2.12) - 2024-09-18

### Bug Fixes

- No_std for op-alloy-rpc-types-engine ([#109](https://github.com/alloy-rs/op-alloy/issues/109))
- [protocol] Invalid Frame Data Length ([#108](https://github.com/alloy-rs/op-alloy/issues/108))

### Dependencies

- Bump alloy 0.3.6 ([#111](https://github.com/alloy-rs/op-alloy/issues/111))
- Bump msrv 1.81 ([#106](https://github.com/alloy-rs/op-alloy/issues/106))

### Features

- [engine] Payload Conversion Utilities ([#110](https://github.com/alloy-rs/op-alloy/issues/110))
- Remove the superchain primitives dependency ([#100](https://github.com/alloy-rs/op-alloy/issues/100))
- [rpc-types-engine] No_std Support ([#104](https://github.com/alloy-rs/op-alloy/issues/104))
- [rpc-types] No_std Support ([#103](https://github.com/alloy-rs/op-alloy/issues/103))
- Remove std flag over alloc ([#101](https://github.com/alloy-rs/op-alloy/issues/101))

### Miscellaneous Tasks

- Release 0.2.12
- Re-export module items ([#102](https://github.com/alloy-rs/op-alloy/issues/102))

## [0.2.11](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.2.11) - 2024-09-13

### Bug Fixes

- Remove Block ID ([#94](https://github.com/alloy-rs/op-alloy/issues/94))
- Issue Template ([#96](https://github.com/alloy-rs/op-alloy/issues/96))

### Features

- Genesis Types ([#97](https://github.com/alloy-rs/op-alloy/issues/97))
- Attributes with parent ([#95](https://github.com/alloy-rs/op-alloy/issues/95))

### Miscellaneous Tasks

- Release 0.2.11

### Other

- Make `l1_origin` in `L2BlockRef` a struct instead of an enum ([#91](https://github.com/alloy-rs/op-alloy/issues/91))

## [0.2.10](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.2.10) - 2024-09-13

### Dependencies

- Bump alloy ([#98](https://github.com/alloy-rs/op-alloy/issues/98))

### Features

- [rpc-types] Replace u8 with Connectedness Enum ([#84](https://github.com/alloy-rs/op-alloy/issues/84))
- Feat(protocol) add block information module ([#82](https://github.com/alloy-rs/op-alloy/issues/82))

### Miscellaneous Tasks

- Release 0.2.10

## [0.2.9](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.2.9) - 2024-09-09

### Bug Fixes

- Use no_std workflow ([#78](https://github.com/alloy-rs/op-alloy/issues/78))
- Alloy-protocols utils fix ([#80](https://github.com/alloy-rs/op-alloy/issues/80))
- Alloy-rs/core update ([#75](https://github.com/alloy-rs/op-alloy/issues/75))
- [protocol] Native u64 ([#73](https://github.com/alloy-rs/op-alloy/issues/73))

### Dependencies

- Bump alloy 0.3.2 ([#86](https://github.com/alloy-rs/op-alloy/issues/86))

### Documentation

- [rpc-type] Add reference to peerdump ([#83](https://github.com/alloy-rs/op-alloy/issues/83))

### Features

- [op-alloy-protocol] Add deposit module ([#81](https://github.com/alloy-rs/op-alloy/issues/81))
- Bump superchain-primitives ([#79](https://github.com/alloy-rs/op-alloy/issues/79))
- [protocol] Deposit Tx Utility ([#74](https://github.com/alloy-rs/op-alloy/issues/74))
- Feature Powerset Job ([#72](https://github.com/alloy-rs/op-alloy/issues/72))
- [protocol] Exports Frame Constants ([#71](https://github.com/alloy-rs/op-alloy/issues/71))

### Miscellaneous Tasks

- Release 0.2.9
- Cleanup depositerror ([#87](https://github.com/alloy-rs/op-alloy/issues/87))

## [0.2.8](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.2.8) - 2024-09-04

### Bug Fixes

- [docs] L1 gas used deprecated since Fjord not Ecotone ([#67](https://github.com/alloy-rs/op-alloy/issues/67))

### Dependencies

- Bump MSRV ([#66](https://github.com/alloy-rs/op-alloy/issues/66))

### Features

- [protocol] Batch Transaction ([#70](https://github.com/alloy-rs/op-alloy/issues/70))

### Miscellaneous Tasks

- Release 0.2.8

### Other

- Make decode_fields pub for TxDeposit ([#68](https://github.com/alloy-rs/op-alloy/issues/68))
- Add encode methods for `TxDeposit` ([#69](https://github.com/alloy-rs/op-alloy/issues/69))

## [0.2.7](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.2.7) - 2024-09-02

### Miscellaneous Tasks

- Release 0.2.7

### Other

- Derive arbitrary for TxDeposit ([#65](https://github.com/alloy-rs/op-alloy/issues/65))

## [0.2.6](https://github.com/alloy-rs/op-alloy
/releases/tag/v0.2.6) - 2024-09-02

### Bug Fixes

- Derive_more dep ([#63](https://github.com/alloy-rs/op-alloy/issues/63))
- [rpc] Add l1 block info to OpTransactionReceipt ([#62](https://github.com/alloy-rs/op-alloy/issues/62))

### Features

- Workflow to validate no_std Compatibility ([#64](https://github.com/alloy-rs/op-alloy/issues/64))
- [consensus] Hardfork Transaction Builders ([#55](https://github.com/alloy-rs/op-alloy/issues/55))

### Miscellaneous Tasks

- Release 0.2.6
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
