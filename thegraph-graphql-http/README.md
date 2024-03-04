thegraph-graphql-http
---------------------

[![Crates.io](https://img.shields.io/crates/v/thegraph-graphql-http)](https://crates.io/crates/thegraph-graphql-http)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](../LICENSE)
[![ci](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml/badge.svg)](https://github.com/edgeandnode/toolshed/actions/workflows/ci.yml)

A rust implementation of the GraphQL-over-HTTP spec for The Graph network services.

## Usage

To add this crate to your project as a depenency use the `cargo add` command:

```shell
cargo add thegraph-graphql-http
```

In a cargo workspace use the package selection feature to add it as a dependency
of a specific package in the workspace:

```shell
cargo add --package <package-name> thegraph-graphql-http
```

Alternatively, you can use the `Cargo.toml` file to add the dependency manually
and point to the git repository's URL and the specific tag you want to use,
for example:

```toml
thegraph-graphql-http = { git = "https://github.com/edgeandnode/toolshed.git", tag = "thegraph-graphql-http-vX.Y.Z" }
```
