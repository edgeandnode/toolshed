//! A pointer to a block in the chain.

use alloy_primitives::{BlockHash, BlockNumber};

/// A pointer to a block in the chain.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockPointer {
    /// The block number.
    pub number: BlockNumber,
    /// The block hash.
    pub hash: BlockHash,
}
