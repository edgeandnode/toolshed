thegraph-core
-------------

[![crates.io](https://img.shields.io/crates/v/thegraph-core)](https://crates.io/crates/thegraph-core)
![msrv](https://img.shields.io/crates/msrv/thegraph-core?color=darkgray)
[![license](https://img.shields.io/crates/l/thegraph-core)](../LICENSE)
[![ci](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml/badge.svg)](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/thegraph-core)](https://docs.rs/thegraph-core)

Rust core modules for _The Graph_ network.

## Usage

To add this crate to your project as a depenency use the `cargo add` command:

```shell
cargo add thegraph-core
```

In a cargo workspace use the package selection feature to add it as a dependency
of a specific package in the workspace:

```shell
cargo add --package <package-name> thegraph-core 
```

Alternatively, you can use the `Cargo.toml` file to add the dependency manually
and point to the git repository's URL and the specific tag you want to use,
for example:

```toml
thegraph-core = { git = "https://github.com/edgeandnode/toolshed.git", tag = "thegraph-core-vX.Y.Z" }
```
