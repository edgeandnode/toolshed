use alloy::primitives::Address;

/// A unique identifier for an indexer: the indexer's Ethereum address.
///
/// This is a "new-type" wrapper around [`Address`] to provide type safety.
///
/// ## Formatting
///
/// The `IndexerId` type implements the following formatting traits:
///
/// - Use [`std::fmt::Display`] for formatting the `IndexerId` as an [EIP-55] checksum string.
/// - Use [`std::fmt::LowerHex`] (or [`std::fmt::UpperHex`]) for formatting   the `IndexerId` as a
///   hexadecimal string.
///
/// See the [`Display`], [`LowerHex`], and [`UpperHex`] trait implementations for usage examples.
///
/// ## Generating test data
///
/// The `IndexerId` type implements the [`fake`] crate's [`fake::Dummy`] trait, allowing you to
/// generate random `IndexerId` values for testing.
///
/// Note that the `fake` feature must be enabled to use this functionality.
///
/// See the [`Dummy`] trait impl for usage examples.
///
/// [EIP-55]: https://eips.ethereum.org/EIPS/eip-55
/// [`Display`]: #impl-Display-for-IndexerId
/// [`LowerHex`]: #impl-LowerHex-for-IndexerId
/// [`UpperHex`]: #impl-UpperHex-for-IndexerId
/// [`Dummy`]: #impl-Dummy<Faker>-for-IndexerId
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexerId(Address);

impl IndexerId {
    /// Create a new [`IndexerId`].
    pub const fn new(address: Address) -> Self {
        IndexerId(address)
    }

    /// Return the internal representation.
    pub fn into_inner(self) -> Address {
        self.0
    }
}

impl std::fmt::Display for IndexerId {
    /// Formats the `IndexerId` using its [EIP-55](https://eips.ethereum.org/EIPS/eip-55)
    /// checksum representation.
    ///
    /// See [`LowerHex`] (and [`UpperHex`]) for formatting the `IndexerId` as a hexadecimal
    /// string.
    ///
    /// [`LowerHex`]: struct.IndexerId.html#impl-LowerHex-for-IndexerId
    /// [`UpperHex`]: struct.IndexerId.html#impl-UpperHex-for-IndexerId
    ///
    /// ```rust
    /// # use thegraph_core::{indexer_id, IndexerId};
    ///
    /// const ID: IndexerId = indexer_id!("0002c67268fb8c8917f36f865a0cbdf5292fa68d");
    ///
    /// // Note the uppercase and lowercase hex characters in the checksum
    /// assert_eq!(format!("{}", ID), "0x0002c67268FB8C8917F36F865a0CbdF5292FA68d");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for IndexerId {
    /// Formats the `IndexerId` using its raw lower-case hexadecimal representation.
    ///
    /// It is advised to use the [`LowerHex`] (and [`UpperHex`]) format trait implementation over
    /// the [`Debug`](std::fmt::Debug) implementation to format the `IndexerId` as a lower-case
    /// hexadecimal string.
    ///
    /// This implementation matches `alloy_primitives::Address`'s `Debug` implementation.
    ///
    /// [`LowerHex`]: struct.IndexerId.html#impl-LowerHex-for-IndexerId
    /// [`UpperHex`]: struct.IndexerId.html#impl-UpperHex-for-IndexerId
    ///
    /// ```rust
    /// # use thegraph_core::{indexer_id, IndexerId};
    /// const ID: IndexerId = indexer_id!("0002c67268fb8c8917f36f865a0cbdf5292fa68d");
    ///
    /// assert_eq!(format!("{:?}", ID), "0x0002c67268fb8c8917f36f865a0cbdf5292fa68d");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl std::fmt::LowerHex for IndexerId {
    /// Lowercase hex representation of the `IndexerId`.
    ///
    /// Note that the alternate flag, `#`, adds a `0x` in front of the output.
    ///
    /// ```rust
    /// # use thegraph_core::{indexer_id, IndexerId};
    /// const ID: IndexerId = indexer_id!("0002c67268fb8c8917f36f865a0cbdf5292fa68d");
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

impl std::fmt::UpperHex for IndexerId {
    /// Uppercase hex representation of the `IndexerId`.
    ///
    /// Note that the alternate flag, `#`, adds a `0x` in front of the output.
    ///
    /// ```rust
    /// # use thegraph_core::{indexer_id, IndexerId};
    /// const ID: IndexerId = indexer_id!("0002c67268fb8c8917f36f865a0cbdf5292fa68d");
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

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for IndexerId {
    fn deserialize<D>(deserializer: D) -> Result<IndexerId, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let address = Address::deserialize(deserializer)?;
        Ok(IndexerId(address))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for IndexerId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "fake")]
/// To use the [`fake`] crate to generate random [`IndexerId`] values, **the `fake` feature must
/// be enabled.**
///
/// ```rust
/// # use thegraph_core::IndexerId;
/// # use fake::Fake;
/// let indexer_id = fake::Faker.fake::<IndexerId>();
///
/// println!("IndexerId: {:#x}", indexer_id);
/// ```
impl fake::Dummy<fake::Faker> for IndexerId {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
        use crate::fake_impl::alloy::Alloy;
        Self(Address::dummy_with_rng(&Alloy, rng))
    }
}

/// Converts a sequence of string literals containing hex-encoded data into a new [`IndexerId`]
/// at compile time.
///
/// To create an `IndexerId` from a string literal (no `0x` prefix) at compile time:
///
/// ```rust
/// # use thegraph_core::{indexer_id, IndexerId};
/// const INDEXER_ID: IndexerId = indexer_id!("0002c67268fb8c8917f36f865a0cbdf5292fa68d");
/// ```
///
/// If no argument is provided, the macro will create an `IndexerId` with the zero address:
///
/// ```rust
/// # use thegraph_core::{
/// #    alloy::primitives::Address,
/// #    indexer_id, IndexerId
/// # };
/// const INDEXER_ID: IndexerId = indexer_id!();
///
/// assert_eq!(INDEXER_ID, Address::ZERO);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __indexer_id {
    () => {
        $crate::IndexerId::new($crate::alloy::primitives::Address::ZERO)
    };
    ($value:tt) => {
        $crate::IndexerId::new($crate::alloy::primitives::address!($value))
    };
}
