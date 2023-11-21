use alloy_primitives::{BlockHash, BlockNumber};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BlockPointer {
    pub number: BlockNumber,
    pub hash: BlockHash,
}
