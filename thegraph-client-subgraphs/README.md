thegraph-client-subgraphs
-------------------------

[![crates.io](https://img.shields.io/crates/v/thegraph-client-subgraphs)](https://crates.io/crates/thegraph-client-subgraphs)
![msrv](https://img.shields.io/crates/msrv/thegraph-client-subgraphs?color=darkgray)
[![license](https://img.shields.io/crates/l/thegraph-client-subgraphs)](../LICENSE)
[![ci](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml/badge.svg)](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/thegraph-client-subgraphs)](https://docs.rs/thegraph-client-subgraphs)

A client for _The Graph_ network's Subgraphs data service.

## Usage

To add this crate to your project as a dependency use the `cargo add` command:

```shell
cargo add thegraph-client-subgraphs
```

In a cargo workspace use the package selection feature to add it as a dependency
of a specific package in the workspace:

```shell
cargo add --package <package-name> thegraph-client-subgraphs 
```

Alternatively, you can use the `Cargo.toml` file to add the dependency manually
and point to the git repository's URL and the specific tag you want to use,
for example:

```toml
thegraph-client-subgraphs = { git = "https://github.com/edgeandnode/toolshed.git", tag = "thegraph-client-subgraphs-vX.Y.Z" }
```
