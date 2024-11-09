//! Rust core modules for _The Graph_ network.
//!
//! # Re-exports
//!
//! This crate re-exports the `alloy` crate, which provides essential types, traits, and macros.
//!
//! As this crate relies on types from the `alloy` crate, it is advisable to use the re-exported
//! `alloy` crate instead of adding it to your `Cargo.toml` file. This approach helps to avoid
//! potential future crate version conflicts.

// Enable `doc_cfg` feature for `docs.rs`
#![cfg_attr(docsrs, feature(doc_cfg))]

// Re-export `alloy` crate
pub use alloy;

#[doc(inline)]
pub use self::{
    allocation_id::AllocationId,
    attestation::Attestation,
    block::BlockPointer,
    deployment_id::{DeploymentId, ParseDeploymentIdError},
    indexer_id::IndexerId,
    proof_of_indexing::ProofOfIndexing,
    subgraph_id::{ParseSubgraphIdError, SubgraphId},
};

mod allocation_id;
pub mod attestation;
mod block;
#[deprecated(
    note = "Use the `thegraph-client-subgraphs` crate instead",
    since = "0.8.1"
)]
#[cfg(feature = "subgraph-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "subgraph-client")))]
pub mod client;
mod deployment_id;
mod indexer_id;
mod proof_of_indexing;
mod subgraph_id;

// Export macros
#[doc(inline)]
pub use self::__allocation_id as allocation_id;
#[doc(inline)]
pub use self::__deployment_id as deployment_id;
#[doc(inline)]
pub use self::__indexer_id as indexer_id;
#[doc(inline, alias = "poi")]
pub use self::__proof_of_indexing as proof_of_indexing;
#[doc(inline)]
pub use self::__subgraph_id as subgraph_id;
// Export internal functions required by macros
#[doc(hidden)]
pub use self::{deployment_id::__parse_cid_v0_const, subgraph_id::__parse_subgraph_id_const};
