use alloy_primitives::Address;

/// A unique identifier for an allocation: the allocation's Ethereum address.
///
/// This is a "new-type" wrapper around `Address` to provide type safety.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AllocationId(Address);

impl AllocationId {
    /// Return the internal representation.
    pub fn into_inner(self) -> Address {
        self.0
    }
}

impl std::fmt::Display for AllocationId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for AllocationId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl From<Address> for AllocationId {
    fn from(address: Address) -> Self {
        AllocationId(address)
    }
}

impl std::str::FromStr for AllocationId {
    type Err = <Address as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let address = std::str::FromStr::from_str(s)?;
        Ok(AllocationId(address))
    }
}

impl PartialEq<Address> for AllocationId {
    fn eq(&self, other: &Address) -> bool {
        self.0.eq(other)
    }
}

impl AsRef<Address> for AllocationId {
    fn as_ref(&self) -> &Address {
        &self.0
    }
}

impl std::ops::Deref for AllocationId {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> serde::Deserialize<'de> for AllocationId {
    fn deserialize<D>(deserializer: D) -> Result<AllocationId, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let address = Address::deserialize(deserializer)?;
        Ok(AllocationId(address))
    }
}

impl serde::Serialize for AllocationId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

/// Converts a sequence of string literals containing hex-encoded data into a new [`AllocationId`]
/// at compile time.
#[macro_export]
macro_rules! allocation_id {
    () => {
        AllocationId(Address::ZERO)
    };
    ($addr:tt) => {
        AllocationId(alloy_primitives::address!($addr))
    };
}
