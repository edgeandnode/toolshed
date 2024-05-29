//! The client module provides a high-level client API to query subgraphs.

pub use url;

pub use subgraph_client::*;

pub mod queries;
mod subgraph_client;
