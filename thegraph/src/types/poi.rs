//! A Proof of Indexing (POI) a cryptographic proof submitted by indexers to demonstrate that they
//! have accurately indexed a subgraph.
//!
//! The POI is essentially a signature over a message digest that is generated during the indexing
//! of a subgraph from genesis. Each time a subgraph’s state is updated, so does the message digest.

use super::primitives::B256;

/// A Proof of Indexing (POI) a cryptographic proof submitted by indexers to demonstrate that they
/// have  accurately indexed a subgraph.
pub type ProofOfIndexing = B256;
