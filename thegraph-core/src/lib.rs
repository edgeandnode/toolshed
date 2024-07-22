//! A collection of Rust modules that are shared between _The Graph_'s network services.
//!
//! The `types` module contains the rust types common between the different services
//! in _The Graph_ network and serve as foundational building blocks for the rest of
//! the codebase.

pub use alloy_primitives::address;
#[doc(hidden)]
pub use {::alloy_primitives, ::alloy_signer, ::alloy_sol_types};

pub mod types;

#[cfg(feature = "subgraph-client")]
pub mod client;
