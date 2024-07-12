use alloy_primitives::Address;

/// A unique identifier for an indexer: the indexer's Ethereum address.
///
/// This is a "new-type" wrapper around `Address` to provide type safety.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexerId(Address);

impl IndexerId {
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
#[macro_export]
macro_rules! indexer_id {
    () => {
        IndexerId(Address::ZERO)
    };
    ($addr:tt) => {
        IndexerId(alloy_primitives::address!($addr))
    };
}
