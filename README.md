toolshed
--------
[![ci](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml/badge.svg)](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

This repo contains a collection of crates that constitute the foundation for the different Rust projects of the _The
Graph network_ (e.g., _The Graph Gateway_ and its _Candidate Selection_, etc.).

### Crates

| Name                                             |                                                          Latest version                                                           |
|:-------------------------------------------------|:---------------------------------------------------------------------------------------------------------------------------------:|
| [thegraph-core](./thegraph-core)                 |             [![thegraph-core](https://img.shields.io/crates/v/thegraph-core)](https://crates.io/crates/thegraph-core)             |
| [thegraph-graphql-http](./thegraph-graphql-http) | [![thegraph-graphql-http](https://img.shields.io/crates/v/thegraph-graphql-http)](https://crates.io/crates/thegraph-graphql-http) |

### Internal (crates not published to crates.io)

* **graphql:** A collection of GraphQL related Rust modules that are share between The Graph's network services.

    ```toml
    graphql = { git = "https://github.com/edgeandnode/toolshed", tag = "graphql-v0.3.0" }
    ```

* **toolshed:** A collection of miscellaneous rust modules.

    ```toml
    toolshed = { git = "https://github.com/edgeandnode/toolshed", tag = "toolshed-v0.6.0" }
    ```
