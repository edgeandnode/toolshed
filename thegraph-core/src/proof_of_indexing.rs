//! A Proof of Indexing (POI) a cryptographic proof submitted by indexers to demonstrate that they
//! have accurately indexed a subgraph.
//!
//! The POI is essentially a signature over a message digest that is generated during the indexing
//! of a subgraph from genesis. Each time a subgraph’s state is updated, so does the message digest.

use alloy_primitives::B256;

/// A Proof of Indexing, "POI", is a cryptographic proof submitted by indexers to demonstrate that
/// they have accurately indexed a subgraph.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProofOfIndexing(B256);

impl ProofOfIndexing {
    /// The "zero" [`ProofOfIndexing`].
    ///
    /// This is a constant value that represents the zero POI. It is equivalent to parsing a zeroed
    /// 32-byte array.
    pub const ZERO: Self = Self(B256::ZERO);

    /// Creates a new [`ProofOfIndexing`].
    pub const fn new(bytes: B256) -> Self {
        Self(bytes)
    }
}

impl std::fmt::Display for ProofOfIndexing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for ProofOfIndexing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProofOfIndexing({})", self.0)
    }
}

impl From<B256> for ProofOfIndexing {
    fn from(bytes: B256) -> Self {
        Self(bytes)
    }
}

impl From<[u8; 32]> for ProofOfIndexing {
    fn from(value: [u8; 32]) -> Self {
        Self(B256::from(value))
    }
}

impl From<ProofOfIndexing> for B256 {
    fn from(id: ProofOfIndexing) -> Self {
        id.0
    }
}

impl From<&ProofOfIndexing> for B256 {
    fn from(id: &ProofOfIndexing) -> Self {
        id.0
    }
}

impl AsRef<B256> for ProofOfIndexing {
    fn as_ref(&self) -> &B256 {
        &self.0
    }
}

impl std::ops::Deref for ProofOfIndexing {
    type Target = B256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<B256> for ProofOfIndexing {
    fn eq(&self, other: &B256) -> bool {
        self.0.eq(other)
    }
}

/// Converts a sequence of string literals containing hex-encoded data into a new
/// [`ProofOfIndexing`] at compile time.
///
/// To create an `ProofOfIndexing` from a string literal (no `0x` prefix) at compile time:
///
/// ```rust
/// use thegraph_core::{poi, ProofOfIndexing};
///
/// const PROOF_OF_INDEXING: ProofOfIndexing =
///     poi!("bb31abb3bb85428d894fb4b3cee8a0889bbe8585939b70910bbdda31b30d2240");
/// ```
///
/// If no argument is provided, the macro will create an `ProofOfIndexing` with the zero POI:
///
/// ```rust
/// use thegraph_core::{poi, ProofOfIndexing};
///
/// const PROOF_OF_INDEXING: ProofOfIndexing = poi!();
///
/// assert_eq!(PROOF_OF_INDEXING, ProofOfIndexing::ZERO);
/// ```
#[macro_export]
macro_rules! poi {
    () => {
        $crate::ProofOfIndexing::ZERO
    };
    ($id:tt) => {
        $crate::ProofOfIndexing::new($crate::alloy_primitives::b256!($id))
    };
}
