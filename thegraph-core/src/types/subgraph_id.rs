//! Subgraph ID type and related utilities.

use alloy_primitives::B256;

/// A Subgraph ID is a 32-byte identifier for a subgraph.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
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
}

impl From<B256> for SubgraphId {
    fn from(bytes: B256) -> Self {
        Self(bytes)
    }
}

impl From<[u8; 32]> for SubgraphId {
    fn from(value: [u8; 32]) -> Self {
        Self(B256::from(value))
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

impl AsRef<B256> for SubgraphId {
    fn as_ref(&self) -> &B256 {
        &self.0
    }
}

impl std::str::FromStr for SubgraphId {
    type Err = &'static str;

    /// Attempt to parse a Subgraph ID in v2 format: `base58(sha256(<subgraph_id>))`
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut buffer = [0_u8; 32];

        // Decode the base58 string into a byte array, and get the number of bytes written
        let len = bs58::decode(value)
            .onto(&mut buffer)
            .map_err(|_| "invalid subgraph ID")?;

        // If the decoded hash is not 32 bytes long, rotate it to the right so the zero bytes
        // are at the beginning of the array.
        if len < 32 {
            buffer.rotate_right(32 - len);
        }

        Ok(Self::from(buffer))
    }
}

impl std::fmt::Display for SubgraphId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&bs58::encode(self.0.as_slice()).into_string())
    }
}

impl std::fmt::Debug for SubgraphId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Subgraph({})", self)
    }
}

/// Converts a sequence of string literals containing 32-bytes Base58-encoded data into a new
/// [`SubgraphId`] at compile time.
///
/// To create an `SubgraphId` from a string literal (Base58) at compile time:
///
/// ```rust
/// use thegraph_core::subgraph_id;
/// use thegraph_core::types::SubgraphId;
///
/// const SUBGRAPH_ID: SubgraphId = subgraph_id!("DZz4kDTdmzWLWsV373w2bSmoar3umKKH9y82SUKr5qmp");
/// ```
///
/// If no argument is provided, the macro will create an `SubgraphId` with the zero ID:
///
/// ```rust
/// use thegraph_core::subgraph_id;
/// use thegraph_core::types::SubgraphId;
///
/// const SUBGRAPH_ID: SubgraphId = subgraph_id!();
///
/// assert_eq!(SUBGRAPH_ID, SubgraphId::ZERO);
/// ```
#[macro_export]
macro_rules! subgraph_id {
    () => {
        $crate::types::SubgraphId::new($crate::alloy_primitives::B256::ZERO)
    };
    ($id:tt) => {
        $crate::types::SubgraphId::new($crate::types::parse_subgraph_id_const($id))
    };
}

/// Parse a base58-encoded string into a 32-bytes array at compile time.
#[doc(hidden)]
pub const fn parse_subgraph_id_const(value: &str) -> B256 {
    let data = value.as_bytes();
    let bytes = bs58::decode(data).into_array_const_unwrap::<32>();
    B256::new(bytes)
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{b256, B256};

    use super::SubgraphId;

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
        let err = result.expect_err("invalid subgraph ID");
        assert_eq!(err, "invalid subgraph ID");
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
        let expected_debug_repr = format!("Subgraph({})", VALID_SUBGRAPH_ID);

        let valid_id = EXPECTED_ID;

        //* When
        let result = format!("{:?}", valid_id);

        //* Then
        assert_eq!(result, expected_debug_repr);
    }
}
