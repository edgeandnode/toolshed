//! The graph network services' foundation types.

pub use alloy_primitives::{address, Address};
pub use attestation::*;
#[doc(hidden)]
pub use {::alloy_primitives, ::alloy_signer, ::alloy_sol_types};

#[doc(inline)]
pub use self::{
    allocation_id::*, attestation::Attestation, block_pointer::*, deployment_id::*, indexer_id::*,
    poi::*, subgraph_id::*,
};

mod allocation_id;
pub mod attestation;
pub mod block_pointer;
pub mod deployment_id;
mod indexer_id;
pub mod poi;
pub mod subgraph_id;
