//! Rust core modules for _The Graph_ network.
//!
//! # Re-exports
//!
//! This crate re-exports the `alloy` crate, which provides core types, traits and macros.
//!
//! As this crate exports types from the `alloy` crate, it is recommended to use the re-exported
//! types and traits from this crate instead of importing the `alloy` crate directly in order to
//! avoid future version conflicts.

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
// Re-export functions required by the `deployment_id!(...)` and `subgraph_id!(...)` macros.
#[doc(hidden)]
pub use self::{deployment_id::__parse_cid_v0_const, subgraph_id::__parse_subgraph_id_const};

mod allocation_id;
pub mod attestation;
mod block;
#[cfg(feature = "subgraph-client")]
pub mod client;
mod deployment_id;
mod indexer_id;
mod proof_of_indexing;
mod subgraph_id;

// Export macros
#[doc(inline)]
pub use __allocation_id as allocation_id;
#[doc(inline)]
pub use __deployment_id as deployment_id;
#[doc(inline)]
pub use __indexer_id as indexer_id;
#[doc(inline, alias = "poi")]
pub use __proof_of_indexing as proof_of_indexing;
#[doc(inline)]
pub use __subgraph_id as subgraph_id;
