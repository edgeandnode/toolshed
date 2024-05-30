//! The client module provides a high-level client API to query subgraphs.

pub use subgraph_client::*;
pub use url;

pub mod queries;
mod subgraph_client;
