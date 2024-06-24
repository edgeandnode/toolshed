# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
