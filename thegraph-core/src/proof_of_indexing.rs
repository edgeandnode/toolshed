//! A Proof of Indexing (POI) a cryptographic proof submitted by indexers to demonstrate that they
//! have accurately indexed a subgraph.
//!
//! The POI is essentially a signature over a message digest that is generated during the indexing
//! of a subgraph from genesis. Each time a subgraph’s state is updated, so does the message digest.

use alloy::primitives::B256;

/// A Proof of Indexing, "POI", is a cryptographic proof submitted by indexers to demonstrate that
/// they have accurately indexed a subgraph.
///
/// ## Generating test data
///
/// The `ProofOfIndexing` type implements the [`fake`] crate's [`fake::Dummy`] trait, allowing you
/// to generate random `ProofOfIndexing` values for testing.
///
/// Note that the `fake` feature must be enabled to use this functionality.
///
/// See the [`Dummy`] trait impl for usage examples.
///
/// [`Dummy`]: #impl-Dummy<Faker>-for-ProofOfIndexing
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc(alias = "poi")]
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

impl AsRef<B256> for ProofOfIndexing {
    fn as_ref(&self) -> &B256 {
        &self.0
    }
}

impl AsRef<[u8]> for ProofOfIndexing {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<[u8; 32]> for ProofOfIndexing {
    fn as_ref(&self) -> &[u8; 32] {
        self.0.as_ref()
    }
}

impl std::borrow::Borrow<[u8]> for ProofOfIndexing {
    fn borrow(&self) -> &[u8] {
        self.0.borrow()
    }
}

impl std::borrow::Borrow<[u8; 32]> for ProofOfIndexing {
    fn borrow(&self) -> &[u8; 32] {
        self.0.borrow()
    }
}

impl std::ops::Deref for ProofOfIndexing {
    type Target = B256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<B256> for ProofOfIndexing {
    fn from(bytes: B256) -> Self {
        Self(bytes)
    }
}

impl From<[u8; 32]> for ProofOfIndexing {
    fn from(value: [u8; 32]) -> Self {
        Self(value.into())
    }
}

impl<'a> From<&'a [u8; 32]> for ProofOfIndexing {
    fn from(value: &'a [u8; 32]) -> Self {
        Self(value.into())
    }
}

impl<'a> TryFrom<&'a [u8]> for ProofOfIndexing {
    type Error = <B256 as TryFrom<&'a [u8]>>::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
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

impl PartialEq<B256> for ProofOfIndexing {
    fn eq(&self, other: &B256) -> bool {
        self.0.eq(other)
    }
}

impl std::fmt::Display for ProofOfIndexing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for ProofOfIndexing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProofOfIndexing({})", self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for ProofOfIndexing {
    fn deserialize<D>(deserializer: D) -> Result<ProofOfIndexing, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::Deserialize::deserialize(deserializer).map(Self)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for ProofOfIndexing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "fake")]
/// To use the [`fake`] crate to generate random [`ProofOfIndexing`] values, **the `fake` feature
/// must be enabled.**
///
/// ```rust
/// # use thegraph_core::ProofOfIndexing;
/// # use fake::Fake;
/// let poi = fake::Faker.fake::<ProofOfIndexing>();
///
/// println!("ProofOfIndexing: {}", poi);
/// ```
impl fake::Dummy<fake::Faker> for ProofOfIndexing {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(config: &fake::Faker, rng: &mut R) -> Self {
        let bytes = <[u8; 32]>::dummy_with_rng(config, rng);
        Self(B256::new(bytes))
    }
}

/// Converts a sequence of string literals containing hex-encoded data into a new
/// [`ProofOfIndexing`] at compile time.
///
/// To create an `ProofOfIndexing` from a string literal (no `0x` prefix) at compile time:
///
/// ```rust
/// use thegraph_core::{proof_of_indexing, ProofOfIndexing};
///
/// const PROOF_OF_INDEXING: ProofOfIndexing =
///     proof_of_indexing!("bb31abb3bb85428d894fb4b3cee8a0889bbe8585939b70910bbdda31b30d2240");
/// ```
///
/// If no argument is provided, the macro will create an `ProofOfIndexing` with the zero POI:
///
/// ```rust
/// use thegraph_core::{proof_of_indexing, ProofOfIndexing};
///
/// const PROOF_OF_INDEXING: ProofOfIndexing = proof_of_indexing!();
///
/// assert_eq!(PROOF_OF_INDEXING, ProofOfIndexing::ZERO);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __proof_of_indexing {
    () => {
        $crate::ProofOfIndexing::ZERO
    };
    ($id:tt) => {
        $crate::ProofOfIndexing::new($crate::alloy::primitives::b256!($id))
    };
}
