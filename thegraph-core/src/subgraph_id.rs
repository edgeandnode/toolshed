use alloy::primitives::B256;

/// Subgraph ID parsing error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseSubgraphIdError {
    /// Invalid string length. The input string is longer than 44 characters.
    #[error("invalid length {length}: {value} (length must be <=44)")]
    InvalidLength { value: String, length: usize },

    /// Invalid base-58 string. The input string contains invalid characters.
    #[error("invalid character \"{value}\": {error}")]
    InvalidCharacter { value: String, error: String },
}

/// A Subgraph ID is a 32-byte identifier for a subgraph.
///
/// ## Generating test data
///
/// The `SubgraphId` type implements the [`fake`] crate's [`fake::Dummy`] trait, allowing you to
/// generate random `SubgraphId` values for testing.
///
/// Note that the `fake` feature must be enabled to use this functionality.
///
/// See the [`Dummy`] trait impl for usage examples.
///
/// [`Dummy`]: #impl-Dummy<Faker>-for-SubgraphId
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[repr(transparent)]
pub struct SubgraphId(B256);

impl SubgraphId {
    /// The "zero" [`SubgraphId`].
    ///
    /// This is a constant value that represents the zero ID. It is equivalent to parsing a zeroed
    /// 32-byte array.
    pub const ZERO: Self = Self(B256::ZERO);

    /// Create a new [`SubgraphId`].
    pub const fn new(value: B256) -> Self {
        Self(value)
    }

    /// Get the bytes of the [`SubgraphId`] as a slice.
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_ref()
    }
}

impl AsRef<B256> for SubgraphId {
    fn as_ref(&self) -> &B256 {
        &self.0
    }
}

impl AsRef<[u8]> for SubgraphId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<[u8; 32]> for SubgraphId {
    fn as_ref(&self) -> &[u8; 32] {
        self.0.as_ref()
    }
}

impl std::borrow::Borrow<[u8]> for SubgraphId {
    fn borrow(&self) -> &[u8] {
        self.0.borrow()
    }
}

impl std::borrow::Borrow<[u8; 32]> for SubgraphId {
    fn borrow(&self) -> &[u8; 32] {
        self.0.borrow()
    }
}

impl std::ops::Deref for SubgraphId {
    type Target = B256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<B256> for SubgraphId {
    fn from(bytes: B256) -> Self {
        Self(bytes)
    }
}

impl From<[u8; 32]> for SubgraphId {
    fn from(value: [u8; 32]) -> Self {
        Self(value.into())
    }
}

impl<'a> From<&'a [u8; 32]> for SubgraphId {
    fn from(value: &'a [u8; 32]) -> Self {
        Self(value.into())
    }
}

impl<'a> TryFrom<&'a [u8]> for SubgraphId {
    type Error = <B256 as TryFrom<&'a [u8]>>::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

impl From<SubgraphId> for B256 {
    fn from(id: SubgraphId) -> Self {
        id.0
    }
}

impl From<&SubgraphId> for B256 {
    fn from(id: &SubgraphId) -> Self {
        id.0
    }
}

impl std::str::FromStr for SubgraphId {
    type Err = ParseSubgraphIdError;

    /// Parse a `SubgraphID` from a base-58 encoded string.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut buffer = [0_u8; 32];

        // Decode the base58 string into a byte array, and get the number of bytes written
        let len = bs58::decode(value)
            .onto(&mut buffer)
            .map_err(|err| match err {
                bs58::decode::Error::BufferTooSmall => ParseSubgraphIdError::InvalidLength {
                    value: value.to_string(),
                    length: value.len(),
                },
                bs58::decode::Error::InvalidCharacter { .. } => {
                    ParseSubgraphIdError::InvalidCharacter {
                        value: value.to_string(),
                        error: err.to_string(),
                    }
                }
                bs58::decode::Error::NonAsciiCharacter { .. } => {
                    ParseSubgraphIdError::InvalidCharacter {
                        value: value.to_string(),
                        error: err.to_string(),
                    }
                }
                _ => unreachable!(),
            })?;

        // If the decoded hash is not 32 bytes long, rotate it to the right so the zero bytes
        // are at the beginning of the array.
        if len < 32 {
            buffer.rotate_right(32 - len);
        }

        Ok(Self::from(buffer))
    }
}

impl std::fmt::Display for SubgraphId {
    /// Format the `SubgraphId` as a base58-encoded string.
    ///
    /// ```rust
    /// # use thegraph_core::{subgraph_id, SubgraphId};
    /// const ID: SubgraphId = subgraph_id!("DZz4kDTdmzWLWsV373w2bSmoar3umKKH9y82SUKr5qmp");
    ///
    /// assert_eq!(format!("{}", ID), "DZz4kDTdmzWLWsV373w2bSmoar3umKKH9y82SUKr5qmp");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let leading_zeroes = self.0.iter().take_while(|b| **b == 0).count();
        f.write_str(&bs58::encode(&self.0[leading_zeroes..]).into_string())
    }
}

impl std::fmt::Debug for SubgraphId {
    /// Format the `SubgraphId` as a debug string.
    ///
    /// ```rust
    /// # use thegraph_core::{subgraph_id, SubgraphId};
    /// const ID: SubgraphId = subgraph_id!("DZz4kDTdmzWLWsV373w2bSmoar3umKKH9y82SUKr5qmp");
    ///
    /// assert_eq!(format!("{:?}", ID), "SubgraphId(DZz4kDTdmzWLWsV373w2bSmoar3umKKH9y82SUKr5qmp)");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SubgraphId({})", self)
    }
}

#[cfg(feature = "fake")]
/// To use the [`fake`] crate to generate random [`SubgraphId`] values, **the `fake` feature must
/// be enabled.**
///
/// ```rust
/// # use thegraph_core::SubgraphId;
/// # use fake::Fake;
/// let subgraph_id = fake::Faker.fake::<SubgraphId>();
///
/// println!("SubgraphId: {}", subgraph_id);
/// ```
impl fake::Dummy<fake::Faker> for SubgraphId {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(config: &fake::Faker, rng: &mut R) -> Self {
        <[u8; 32]>::dummy_with_rng(config, rng).into()
    }
}

/// Converts a sequence of string literals containing 32-bytes Base58-encoded data into a new
/// [`SubgraphId`] at compile time.
///
/// To create an `SubgraphId` from a string literal (Base58) at compile time:
///
/// ```rust
/// # use thegraph_core::{subgraph_id, SubgraphId};
/// const SUBGRAPH_ID: SubgraphId = subgraph_id!("DZz4kDTdmzWLWsV373w2bSmoar3umKKH9y82SUKr5qmp");
/// ```
///
/// If no argument is provided, the macro will create an `SubgraphId` with the zero ID:
///
/// ```rust
/// # use thegraph_core::{subgraph_id, SubgraphId};
/// const SUBGRAPH_ID: SubgraphId = subgraph_id!();
///
/// assert_eq!(SUBGRAPH_ID, SubgraphId::ZERO);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __subgraph_id {
    () => {
        $crate::SubgraphId::ZERO
    };
    ($id:tt) => {
        $crate::SubgraphId::new($crate::__parse_subgraph_id_const($id))
    };
}

/// Parse a base58-encoded string into a 32-bytes array at compile time.
#[doc(hidden)]
pub const fn __parse_subgraph_id_const(value: &str) -> B256 {
    let data = value.as_bytes();
    let bytes = bs58::decode(data).into_array_const_unwrap::<32>();
    B256::new(bytes)
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{B256, b256};

    use super::{ParseSubgraphIdError, SubgraphId};
    use crate::subgraph_id;

    const VALID_SUBGRAPH_ID: &str = "7xB3yxxD8okmq4dZPky3eP1nYRgLfZrwMyUQBGo32t4U";

    const EXPECTED_ID_BYTES: B256 =
        b256!("67486e65165b1474898247760a4b852d70d95782c6325960e5b6b4fd82fed1bd");

    const EXPECTED_ID: SubgraphId = subgraph_id!(VALID_SUBGRAPH_ID);

    #[test]
    fn parse_valid_string() {
        //* Given
        let valid_id = VALID_SUBGRAPH_ID;

        //* When
        let result = valid_id.parse::<SubgraphId>();

        //* Then
        let id = result.expect("invalid subgraph ID");
        assert_eq!(id, EXPECTED_ID);
        assert_eq!(id.0, EXPECTED_ID_BYTES);
    }

    #[test]
    fn parse_failure_on_invalid_string() {
        //* Given
        // The following string is not a valid base58 string as it contains the `l` character
        let invalid_id = "invalid";

        //* When
        let result = invalid_id.parse::<SubgraphId>();

        //* Then
        let err = result.expect_err("expected an error");
        assert_eq!(
            err,
            ParseSubgraphIdError::InvalidCharacter {
                value: invalid_id.to_string(),
                error: "provided string contained invalid character 'l' at byte 4".to_string(),
            }
        );
    }

    #[test]
    fn format_subgraph_id_display() {
        //* Given
        let valid_id = EXPECTED_ID;

        //* When
        let result = format!("{}", valid_id);

        //* Then
        assert_eq!(result, VALID_SUBGRAPH_ID);
    }

    #[test]
    fn format_subgraph_id_debug() {
        //* Given
        let expected_debug_repr = format!("SubgraphId({})", VALID_SUBGRAPH_ID);

        let valid_id = EXPECTED_ID;

        //* When
        let result = format!("{:?}", valid_id);

        //* Then
        assert_eq!(result, expected_debug_repr);
    }

    #[test]
    fn serialize_leading_zeroes() {
        let input = "4JruhWH1ZdwvUuMg2xCmtnZQYYHvmEq6cmTcZkpM6pW";
        let output: SubgraphId = input.parse().unwrap();
        assert_eq!(output.to_string(), input.to_string());
    }
}
