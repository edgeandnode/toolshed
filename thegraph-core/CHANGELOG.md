# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.14.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.12.1...thegraph-core-v0.14.0) - 2025-04-17

### Other

- *(deps)* update rust crate alloy to 0.14 ([#523](https://github.com/edgeandnode/toolshed/pull/523))
- set Rust edition to 2024 ([#519](https://github.com/edgeandnode/toolshed/pull/519))

## [0.12.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.12.0...thegraph-core-v0.12.1) - 2025-04-17

### Fixed

- impl Eq for Attestation ([#533](https://github.com/edgeandnode/toolshed/pull/533))

### Other

- *(deps)* update dependency rust to v1.86.0 ([#525](https://github.com/edgeandnode/toolshed/pull/525))

## [0.12.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.11.0...thegraph-core-v0.12.0) - 2025-03-19

### Fixed

- *(alloy)* replace rng.gen() with rng.random() to fix the deprecation warning ([#503](https://github.com/edgeandnode/toolshed/pull/503))

### Other

- *(deps)* update rust crate alloy to 0.12 ([#486](https://github.com/edgeandnode/toolshed/pull/486))
- *(deps)* update rust crate fake to v4 ([#491](https://github.com/edgeandnode/toolshed/pull/491))

## [0.11.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.10.0...thegraph-core-v0.11.0) - 2025-01-27

### Added

- *(thegraph-core)* add fake_impl module and alloy fake support (#469)

### Other

- *(deps)* update rust crate alloy to 0.9 (#439)
- *(deps)* update rust crate thiserror to v2 (#467)

## [0.10.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.10.0...thegraph-core-v0.10.0) - 2025-01-09

### Other

- *(thegraph-core)* update rust crate alloy v0.8 (#465)

## [0.9.6](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.9.5...thegraph-core-v0.9.6) - 2024-12-19

### Added

- *(thegraph-core)* re-export various alloy crate features (#452)

## [0.9.5](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.9.4...thegraph-core-v0.9.5) - 2024-12-13

### Fixed

- *(thegraph-core)* reexport signed message additional types (#448)

## [0.9.4](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.9.3...thegraph-core-v0.9.4) - 2024-12-13

### Added

- *(thegraph-core)* add more POI conversion trait implementations (#446)

## [0.9.3](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.9.2...thegraph-core-v0.9.3) - 2024-12-11

### Added

- *(thegraph-core)* add EIP-712 signed message module (#338)

## [0.9.2](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.9.1...thegraph-core-v0.9.2) - 2024-12-11

### Added

- *(thegraph-core)* add more conversion trait implementations (#441)

## [0.9.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.9.0...thegraph-core-v0.9.1) - 2024-12-09

### Added

- *(thegraph-core)* implement fake trait for attestation type (#416)

### Other

- *(thegraph-core)* update msrv to 1.81.0 (#423)

## [0.9.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.8.5...thegraph-core-v0.9.0) - 2024-11-29

### Added

- *(thegraph-core)* add fake-rs support ([#403](https://github.com/edgeandnode/toolshed/pull/403))
- *(thegraph-core)* remove attestation feature from defaults set ([#397](https://github.com/edgeandnode/toolshed/pull/397))
- *(thegraph-core)* remove deprecated client module ([#396](https://github.com/edgeandnode/toolshed/pull/396))

### Other

- *(thegraph-core)* rename async-graphql support feature ([#404](https://github.com/edgeandnode/toolshed/pull/404))
- *(deps)* update rust crate alloy to 0.7 ([#400](https://github.com/edgeandnode/toolshed/pull/400))

## [0.8.5](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.8.4...thegraph-core-v0.8.5) - 2024-11-27

### Added

- *(thegraph-core)* gate the attestation module behind a crate feature ([#394](https://github.com/edgeandnode/toolshed/pull/394))

## [0.8.4](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.8.3...thegraph-core-v0.8.4) - 2024-11-26

### Added

- *(thegraph-core)* add alloy-full feature ([#387](https://github.com/edgeandnode/toolshed/pull/387))
- *(thegraph-core)* add alloy-contract feature ([#386](https://github.com/edgeandnode/toolshed/pull/386))

## [0.8.3](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.8.2...thegraph-core-v0.8.3) - 2024-11-26

### Added

- *(thegraph-core)* add alloy-rlp feature ([#384](https://github.com/edgeandnode/toolshed/pull/384))

## [0.8.2](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.8.1...thegraph-core-v0.8.2) - 2024-11-11

### Fixed

- *(thegraph-core)* remove explicit dependency on alloy-sol-types ([#374](https://github.com/edgeandnode/toolshed/pull/374))

## [0.8.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.8.0...thegraph-core-v0.8.1) - 2024-11-09

### Added

- *(thegraph-core)* set crate msrv to 1.79.0 ([#370](https://github.com/edgeandnode/toolshed/pull/370))

### Other

- *(thegraph-core)* deprecate subgraph-client feature in favor of thegraph-client-subgraphs crate ([#368](https://github.com/edgeandnode/toolshed/pull/368))

## [0.8.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.7.0...thegraph-core-v0.8.0) - 2024-11-09

### Added

- *(thegraph-core)* re-export alloy meta-crate ([#360](https://github.com/edgeandnode/toolshed/pull/360))
- *(thegraph-core)* remove serde feature from default features set ([#359](https://github.com/edgeandnode/toolshed/pull/359))

### Fixed

- pin alloy-signer ([#353](https://github.com/edgeandnode/toolshed/pull/353))

### Other

- *(thegraph-core)* improve crate docs.rs docs ([#363](https://github.com/edgeandnode/toolshed/pull/363))
- *(thegraph-core)* extend subgraph client integration tests timeout to 30s ([#362](https://github.com/edgeandnode/toolshed/pull/362))
- *(thegraph-core)* update crate readme ([#361](https://github.com/edgeandnode/toolshed/pull/361))
- *(thegraph-core)* update crate readme ([#357](https://github.com/edgeandnode/toolshed/pull/357))
- *(deps)* update alloy-rs crates to 0.5 ([#336](https://github.com/edgeandnode/toolshed/pull/336))

## [0.7.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.6.1...thegraph-core-v0.7.0) - 2024-10-02

### Other

- *(deps)* update alloy-rs crates ([#294](https://github.com/edgeandnode/toolshed/pull/294))

## [0.6.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.6.0...thegraph-core-v0.6.1) - 2024-10-02

### Other

- *(deps)* update rust crate test-with to 0.14.0 ([#314](https://github.com/edgeandnode/toolshed/pull/314))

## [0.6.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.7...thegraph-core-v0.6.0) - 2024-08-19

### Added
- *(thegraph-core)* implement serde traits for poi type ([#303](https://github.com/edgeandnode/toolshed/pull/303))
- *(thegraph-core)* flatten types module ([#282](https://github.com/edgeandnode/toolshed/pull/282))
- *(thegraph-core)* migrate to alloy-rs ([#268](https://github.com/edgeandnode/toolshed/pull/268))
- *(thegraph-core)* add proof of indexing new-type ([#269](https://github.com/edgeandnode/toolshed/pull/269))

### Fixed
- serialize SubgraphId with leading zero bytes

### Other
- *(ci)* remove L1 test environment configuration ([#298](https://github.com/edgeandnode/toolshed/pull/298))
- *(docs)* fix clippy::doc_lazy_continuation error ([#283](https://github.com/edgeandnode/toolshed/pull/283))
- *(deps)* update alloy-rs crates to 0.2 ([#280](https://github.com/edgeandnode/toolshed/pull/280))

## [0.5.7](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.6...thegraph-core-v0.5.7) - 2024-07-23

### Fixed
- *(thegraph-core)* fix allocation and indexer ID fmt implementations ([#272](https://github.com/edgeandnode/toolshed/pull/272))

## [0.5.6](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.5...thegraph-core-v0.5.6) - 2024-07-15

### Added
- *(thegraph-core)* make serde dependencies optional ([#258](https://github.com/edgeandnode/toolshed/pull/258))
- *(thegraph-core)* remove lazy_static dependency ([#257](https://github.com/edgeandnode/toolshed/pull/257))
- *(thegraph-core)* add const support for ID macros ([#256](https://github.com/edgeandnode/toolshed/pull/256))
- *(thegraph-core)* add deployment ID creation macro ([#254](https://github.com/edgeandnode/toolshed/pull/254))

### Other
- *(thegraph-core)* use zero const in subgraph and deployment id macros ([#260](https://github.com/edgeandnode/toolshed/pull/260))
- *(thegraph-core)* add attestation module docs ([#259](https://github.com/edgeandnode/toolshed/pull/259))

## [0.5.5](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.4...thegraph-core-v0.5.5) - 2024-07-12

### Fixed
- *(thegraph-core)* add subgraph ID base58 parsing macro ([#252](https://github.com/edgeandnode/toolshed/pull/252))

## [0.5.4](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.3...thegraph-core-v0.5.4) - 2024-07-12

### Added
- *(thegraph-core)* add allocation, indexer and subgraph ID creation macros ([#251](https://github.com/edgeandnode/toolshed/pull/251))

### Other
- *(deps)* update rust crate test-with to 0.13.0 ([#249](https://github.com/edgeandnode/toolshed/pull/249))

## [0.5.3](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.2...thegraph-core-v0.5.3) - 2024-06-24

### Added
- *(thegraph-core)* add allocation ID and indexer ID new-types ([#237](https://github.com/edgeandnode/toolshed/pull/237))

## [0.5.2](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.1...thegraph-core-v0.5.2) - 2024-06-17

### Fixed
- *(thegraph-core)* return empty vec on empty results response ([#233](https://github.com/edgeandnode/toolshed/pull/233))
- *(thegraph-core)* subgraph_client future does not implement Send trait ([#231](https://github.com/edgeandnode/toolshed/pull/231))

## [0.5.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.0...thegraph-core-v0.5.1) - 2024-06-03

### Fixed
- *(thegraph-core)* add missing dependency for default-features ([#221](https://github.com/edgeandnode/toolshed/pull/221))

### Other
- fix code format issues ([#216](https://github.com/edgeandnode/toolshed/pull/216))

## [0.5.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.4.3...thegraph-core-v0.5.0) - 2024-05-30

### Added
- *(thegraph-core)* report subgraph client errors ([#213](https://github.com/edgeandnode/toolshed/pull/213))

### Other
- *(thegraph-core)* remove deprecated subscriptions module ([#211](https://github.com/edgeandnode/toolshed/pull/211))

## [0.4.3](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.4.2...thegraph-core-v0.4.3) - 2024-05-29

### Other
- *(thegraph-core)* mark subscriptions  module as deprecated ([#207](https://github.com/edgeandnode/toolshed/pull/207))

## [0.4.2](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.4.1...thegraph-core-v0.4.2) - 2024-05-24

### Added
- *(thegraph-core)* make subgraph client cloneable ([#201](https://github.com/edgeandnode/toolshed/pull/201))

## [0.4.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.4.0...thegraph-core-v0.4.1) - 2024-05-09

### Fixed
- *(subgraph_client)* avoid panic on missing query block ([#186](https://github.com/edgeandnode/toolshed/pull/186))

### Other
- *(ci)* update the encrypted test credentials env file ([#185](https://github.com/edgeandnode/toolshed/pull/185))

## [0.4.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.3.2...thegraph-core-v0.4.0) - 2024-05-06

### Added
- set page size per query batch
- set block height for paginated queries

## [0.3.2](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.3.1...thegraph-core-v0.3.2) - 2024-05-01

### Fixed
- *(subgraph_client)* fail fast on reorg detected ([#172](https://github.com/edgeandnode/toolshed/pull/172))

### Other
- *(deps)* update rust crate serde_with to 3.8 ([#166](https://github.com/edgeandnode/toolshed/pull/166))
- *(deps)* update rust crate alloy-chains to 0.1.17 ([#165](https://github.com/edgeandnode/toolshed/pull/165))

## [0.3.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.3.0...thegraph-core-v0.3.1) - 2024-04-22

### Other
- *(deps)* update rust crate serde_json to 1.0.116 ([#159](https://github.com/edgeandnode/toolshed/pull/159))
- *(deps)* update rust crate alloy-chains to 0.1.16 ([#155](https://github.com/edgeandnode/toolshed/pull/155))

## [0.3.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.2.3...thegraph-core-v0.3.0) - 2024-04-10

### Other
- *(deps)* update rust crate reqwest to 0.12.3 ([#144](https://github.com/edgeandnode/toolshed/pull/144))

## [0.2.3](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.2.2...thegraph-core-v0.2.3) - 2024-04-10

### Other
- *(deps)* update alloy-rs core types monorepo to 0.7 ([#151](https://github.com/edgeandnode/toolshed/pull/151))
- *(deps)* update rust crate indoc to 2.0.5 ([#146](https://github.com/edgeandnode/toolshed/pull/146))
- *(deps)* update rust crate tokio to v1.37.0 ([#150](https://github.com/edgeandnode/toolshed/pull/150))
- *(deps)* update rust crate serde_json to 1.0.115
- *(deps)* update rust crate reqwest to 0.11.27 ([#143](https://github.com/edgeandnode/toolshed/pull/143))
- *(deps)* update rust crate reqwest to 0.11.26 ([#140](https://github.com/edgeandnode/toolshed/pull/140))
- *(deps)* update rust crate serde_with to 3.7 ([#138](https://github.com/edgeandnode/toolshed/pull/138))

## [0.2.2](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.2.1...thegraph-core-v0.2.2) - 2024-03-11

### Other
- *(deps)* update rust crate reqwest to 0.11.25 ([#135](https://github.com/edgeandnode/toolshed/pull/135))
- add integration tests encrypted environment ([#132](https://github.com/edgeandnode/toolshed/pull/132))
- *(deps)* update rust crate ethers-core to 2.0.14 ([#131](https://github.com/edgeandnode/toolshed/pull/131))
- *(deps)* update rust crate ethers to 2.0.14 ([#130](https://github.com/edgeandnode/toolshed/pull/130))
- *(deps)* update rust crate test-with to 0.12.6 ([#128](https://github.com/edgeandnode/toolshed/pull/128))
- *(deps)* update rust crate alloy-chains to 0.1.15 ([#127](https://github.com/edgeandnode/toolshed/pull/127))

## [0.2.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.2.0...thegraph-core-v0.2.1) - 2024-03-04

### Added
- *(thegraph-core)* add subscriptions auth support ([#124](https://github.com/edgeandnode/toolshed/pull/124))

## [0.1.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.1.0...thegraph-core-v0.1.1) - 2024-03-04

### Added
- *(thegraph-graphql-http)* add thegraph-graphql-http crate ([#120](https://github.com/edgeandnode/toolshed/pull/120))
