toolshed
--------
[![ci](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml/badge.svg)](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

This repository contains a collection of crates that constitute the foundation for the different Rust projects of
_The Graph_ network (e.g., [_the Subgraphs Gateway_](https://github.com/edgeandnode/gateway), [_the Indexer
software_](https://github.com/graphprotocol/indexer-rs), etc.).

### Crates

| Name                                                     |                                                                Latest version                                                                 |                                                  Documentation                                                   |
|:---------------------------------------------------------|:---------------------------------------------------------------------------------------------------------------------------------------------:|:----------------------------------------------------------------------------------------------------------------:|
| [thegraph-core](./thegraph-core)                         |                   [![thegraph-core](https://img.shields.io/crates/v/thegraph-core)](https://crates.io/crates/thegraph-core)                   |             [![docs.rs](https://img.shields.io/docsrs/thegraph-core)](https://docs.rs/thegraph-core)             |
| [thegraph-graphql-http](./thegraph-graphql-http)         |       [![thegraph-graphql-http](https://img.shields.io/crates/v/thegraph-graphql-http)](https://crates.io/crates/thegraph-graphql-http)       |     [![docs.rs](https://img.shields.io/docsrs/thegraph-graphql-http)](https://docs.rs/thegraph-graphql-http)     |
| [thegraph-client-subgraphs](./thegraph-client-subgraphs) | [![thegraph-client-subgraphs](https://img.shields.io/crates/v/thegraph-client-subgraphs)](https://crates.io/crates/thegraph-client-subgraphs) | [![docs.rs](https://img.shields.io/docsrs/thegraph-client-subgraphs)](https://docs.rs/thegraph-client-subgraphs) |

### Internal (not published to crates.io)

* **graphql:** A collection of GraphQL related Rust modules that are share between The Graph's network services.

    ```toml
    graphql = { git = "https://github.com/edgeandnode/toolshed", tag = "graphql-v0.3.0" }
    ```

### Contributing

This is an open-source project and we welcome contributions. Please refer to our
[contributing guide](CONTRIBUTING.md) for more information on how contribute to this project.
