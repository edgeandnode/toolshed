//! A collection of Rust modules that are shared between _The Graph_'s network services.

#[doc(inline)]
pub use alloy_primitives::{address, Address, BlockHash, BlockNumber, BlockTimestamp, ChainId};
#[doc(hidden)]
pub use {::alloy_primitives, ::alloy_signer, ::alloy_sol_types};

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
