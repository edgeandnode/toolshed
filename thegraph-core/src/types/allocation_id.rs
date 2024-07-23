use alloy_primitives::Address;

/// A unique identifier for an allocation: the allocation's Ethereum address.
///
/// This is a "new-type" wrapper around [`Address`] to provide type safety.
///
/// ## Formatting
///
/// The `AllocationId` type implements the following formatting traits:
///
/// - Use [`Display`] for formatting the `AllocationId` as an [EIP-55] checksum string.
/// - Use [`LowerHex`] (or [`UpperHex`]) for formatting the `AllocationId` as a hexadecimal string.
///
/// [EIP-55]: https://eips.ethereum.org/EIPS/eip-55
/// [`Display`]: struct.AllocationId.html#impl-Display-for-AllocationId
/// [`LowerHex`]: struct.AllocationId.html#impl-LowerHex-for-AllocationId
/// [`UpperHex`]: struct.AllocationId.html#impl-UpperHex-for-AllocationId
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AllocationId(Address);

impl AllocationId {
    /// Create a new [`AllocationId`].
    pub const fn new(address: Address) -> Self {
        AllocationId(address)
    }

    /// Return the internal representation.
    pub fn into_inner(self) -> Address {
        self.0
    }
}

impl std::fmt::Display for AllocationId {
    /// Formats the `AllocationId` using its [EIP-55](https://eips.ethereum.org/EIPS/eip-55)
    /// checksum representation.
    ///
    /// See [`LowerHex`] (and [`UpperHex`]) for formatting the `AllocationId` as a hexadecimal
    /// string.
    ///
    /// [`LowerHex`]: struct.AllocationId.html#impl-LowerHex-for-AllocationId
    /// [`UpperHex`]: struct.AllocationId.html#impl-UpperHex-for-AllocationId
    ///
    /// ```rust
    /// use thegraph_core::allocation_id;
    /// use thegraph_core::types::AllocationId;
    ///
    /// const ID: AllocationId = allocation_id!("0002c67268fb8c8917f36f865a0cbdf5292fa68d");
    ///
    /// // Note the uppercase and lowercase hex characters in the checksum
    /// assert_eq!(format!("{}", ID), "0x0002c67268FB8C8917F36F865a0CbdF5292FA68d");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for AllocationId {
    /// Formats the `AllocationId` using its raw lower-case hexadecimal representation.
    ///
    /// It is advised to use the [`LowerHex`] (and [`UpperHex`]) format trait implementation over
    /// the [`Debug`](std::fmt::Debug) implementation to format the `AllocationId` as a lower-case
    /// hexadecimal string.
    ///
    /// This implementation matches `alloy_primitives::Address`'s `Debug` implementation.
    ///
    /// [`LowerHex`]: struct.AllocationId.html#impl-LowerHex-for-AllocationId
    /// [`UpperHex`]: struct.AllocationId.html#impl-UpperHex-for-AllocationId
    ///
    /// ```rust
    /// use thegraph_core::allocation_id;
    /// use thegraph_core::types::AllocationId;
    ///
    /// const ID: AllocationId = allocation_id!("0002c67268fb8c8917f36f865a0cbdf5292fa68d");
    ///
    /// assert_eq!(format!("{:?}", ID), "0x0002c67268fb8c8917f36f865a0cbdf5292fa68d");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl std::fmt::LowerHex for AllocationId {
    /// Lowercase hex representation of the `AllocationId`.
    ///
    /// Note that the alternate flag, `#`, adds a `0x` in front of the output.
    ///
    /// ```rust
    /// use thegraph_core::allocation_id;
    /// use thegraph_core::types::AllocationId;
    ///
    /// const ID: AllocationId = allocation_id!("0002c67268fb8c8917f36f865a0cbdf5292fa68d");
    ///
    /// // Lower hex
    /// assert_eq!(format!("{:x}", ID), "0002c67268fb8c8917f36f865a0cbdf5292fa68d");
    ///
    /// // Lower hex with alternate flag
    /// assert_eq!(format!("{:#x}", ID), "0x0002c67268fb8c8917f36f865a0cbdf5292fa68d");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl std::fmt::UpperHex for AllocationId {
    /// Uppercase hex representation of the `AllocationId`.
    ///
    /// Note that the alternate flag, `#`, adds a `0x` in front of the output.
    ///
    /// ```rust
    /// use thegraph_core::allocation_id;
    /// use thegraph_core::types::AllocationId;
    ///
    /// const ID: AllocationId = allocation_id!("0002c67268fb8c8917f36f865a0cbdf5292fa68d");
    ///
    /// // Upper hex
    /// assert_eq!(format!("{:X}", ID), "0002C67268FB8C8917F36F865A0CBDF5292FA68D");
    ///
    /// // Upper hex with alternate flag
    /// assert_eq!(format!("{:#X}", ID), "0x0002C67268FB8C8917F36F865A0CBDF5292FA68D");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.0, f)
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

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AllocationId {
    fn deserialize<D>(deserializer: D) -> Result<AllocationId, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let address = Address::deserialize(deserializer)?;
        Ok(AllocationId(address))
    }
}

#[cfg(feature = "serde")]
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
///
/// To create an `AllocationId` from a string literal (no `0x` prefix) at compile time:
///
/// ```rust
/// use thegraph_core::allocation_id;
/// use thegraph_core::types::AllocationId;
///
/// const ALLOCATION_ID: AllocationId = allocation_id!("0002c67268fb8c8917f36f865a0cbdf5292fa68d");
/// ```
///
/// If no argument is provided, the macro will create an `AllocationId` with the zero address:
///
/// ```rust
/// use thegraph_core::allocation_id;
/// use thegraph_core::types::{Address, AllocationId };
///
/// const ALLOCATION_ID: AllocationId = allocation_id!();
///
/// assert_eq!(ALLOCATION_ID, Address::ZERO);
/// ```
#[macro_export]
macro_rules! allocation_id {
    () => {
        $crate::types::AllocationId::new($crate::alloy_primitives::Address::ZERO)
    };
    ($value:tt) => {
        $crate::types::AllocationId::new($crate::alloy_primitives::address!($value))
    };
}
