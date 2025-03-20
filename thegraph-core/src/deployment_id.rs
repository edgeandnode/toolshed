use alloy::{hex, primitives::B256};

/// Subgraph deployment ID parsing error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseDeploymentIdError {
    /// Invalid IPFS hash length. The input string must 46 characters long.
    #[error("invalid IPFS / CIDv0 hash length {length}: {value} (length must be 46)")]
    InvalidIpfsHashLength { value: String, length: usize },

    /// Invalid IPFS hash format. The input hash string could not be decoded as a CIDv0.
    #[error("invalid IPFS hash \"{value}\": {error}")]
    InvalidIpfsHash { value: String, error: String },

    /// Invalid hex string format. The input hex string could not be decoded.
    #[error("invalid hex string \"{value}\": {error}")]
    InvalidHexString { value: String, error: String },
}

impl From<hex::FromHexError> for ParseDeploymentIdError {
    fn from(err: hex::FromHexError) -> Self {
        ParseDeploymentIdError::InvalidHexString {
            value: String::new(),
            error: format!("{}", err),
        }
    }
}

/// A Subgraph's Deployment ID represents unique identifier for a deployed subgraph on The Graph.
///
/// This is the content ID of the subgraph's manifest.
///
/// ## Generating test data
///
/// The `DeploymentId` type implements the [`fake`] crate's [`fake::Dummy`] trait, allowing you to
/// generate random `DeploymentId` values for testing.
///
/// Note that the `fake` feature must be enabled to use this functionality.
///
/// See the [`Dummy`] trait impl for usage examples.
///
/// [`Dummy`]: #impl-Dummy<Faker>-for-DeploymentId
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
#[repr(transparent)]
pub struct DeploymentId(B256);

impl DeploymentId {
    /// The "zero" [`DeploymentId`].
    ///
    /// This is a constant value that represents the zero ID. It is equivalent to parsing a zeroed
    /// 32-byte array.
    pub const ZERO: Self = Self(B256::ZERO);

    /// Create a new [`DeploymentId`].
    pub const fn new(bytes: B256) -> Self {
        Self(bytes)
    }

    /// Get the bytes of the [`DeploymentId`] as a slice.
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_ref()
    }
}

impl AsRef<B256> for DeploymentId {
    fn as_ref(&self) -> &B256 {
        &self.0
    }
}

impl AsRef<[u8]> for DeploymentId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<[u8; 32]> for DeploymentId {
    fn as_ref(&self) -> &[u8; 32] {
        self.0.as_ref()
    }
}

impl std::borrow::Borrow<[u8]> for DeploymentId {
    fn borrow(&self) -> &[u8] {
        self.0.borrow()
    }
}

impl std::borrow::Borrow<[u8; 32]> for DeploymentId {
    fn borrow(&self) -> &[u8; 32] {
        self.0.borrow()
    }
}

impl std::ops::Deref for DeploymentId {
    type Target = B256;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<B256> for DeploymentId {
    fn from(bytes: B256) -> Self {
        Self(bytes)
    }
}

impl From<[u8; 32]> for DeploymentId {
    fn from(value: [u8; 32]) -> Self {
        Self(value.into())
    }
}

impl<'a> From<&'a [u8; 32]> for DeploymentId {
    fn from(value: &'a [u8; 32]) -> Self {
        Self(value.into())
    }
}

impl<'a> TryFrom<&'a [u8]> for DeploymentId {
    type Error = <B256 as TryFrom<&'a [u8]>>::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        value.try_into().map(Self)
    }
}

impl From<DeploymentId> for B256 {
    fn from(id: DeploymentId) -> Self {
        id.0
    }
}

impl From<&DeploymentId> for B256 {
    fn from(id: &DeploymentId) -> Self {
        id.0
    }
}

impl std::str::FromStr for DeploymentId {
    type Err = ParseDeploymentIdError;

    /// Parse a deployment ID from a 32-byte hex string or a base58-encoded IPFS hash (CIDv0).
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.starts_with("Qm") {
            // Attempt to decode base58-encoded CIDv0
            parse_cid_v0_str(value)
        } else {
            // Attempt to decode 32-byte hex string
            hex::FromHex::from_hex(value).map_err(Into::into)
        }
    }
}

impl hex::FromHex for DeploymentId {
    type Error = hex::FromHexError;

    /// Parse a deployment ID from a 32-byte hex string.
    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        B256::from_hex(hex).map(Self)
    }
}

impl std::fmt::Display for DeploymentId {
    /// Format the `DeploymentId` as CIDv0 (base58-encoded sha256-hash) string.
    ///
    /// ```rust
    /// # use thegraph_core::{deployment_id, DeploymentId};
    /// const ID: DeploymentId = deployment_id!("QmSWxvd8SaQK6qZKJ7xtfxCCGoRzGnoi2WNzmJYYJW9BXY");
    ///
    /// assert_eq!(format!("{}", ID), "QmSWxvd8SaQK6qZKJ7xtfxCCGoRzGnoi2WNzmJYYJW9BXY");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format_cid_v0(self.0.as_slice()))
    }
}

impl std::fmt::Debug for DeploymentId {
    /// Format the `DeploymentId` as a debug string.
    ///
    /// ```rust
    /// # use thegraph_core::{deployment_id, DeploymentId};
    /// const ID: DeploymentId = deployment_id!("QmSWxvd8SaQK6qZKJ7xtfxCCGoRzGnoi2WNzmJYYJW9BXY");
    ///
    /// assert_eq!(
    ///     format!("{:?}", ID),
    ///     "DeploymentId(QmSWxvd8SaQK6qZKJ7xtfxCCGoRzGnoi2WNzmJYYJW9BXY)",
    /// );
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeploymentId({})", self)
    }
}

impl std::fmt::LowerHex for DeploymentId {
    /// Format the `DeploymentId` as a 32-byte hex string.
    ///
    /// Note that the alternate flag, `#`, adds a `0x` in front of the output.
    ///
    /// ```rust
    /// # use thegraph_core::{deployment_id, DeploymentId};
    /// const ID: DeploymentId = deployment_id!("QmWmyoMoctfbAaiEs2G46gpeUmhqFRDW6KWo64y5r581Vz");
    ///
    /// // Lower hex
    /// assert_eq!(
    ///     format!("{:x}", ID),
    ///     "7d5a99f603f231d53a4f39d1521f98d2e8bb279cf29bebfd0687dc98458e7f89"
    /// );
    ///
    /// // Lower hex with alternate flag
    /// assert_eq!(
    ///     format!("{:#x}", ID),
    ///     "0x7d5a99f603f231d53a4f39d1521f98d2e8bb279cf29bebfd0687dc98458e7f89"
    /// );
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.0, f)
    }
}

#[cfg(feature = "fake")]
/// To use the [`fake`] crate to generate random [`DeploymentId`] values, **the `fake` feature must
/// be enabled.**
///
/// ```rust
/// # use thegraph_core::DeploymentId;
/// # use fake::Fake;
/// let deployment_id = fake::Faker.fake::<DeploymentId>();
///
/// println!("DeploymentId: {}", deployment_id);
/// ```
impl fake::Dummy<fake::Faker> for DeploymentId {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(config: &fake::Faker, rng: &mut R) -> Self {
        <[u8; 32]>::dummy_with_rng(config, rng).into()
    }
}

#[cfg(feature = "async-graphql")]
#[async_graphql::Scalar]
impl async_graphql::ScalarType for DeploymentId {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        if let async_graphql::Value::String(value) = &value {
            Ok(value.parse::<DeploymentId>()?)
        } else {
            Err(async_graphql::InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        // Convert to CIDv0 (Qm... base58-encoded sha256-hash)
        async_graphql::Value::String(self.to_string())
    }
}

/// Format bytes as a CIDv0 string.
///
/// The CIDv0 format is a base58-encoded sha256-hash with a prefix of `Qm`
fn format_cid_v0(bytes: &[u8]) -> String {
    let mut buf = [0_u8; 34];
    buf[0..2].copy_from_slice(&[0x12, 0x20]);
    buf[2..].copy_from_slice(bytes);
    bs58::encode(buf).into_string()
}

fn parse_cid_v0_str(value: &str) -> Result<DeploymentId, ParseDeploymentIdError> {
    // Check if the string has a valid length for a CIDv0 (46 characters)
    if value.len() != 46 {
        return Err(ParseDeploymentIdError::InvalidIpfsHashLength {
            value: value.to_string(),
            length: value.len(),
        });
    }

    // Decode the base58-encoded CIDv0
    let mut buffer = [0_u8; 34];
    bs58::decode(value)
        .onto(&mut buffer)
        .map_err(|e| ParseDeploymentIdError::InvalidIpfsHash {
            value: value.to_string(),
            error: e.to_string(),
        })?;

    // Extract the 32-byte hash from the buffer
    let mut bytes = [0_u8; 32];
    bytes.copy_from_slice(&buffer[2..]);

    Ok(DeploymentId::new(B256::new(bytes)))
}

/// Converts a sequence of string literals containing CIDv0 data into a new [`DeploymentId`] at
/// compile time.
///
/// To create an `DeploymentId` from a string literal (Base58) at compile time:
///
/// ```rust
/// # use thegraph_core::{deployment_id, DeploymentId};
/// const DEPLOYMENT_ID: DeploymentId = deployment_id!("QmSWxvd8SaQK6qZKJ7xtfxCCGoRzGnoi2WNzmJYYJW9BXY");
/// ```
///
/// If no argument is provided, the macro will create an `DeploymentId` with the zero ID:
///
/// ```rust
/// # use thegraph_core::{deployment_id, DeploymentId};
/// const DEPLOYMENT_ID: DeploymentId = deployment_id!();
///
/// assert_eq!(DEPLOYMENT_ID, DeploymentId::ZERO);
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __deployment_id {
    () => {
        $crate::DeploymentId::ZERO
    };
    ($id:tt) => {
        $crate::DeploymentId::new($crate::__parse_cid_v0_const($id))
    };
}

/// Parse a CIDv0 string into a 32-byte hash.
#[doc(hidden)]
pub const fn __parse_cid_v0_const(value: &str) -> B256 {
    // Check if the string has a valid length for a CIDv0 (46 characters)
    if value.len() != 46 {
        panic!("invalid string length (length must be 46)");
    }

    // Check if the string starts with "Qm"
    let data = value.as_bytes();
    if data[0] != b'Q' || data[1] != b'm' {
        panic!("provided string does not start with 'Qm'");
    }

    // Decode the base58-encoded CIDv0 (34 bytes)
    let decoded = bs58::decode(data).into_array_const_unwrap::<34>();

    // Extract the 32-byte hash from the buffer
    // Perform bytes.copy_from_slice(&decoded[2..]) in a const fn context
    let mut bytes = [0_u8; 32];
    let mut i = 0;
    while i < 32 {
        bytes[i] = decoded[i + 2];
        i += 1;
    }
    B256::new(bytes)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy::primitives::{B256, b256};

    use super::{DeploymentId, ParseDeploymentIdError, format_cid_v0, parse_cid_v0_str};
    use crate::deployment_id;

    const VALID_CID: &str = "QmWmyoMoctfbAaiEs2G46gpeUmhqFRDW6KWo64y5r581Vz";
    const VALID_HEX: &str = "0x7d5a99f603f231d53a4f39d1521f98d2e8bb279cf29bebfd0687dc98458e7f89";
    const EXPECTED_DEPLOYMENT_ID: DeploymentId = deployment_id!(VALID_CID);
    const EXPECTED_DEPLOYMENT_BYTES: B256 =
        b256!("7d5a99f603f231d53a4f39d1521f98d2e8bb279cf29bebfd0687dc98458e7f89");

    #[test]
    fn parse_valid_cid_v0() {
        //* Given
        let valid_cid = VALID_CID;

        //* When
        let result = parse_cid_v0_str(valid_cid);

        //* Then
        let id = result.expect("expected a valid ID");
        assert_eq!(id, EXPECTED_DEPLOYMENT_ID);
        assert_eq!(id.0, EXPECTED_DEPLOYMENT_BYTES);
    }

    #[test]
    fn parse_invalid_length_cid_v0() {
        //* Given
        let invalid_cid = "QmA";

        //* When
        let result = parse_cid_v0_str(invalid_cid);

        //* Then
        let err = result.expect_err("expected an error");
        assert_eq!(
            err,
            ParseDeploymentIdError::InvalidIpfsHashLength {
                value: invalid_cid.to_string(),
                length: invalid_cid.len(),
            }
        );
    }

    #[test]
    fn parse_invalid_base58_character_cid_v0() {
        //* Given
        let invalid_cid = "QmfVqZ9gPyMdU6TznRUh+Y0ui7J5zym+v9BofcmEWOf4k=";

        //* When
        let result = parse_cid_v0_str(invalid_cid);

        //* Then
        let err = result.expect_err("expected an error");
        assert_eq!(
            err,
            ParseDeploymentIdError::InvalidIpfsHash {
                value: invalid_cid.to_string(),
                error: bs58::decode::Error::InvalidCharacter {
                    character: '+',
                    index: 20,
                }
                .to_string(),
            }
        );
    }

    #[test]
    fn format_into_cid_v0() {
        //* Given
        let expected_str = VALID_CID;

        let bytes = EXPECTED_DEPLOYMENT_BYTES.as_slice();

        //* When
        let cid = format_cid_v0(bytes);

        //* Then
        assert_eq!(cid, expected_str);
    }

    #[test]
    fn format_deployment_id_display() {
        //* Given
        let expected_str = VALID_CID;

        let valid_id = EXPECTED_DEPLOYMENT_ID;

        //* When
        let result_str = format!("{}", valid_id);

        //* Then
        assert_eq!(result_str, expected_str);
    }

    #[test]
    fn format_deployment_id_lower_hex() {
        //* Given
        let expected_str = VALID_HEX;

        let valid_id = EXPECTED_DEPLOYMENT_ID;

        //* When
        // The alternate flag, #, adds a 0x in front of the output
        let result_str = format!("{:#x}", valid_id);

        //* Then
        assert_eq!(result_str, expected_str);
    }

    #[test]
    fn format_deployment_id_debug() {
        //* Given
        let expected_str = format!("DeploymentId({})", VALID_CID);

        let valid_id = EXPECTED_DEPLOYMENT_ID;

        //* When
        let result_str = format!("{:?}", valid_id);

        //* Then
        assert_eq!(result_str, expected_str);
    }

    #[test]
    fn deployment_id_equality() {
        //* Given
        let expected_id = deployment_id!(VALID_CID);
        let expected_repr = VALID_CID;

        let valid_cid = VALID_CID;
        let valid_hex = VALID_HEX;

        //* When
        let result_cid = DeploymentId::from_str(valid_cid);
        let result_hex = DeploymentId::from_str(valid_hex);

        //* Then
        let id_cid = result_cid.expect("expected a valid ID");
        let id_hex = result_hex.expect("expected a valid ID");

        // Assert the two IDs internal representation is correct
        assert_eq!(id_cid, expected_id);
        assert_eq!(id_hex, expected_id);

        // Assert the two IDs CIDv0 representation is correct
        assert_eq!(id_cid.to_string(), expected_repr);
        assert_eq!(id_hex.to_string(), expected_repr);

        // Assert both IDs are equal
        assert_eq!(id_cid, id_hex);
    }
}
