# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.8](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.7...thegraph-core-v0.5.8) - 2024-07-29

### Fixed

- _(thegraph-core)_ fix: serialize SubgraphId with leading zero bytes ([#288](https://github.com/edgeandnode/toolshed/pull/288))

## [0.5.7](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.6...thegraph-core-v0.5.7) - 2024-07-23

### Fixed

- _(thegraph-core)_ fix allocation and indexer ID fmt implementations ([#272](https://github.com/edgeandnode/toolshed/pull/272))

## [0.5.6](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.5...thegraph-core-v0.5.6) - 2024-07-15

### Added

- _(thegraph-core)_ make serde dependencies optional ([#258](https://github.com/edgeandnode/toolshed/pull/258))
- _(thegraph-core)_ remove lazy_static dependency ([#257](https://github.com/edgeandnode/toolshed/pull/257))
- _(thegraph-core)_ add const support for ID macros ([#256](https://github.com/edgeandnode/toolshed/pull/256))
- _(thegraph-core)_ add deployment ID creation macro ([#254](https://github.com/edgeandnode/toolshed/pull/254))

### Other

- _(thegraph-core)_ use zero const in subgraph and deployment id macros ([#260](https://github.com/edgeandnode/toolshed/pull/260))
- _(thegraph-core)_ add attestation module docs ([#259](https://github.com/edgeandnode/toolshed/pull/259))

## [0.5.5](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.4...thegraph-core-v0.5.5) - 2024-07-12

### Fixed

- _(thegraph-core)_ add subgraph ID base58 parsing macro ([#252](https://github.com/edgeandnode/toolshed/pull/252))

## [0.5.4](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.3...thegraph-core-v0.5.4) - 2024-07-12

### Added

- _(thegraph-core)_ add allocation, indexer and subgraph ID creation macros ([#251](https://github.com/edgeandnode/toolshed/pull/251))

### Other

- _(deps)_ update rust crate test-with to 0.13.0 ([#249](https://github.com/edgeandnode/toolshed/pull/249))

## [0.5.3](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.2...thegraph-core-v0.5.3) - 2024-06-24

### Added

- _(thegraph-core)_ add allocation ID and indexer ID new-types ([#237](https://github.com/edgeandnode/toolshed/pull/237))

## [0.5.2](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.1...thegraph-core-v0.5.2) - 2024-06-17

### Fixed

- _(thegraph-core)_ return empty vec on empty results response ([#233](https://github.com/edgeandnode/toolshed/pull/233))
- _(thegraph-core)_ subgraph_client future does not implement Send trait ([#231](https://github.com/edgeandnode/toolshed/pull/231))

## [0.5.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.5.0...thegraph-core-v0.5.1) - 2024-06-03

### Fixed

- _(thegraph-core)_ add missing dependency for default-features ([#221](https://github.com/edgeandnode/toolshed/pull/221))

### Other

- fix code format issues ([#216](https://github.com/edgeandnode/toolshed/pull/216))

## [0.5.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.4.3...thegraph-core-v0.5.0) - 2024-05-30

### Added

- _(thegraph-core)_ report subgraph client errors ([#213](https://github.com/edgeandnode/toolshed/pull/213))

### Other

- _(thegraph-core)_ remove deprecated subscriptions module ([#211](https://github.com/edgeandnode/toolshed/pull/211))

## [0.4.3](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.4.2...thegraph-core-v0.4.3) - 2024-05-29

### Other

- _(thegraph-core)_ mark subscriptions module as deprecated ([#207](https://github.com/edgeandnode/toolshed/pull/207))

## [0.4.2](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.4.1...thegraph-core-v0.4.2) - 2024-05-24

### Added

- _(thegraph-core)_ make subgraph client cloneable ([#201](https://github.com/edgeandnode/toolshed/pull/201))

## [0.4.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.4.0...thegraph-core-v0.4.1) - 2024-05-09

### Fixed

- _(subgraph_client)_ avoid panic on missing query block ([#186](https://github.com/edgeandnode/toolshed/pull/186))

### Other

- _(ci)_ update the encrypted test credentials env file ([#185](https://github.com/edgeandnode/toolshed/pull/185))

## [0.4.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.3.2...thegraph-core-v0.4.0) - 2024-05-06

### Added

- set page size per query batch
- set block height for paginated queries

## [0.3.2](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.3.1...thegraph-core-v0.3.2) - 2024-05-01

### Fixed

- _(subgraph_client)_ fail fast on reorg detected ([#172](https://github.com/edgeandnode/toolshed/pull/172))

### Other

- _(deps)_ update rust crate serde_with to 3.8 ([#166](https://github.com/edgeandnode/toolshed/pull/166))
- _(deps)_ update rust crate alloy-chains to 0.1.17 ([#165](https://github.com/edgeandnode/toolshed/pull/165))

## [0.3.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.3.0...thegraph-core-v0.3.1) - 2024-04-22

### Other

- _(deps)_ update rust crate serde_json to 1.0.116 ([#159](https://github.com/edgeandnode/toolshed/pull/159))
- _(deps)_ update rust crate alloy-chains to 0.1.16 ([#155](https://github.com/edgeandnode/toolshed/pull/155))

## [0.3.0](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.2.3...thegraph-core-v0.3.0) - 2024-04-10

### Other

- _(deps)_ update rust crate reqwest to 0.12.3 ([#144](https://github.com/edgeandnode/toolshed/pull/144))

## [0.2.3](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.2.2...thegraph-core-v0.2.3) - 2024-04-10

### Other

- _(deps)_ update alloy-rs core types monorepo to 0.7 ([#151](https://github.com/edgeandnode/toolshed/pull/151))
- _(deps)_ update rust crate indoc to 2.0.5 ([#146](https://github.com/edgeandnode/toolshed/pull/146))
- _(deps)_ update rust crate tokio to v1.37.0 ([#150](https://github.com/edgeandnode/toolshed/pull/150))
- _(deps)_ update rust crate serde_json to 1.0.115
- _(deps)_ update rust crate reqwest to 0.11.27 ([#143](https://github.com/edgeandnode/toolshed/pull/143))
- _(deps)_ update rust crate reqwest to 0.11.26 ([#140](https://github.com/edgeandnode/toolshed/pull/140))
- _(deps)_ update rust crate serde_with to 3.7 ([#138](https://github.com/edgeandnode/toolshed/pull/138))

## [0.2.2](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.2.1...thegraph-core-v0.2.2) - 2024-03-11

### Other

- _(deps)_ update rust crate reqwest to 0.11.25 ([#135](https://github.com/edgeandnode/toolshed/pull/135))
- add integration tests encrypted environment ([#132](https://github.com/edgeandnode/toolshed/pull/132))
- _(deps)_ update rust crate ethers-core to 2.0.14 ([#131](https://github.com/edgeandnode/toolshed/pull/131))
- _(deps)_ update rust crate ethers to 2.0.14 ([#130](https://github.com/edgeandnode/toolshed/pull/130))
- _(deps)_ update rust crate test-with to 0.12.6 ([#128](https://github.com/edgeandnode/toolshed/pull/128))
- _(deps)_ update rust crate alloy-chains to 0.1.15 ([#127](https://github.com/edgeandnode/toolshed/pull/127))

## [0.2.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.2.0...thegraph-core-v0.2.1) - 2024-03-04

### Added

- _(thegraph-core)_ add subscriptions auth support ([#124](https://github.com/edgeandnode/toolshed/pull/124))

## [0.1.1](https://github.com/edgeandnode/toolshed/compare/thegraph-core-v0.1.0...thegraph-core-v0.1.1) - 2024-03-04

### Added

- _(thegraph-graphql-http)_ add thegraph-graphql-http crate ([#120](https://github.com/edgeandnode/toolshed/pull/120))
