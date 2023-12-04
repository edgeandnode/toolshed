use serde::{Deserialize, Serialize};

pub use alloy_primitives::{BlockHash, BlockNumber};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BlockPointer {
    pub number: BlockNumber,
    pub hash: BlockHash,
}
