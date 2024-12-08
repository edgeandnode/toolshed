thegraph-headers
----------------

[![crates.io](https://img.shields.io/crates/v/thegraph-headers)](https://crates.io/crates/thegraph-headers)
![msrv](https://img.shields.io/crates/msrv/thegraph-headers?color=darkgray)
[![license](https://img.shields.io/crates/l/thegraph-headers)](../LICENSE)
[![ci](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml/badge.svg)](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/thegraph-headers)](https://docs.rs/thegraph-headers)

Common HTTP headers for _The Graph_ network services.

## Usage

To add this crate to your project as a dependency use the `cargo add` command:

```shell
cargo add thegraph-headers
```

In a cargo workspace use the package selection feature to add it as a dependency
of a specific package in the workspace:

```shell
cargo add --package <package-name> thegraph-headers 
```

Alternatively, you can use the `Cargo.toml` file to add the dependency manually
and point to the git repository's URL and the specific tag you want to use,
for example:

```toml
thegraph-headers = { git = "https://github.com/edgeandnode/toolshed.git", tag = "thegraph-headers-vX.Y.Z" }
```
