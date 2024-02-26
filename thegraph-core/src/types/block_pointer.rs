//! A pointer to a block in the chain.

use alloy_primitives::{BlockHash, BlockNumber};
use serde::{Deserialize, Serialize};

/// A pointer to a block in the chain.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BlockPointer {
    /// The block number.
    pub number: BlockNumber,
    /// The block hash.
    pub hash: BlockHash,
}
