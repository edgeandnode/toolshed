use alloy::primitives::{Address, FixedBytes};

use crate::allocation_id::AllocationId;

/// A unique identifier for a collection: the Graph Tally's payment identifier.
///
/// This is a "new-type" wrapper around [`FixedBytes<32>`] to provide type safety.
///
/// ## Formatting
///
/// The `CollectionId` type implements the following formatting traits:
///
/// - Use [`std::fmt::Display`] for formatting the `CollectionId` as a raw lower-case hexadecimal string.
/// - Use [`std::fmt::LowerHex`] (or [`std::fmt::UpperHex`]) for formatting   the `CollectionId` as
///   a hexadecimal string.
///
/// See the [`Display`], [`LowerHex`], and [`UpperHex`] trait implementations for usage examples.
///
/// ## Generating test data
///
/// The `CollectionId` type implements the [`fake`] crate's [`fake::Dummy`] trait, allowing you to
/// generate random `CollectionId` values for testing.
///
/// Note that the `fake` feature must be enabled to use this functionality.
///
/// See the [`Dummy`] trait impl for usage examples.
///
/// [`Display`]: #impl-Display-for-CollectionId
/// [`LowerHex`]: #impl-LowerHex-for-CollectionId
/// [`UpperHex`]: #impl-UpperHex-for-CollectionId
/// [`Dummy`]: #impl-Dummy<Faker>-for-CollectionId
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CollectionId(FixedBytes<32>);

impl CollectionId {
    /// Create a new [`CollectionId`].
    pub const fn new(collection: FixedBytes<32>) -> Self {
        CollectionId(collection)
    }

    /// Return the internal representation.
    pub fn into_inner(self) -> FixedBytes<32> {
        self.0
    }

    /// Converts this `CollectionId` into an `Address`, assuming it was originally derived from a
    /// left-padded address (i.e., the last 20 bytes are the address).
    ///
    /// ```rust
    /// use thegraph_core::{
    ///     alloy::primitives::Address,
    ///     alloy::primitives::address,
    ///     collection_id, CollectionId,
    /// };
    ///
    /// let collection_id: CollectionId = collection_id!("0000000000000000000000003e1f9c2ab4c7f1b3d7e839ebe6ae451c8a0b1d24");
    /// assert_eq!(collection_id.as_address(), address!("3e1f9c2ab4c7f1b3d7e839ebe6ae451c8a0b1d24"));
    /// ```
    pub fn as_address(&self) -> Address {
        Address::from_slice(&self.0.as_slice()[12..])
    }
}

impl std::fmt::Display for CollectionId {
    /// Formats the `CollectionId` using its raw lower-case hexadecimal representation.
    ///
    /// See [`LowerHex`] (and [`UpperHex`]) for formatting the `CollectionId` as a hexadecimal
    /// string.
    ///
    /// [`LowerHex`]: struct.CollectionId.html#impl-LowerHex-for-CollectionId
    /// [`UpperHex`]: struct.CollectionId.html#impl-UpperHex-for-CollectionId
    ///
    /// ```rust
    /// # use thegraph_core::{collection_id, CollectionId};
    /// const ID: CollectionId = collection_id!("8f2c4a779f66bde2e9c3d81d4315e91db8a42afee0d5f9947c20ab54be73e611");
    ///
    /// assert_eq!(format!("{}", ID), "0x8f2c4a779f66bde2e9c3d81d4315e91db8a42afee0d5f9947c20ab54be73e611");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for CollectionId {
    /// Formats the `CollectionId` using its raw lower-case hexadecimal representation.
    ///
    /// It is advised to use the [`LowerHex`] (and [`UpperHex`]) format trait implementation over
    /// the [`Debug`](std::fmt::Debug) implementation to format the `CollectionId` as a lower-case
    /// hexadecimal string.
    ///
    /// This implementation matches `alloy_primitives::FixedBytes`'s `Debug` implementation.
    ///
    /// [`LowerHex`]: struct.CollectionId.html#impl-LowerHex-for-CollectionId
    /// [`UpperHex`]: struct.CollectionId.html#impl-UpperHex-for-CollectionId
    ///
    /// ```rust
    /// # use thegraph_core::{collection_id, CollectionId};
    /// const ID: CollectionId = collection_id!("8f2c4a779f66bde2e9c3d81d4315e91db8a42afee0d5f9947c20ab54be73e611");
    ///
    /// assert_eq!(format!("{:?}", ID), "0x8f2c4a779f66bde2e9c3d81d4315e91db8a42afee0d5f9947c20ab54be73e611");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl std::fmt::LowerHex for CollectionId {
    /// Lowercase hex representation of the `CollectionId`.
    ///
    /// Note that the alternate flag, `#`, adds a `0x` in front of the output.
    ///
    /// ```rust
    /// # use thegraph_core::{collection_id, CollectionId};
    /// const ID: CollectionId = collection_id!("8f2c4a779f66bde2e9c3d81d4315e91db8a42afee0d5f9947c20ab54be73e611");
    ///
    /// // Lower hex
    /// assert_eq!(format!("{:x}", ID), "8f2c4a779f66bde2e9c3d81d4315e91db8a42afee0d5f9947c20ab54be73e611");
    ///
    /// // Lower hex with alternate flag
    /// assert_eq!(format!("{:#x}", ID), "0x8f2c4a779f66bde2e9c3d81d4315e91db8a42afee0d5f9947c20ab54be73e611");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.0, f)
    }
}

impl std::fmt::UpperHex for CollectionId {
    /// Uppercase hex representation of the `CollectionId`.
    ///
    /// Note that the alternate flag, `#`, adds a `0x` in front of the output.
    ///
    /// ```rust
    /// # use thegraph_core::{collection_id, CollectionId};
    /// const ID: CollectionId = collection_id!("8f2c4a779f66bde2e9c3d81d4315e91db8a42afee0d5f9947c20ab54be73e611");
    ///
    /// // Upper hex
    /// assert_eq!(format!("{:X}", ID), "8F2C4A779F66BDE2E9C3D81D4315E91DB8A42AFEE0D5F9947C20AB54BE73E611");
    ///
    /// // Upper hex with alternate flag
    /// assert_eq!(format!("{:#X}", ID), "0x8F2C4A779F66BDE2E9C3D81D4315E91DB8A42AFEE0D5F9947C20AB54BE73E611");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.0, f)
    }
}

impl From<FixedBytes<32>> for CollectionId {
    fn from(collection: FixedBytes<32>) -> Self {
        CollectionId(collection)
    }
}

/// Convert an [`Address`] into a [`CollectionId`] by zero padding the address to 32 bytes.
///
/// ```rust
/// use thegraph_core::{
///     alloy::primitives::Address,
///     alloy::primitives::address,
///     collection_id, CollectionId,
/// };
/// let address: Address = address!("3e1f9c2aB4C7F1b3d7E839EbE6Ae451c8A0b1d24");
/// let collection_id: CollectionId = address.into();
/// assert_eq!(format!("{:?}", collection_id), "0x0000000000000000000000003e1f9c2ab4c7f1b3d7e839ebe6ae451c8a0b1d24");
///
/// let collection_id2: CollectionId = CollectionId::from(address!("3e1f9c2aB4C7F1b3d7E839EbE6Ae451c8A0b1d24"));
/// assert_eq!(format!("{:?}", collection_id2), "0x0000000000000000000000003e1f9c2ab4c7f1b3d7e839ebe6ae451c8a0b1d24");
/// ```
impl From<Address> for CollectionId {
    fn from(address: Address) -> Self {
        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(address.as_slice());
        CollectionId(FixedBytes::<32>::from(buf))
    }
}

/// Convert an [`AllocationId`] into a [`CollectionId`] by zero padding the allocation id to 32 bytes.
///
/// ```rust
/// use thegraph_core::{
///     alloy::primitives::Address,
///     alloy::primitives::address,
///     collection_id, CollectionId,
///     allocation_id, AllocationId
/// };
///
/// let allocation_id: AllocationId = allocation_id!("3e1f9c2aB4C7F1b3d7E839EbE6Ae451c8A0b1d24");
/// let collection_id: CollectionId = allocation_id.into();
/// assert_eq!(format!("{:?}", collection_id), "0x0000000000000000000000003e1f9c2ab4c7f1b3d7e839ebe6ae451c8a0b1d24");
///
/// let collection_id2: CollectionId = CollectionId::from(allocation_id!("3e1f9c2aB4C7F1b3d7E839EbE6Ae451c8A0b1d24"));
/// assert_eq!(format!("{:?}", collection_id2), "0x0000000000000000000000003e1f9c2ab4c7f1b3d7e839ebe6ae451c8a0b1d24");
/// ```
impl From<AllocationId> for CollectionId {
    fn from(allocation: AllocationId) -> Self {
        let mut buf = [0u8; 32];
        buf[12..].copy_from_slice(allocation.as_slice());
        CollectionId(FixedBytes::<32>::from(buf))
    }
}

impl std::str::FromStr for CollectionId {
    type Err = <FixedBytes<32> as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let collection = std::str::FromStr::from_str(s)?;
        Ok(CollectionId(collection))
    }
}

impl PartialEq<FixedBytes<32>> for CollectionId {
    fn eq(&self, other: &FixedBytes<32>) -> bool {
        self.0.eq(other)
    }
}

impl AsRef<FixedBytes<32>> for CollectionId {
    fn as_ref(&self) -> &FixedBytes<32> {
        &self.0
    }
}

impl std::ops::Deref for CollectionId {
    type Target = FixedBytes<32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CollectionId {
    fn deserialize<D>(deserializer: D) -> Result<CollectionId, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let collection = FixedBytes::<32>::deserialize(deserializer)?;
        Ok(CollectionId(collection))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for CollectionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "fake")]
/// To use the [`fake`] crate to generate random [`CollectionId`] values, **the `fake` feature must
/// be enabled.**
///
/// ```rust
/// # use thegraph_core::CollectionId;
/// # use fake::Fake;
/// let collection_id = fake::Faker.fake::<CollectionId>();
///
/// println!("CollectionId: {:#x}", collection_id);
/// ```
impl fake::Dummy<fake::Faker> for CollectionId {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(_: &fake::Faker, rng: &mut R) -> Self {
        use crate::fake_impl::alloy::Alloy;
        Self(FixedBytes::<32>::dummy_with_rng(&Alloy, rng))
    }
}

/// Converts a sequence of string literals containing hex-encoded data into a new [`CollectionId`]
/// at compile time.
///
/// To create an `CollectionId` from a string literal (no `0x` prefix) at compile time:
///
/// ```rust
/// use thegraph_core::{collection_id, CollectionId};
///
/// const COLLECTION_ID: CollectionId = collection_id!("8f2c4a779f66bde2e9c3d81d4315e91db8a42afee0d5f9947c20ab54be73e611");
/// ```
///
/// If no argument is provided, the macro will create an `CollectionId` with the zero address:
///
/// ```rust
/// use thegraph_core::{
///     alloy::primitives::FixedBytes,
///     collection_id, CollectionId
/// };
///
/// const COLLECTION_ID: CollectionId = collection_id!();
///
/// assert_eq!(COLLECTION_ID, FixedBytes::<32>::ZERO);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __collection_id {
    () => {
        $crate::CollectionId::new($crate::alloy::primitives::FixedBytes::<32>::ZERO)
    };
    ($value:tt) => {
        $crate::CollectionId::new($crate::alloy::primitives::fixed_bytes!($value))
    };
}
