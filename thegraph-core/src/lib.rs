//! Rust core modules for _The Graph_ network.
//!
//! # Re-export of the [`alloy`] crate
//!
//! This crate re-exports the [`alloy`] crate, which provides essential types, traits, and macros.
//!
//! To avoid potential future crate version conflicts, it is recommended to use the re-exported
//! `alloy` crate instead of adding it directly to your `Cargo.toml` file.
//!
//! For convenience, this crate also re-exports the features of the `alloy` crate. These features
//! follow the naming convention `alloy-<feature>`. For example, the `alloy-signers` and
//! `alloy-signer-local` features enable the `signers` and `signer-local` optional features of the
//! `alloy` crate, respectively.
//!
//! If you need to enable an `alloy` crate feature that is not yet re-exported by this crate, you
//! can enable the `alloy-full` feature to enable all `alloy` features.
//!
//! # Features
//!
//! The following features are available for this crate:
//!
//! - `attestation`: Enables the `attestation` module, which provides types and functions for
//!    attestation-related operations.
//! - `async-graphql`: Enables support for the [`async-graphql`] crate.
//! - `fake`: Enables the [`fake`] crate integration for generating random test data.
//! - `serde`: Enables [`serde`] serialization and deserialization support for types in this crate.
//! - `signed-message`: Enables the `signed_message` module, which provides types and functions for
//!                     EIP-712 message signing and verification.
//!
//! Additionally, this crate re-exports other features from the `alloy` crate as described above.

// Enable `doc_cfg` feature for `docs.rs`
#![cfg_attr(docsrs, feature(doc_cfg))]

// Re-export `alloy` crate
pub use alloy;

#[doc(inline)]
pub use self::{
    allocation_id::AllocationId,
    block::BlockPointer,
    deployment_id::{DeploymentId, ParseDeploymentIdError},
    indexer_id::IndexerId,
    proof_of_indexing::ProofOfIndexing,
    subgraph_id::{ParseSubgraphIdError, SubgraphId},
};

mod allocation_id;
#[cfg(feature = "attestation")]
#[cfg_attr(docsrs, doc(cfg(feature = "attestation")))]
pub mod attestation;
mod block;
mod deployment_id;
mod indexer_id;
mod proof_of_indexing;
#[cfg(feature = "signed-message")]
#[cfg_attr(docsrs, doc(cfg(feature = "signed-message")))]
pub mod signed_message;
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
