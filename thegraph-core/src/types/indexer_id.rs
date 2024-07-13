use alloy_primitives::Address;

/// A unique identifier for an indexer: the indexer's Ethereum address.
///
/// This is a "new-type" wrapper around [`Address`] to provide type safety.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexerId(Address);

impl IndexerId {
    /// Create a new `IndexerId` from an [`Address`].
    pub const fn new(address: Address) -> Self {
        IndexerId(address)
    }

    /// Return the internal representation.
    pub fn into_inner(self) -> Address {
        self.0
    }
}

impl std::fmt::Display for IndexerId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for IndexerId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl From<Address> for IndexerId {
    fn from(address: Address) -> Self {
        IndexerId(address)
    }
}

impl std::str::FromStr for IndexerId {
    type Err = <Address as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let address = std::str::FromStr::from_str(s)?;
        Ok(IndexerId(address))
    }
}

impl PartialEq<Address> for IndexerId {
    fn eq(&self, other: &Address) -> bool {
        self.0.eq(other)
    }
}

impl AsRef<Address> for IndexerId {
    fn as_ref(&self) -> &Address {
        &self.0
    }
}

impl std::ops::Deref for IndexerId {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> serde::Deserialize<'de> for IndexerId {
    fn deserialize<D>(deserializer: D) -> Result<IndexerId, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let address = Address::deserialize(deserializer)?;
        Ok(IndexerId(address))
    }
}

impl serde::Serialize for IndexerId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

/// Converts a sequence of string literals containing hex-encoded data into a new [`IndexerId`]
/// at compile time.
///
/// To create an `IndexerId` from a string literal (no `0x` prefix) at compile time:
///
/// ```rust
/// use thegraph_core::indexer_id;
/// use thegraph_core::types::{IndexerId};
///
/// let indexer_id: IndexerId = indexer_id!("0002c67268fb8c8917f36f865a0cbdf5292fa68d");
/// ```
///
/// If no argument is provided, the macro will create an `IndexerId` with the zero address:
///
/// ```rust
/// use thegraph_core::indexer_id;
/// use thegraph_core::types::{Address, IndexerId};
///
/// let indexer_id: IndexerId = indexer_id!();
///
/// assert_eq!(indexer_id, Address::ZERO);
/// ```
#[macro_export]
macro_rules! indexer_id {
    () => {
        $crate::types::IndexerId::from(::alloy_primitives::Address::ZERO)
    };
    ($value:tt) => {
        $crate::types::IndexerId::new(::alloy_primitives::address!($value))
    };
}
