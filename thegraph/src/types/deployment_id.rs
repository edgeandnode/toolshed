use alloy_primitives::B256;
use async_graphql::Scalar;
use serde_with::{DeserializeFromStr, SerializeDisplay};

/// A Subgraph's Deployment ID represents unique identifier for a deployed subgraph on The Graph.
/// This is the content ID of the subgraph's manifest.
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub struct DeploymentId(pub B256);

fn parse_cidv0(value: &str) -> Result<B256, DeploymentIdError> {
    if value.len() != 46 {
        return Err(DeploymentIdError::InvalidIpfsHashLength {
            value: value.to_string(),
            length: value.len(),
        });
    }

    let mut decoded = [0_u8; 34];
    bs58::decode(value)
        .onto(&mut decoded)
        .map_err(|e| DeploymentIdError::InvalidIpfsHash {
            value: value.to_string(),
            error: e,
        })?;
    let mut bytes = [0_u8; 32];
    bytes.copy_from_slice(&decoded[2..]);

    Ok(bytes.into())
}

/// Attempt to parse a 32-byte hex string.
fn parse_hexstr(value: &str) -> Result<B256, DeploymentIdError> {
    value
        .parse::<B256>()
        .map_err(|e| DeploymentIdError::InvalidHexString {
            value: value.to_string(),
            error: format!("{}", e),
        })
}

/// Format bytes as a CIDv0.
fn format_cidv0(bytes: B256) -> String {
    let mut buf = [0_u8; 34];
    buf[0..2].copy_from_slice(&[0x12, 0x20]);
    buf[2..].copy_from_slice(bytes.as_slice());
    bs58::encode(buf).into_string()
}

/// Subgraph deployment ID parsing error.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DeploymentIdError {
    /// Invalid IPFS hash length. The input string must 46 characters long.
    #[error("invalid IPFS / CIDv0 hash length {length}: {value} (length must be 46)")]
    InvalidIpfsHashLength { value: String, length: usize },

    /// Invalid IPFS hash format. The input hash string could not be decoded as a CIDv0.
    #[error("invalid IPFS / CIDv0 hash \"{value}\": {error}")]
    InvalidIpfsHash {
        value: String,
        error: bs58::decode::Error,
    },

    /// Invalid hex string format. The input hex string could not be decoded.
    #[error("invalid hex string \"{value}\": {error}")]
    InvalidHexString { value: String, error: String },
}

impl std::str::FromStr for DeploymentId {
    type Err = DeploymentIdError;

    /// Parse a deployment ID from a 32-byte hex string or a base58-encoded IPFS hash (CIDv0).
    fn from_str(hash: &str) -> Result<Self, Self::Err> {
        if hash.starts_with("Qm") {
            // Attempt to decode IPFS hash (CIDv0)
            Ok(Self(parse_cidv0(hash)?))
        } else {
            // Attempt to decode 32-byte hex string
            Ok(Self(parse_hexstr(hash)?))
        }
    }
}

impl std::fmt::Display for DeploymentId {
    /// Encode the deployment ID as CIDv0 (base58-encoded sha256-hash).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format_cidv0(self.0))
    }
}

impl std::fmt::Debug for DeploymentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::fmt::LowerHex for DeploymentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.0, f)
    }
}

#[Scalar]
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy_primitives::B256;
    use assert_matches::assert_matches;

    use super::{format_cidv0, parse_cidv0, parse_hexstr, DeploymentId, DeploymentIdError};

    const VALID_CID: &str = "QmWmyoMoctfbAaiEs2G46gpeUmhqFRDW6KWo64y5r581Vz";
    const VALID_HEX: &str = "0x7d5a99f603f231d53a4f39d1521f98d2e8bb279cf29bebfd0687dc98458e7f89";

    #[test]
    fn parse_valid_cidv0() {
        //// Given
        let valid_cid = VALID_CID;
        let expected_bytes = VALID_HEX.parse::<B256>().unwrap();

        //// When
        let parsed_id = parse_cidv0(valid_cid);

        //// Then
        assert_matches!(parsed_id, Ok(id) => {
            assert_eq!(id, expected_bytes);
        });
    }

    #[test]
    fn parse_invalid_lenght_cidv0() {
        //// Given
        let invalid_cid = "QmA";

        //// When
        let parsed_id = parse_cidv0(invalid_cid);

        //// Then
        assert_matches!(parsed_id, Err(err) => {
            assert_eq!(err, DeploymentIdError::InvalidIpfsHashLength {
                value: invalid_cid.to_string(),
                length: invalid_cid.len(),
            });
        });
    }

    #[test]
    fn parse_invalid_encoding_cidv0() {
        //// Given
        let invalid_cid = "QmfVqZ9gPyMdU6TznRUh+Y0ui7J5zym+v9BofcmEWOf4k=";

        //// When
        let parsed_id = parse_cidv0(invalid_cid);

        //// Then
        assert_matches!(parsed_id, Err(err) => {
            assert_eq!(err, DeploymentIdError::InvalidIpfsHash {
                value: invalid_cid.to_string(),
                error: bs58::decode::Error::InvalidCharacter {
                    character: '+',
                    index: 20,
                },
            });
        });
    }

    #[test]
    fn parse_valid_hexstr() {
        //// Given
        let valid_hex = VALID_HEX;
        let expected_bytes = VALID_HEX.parse::<B256>().unwrap();

        //// When
        let parsed_id = parse_hexstr(valid_hex);

        //// Then
        assert_matches!(parsed_id, Ok(id) => {
            assert_eq!(id, expected_bytes);
        });
    }

    #[test]
    fn parse_invalid_hexstr() {
        //// Given
        let invalid_hex = "0x0123456789ABCDEF";

        //// When
        let parsed_id = parse_hexstr(invalid_hex);

        //// Then
        assert_matches!(parsed_id, Err(err) => {
            assert_eq!(err, DeploymentIdError::InvalidHexString {
                value: invalid_hex.to_string(),
                error: "Invalid string length".to_string(),
            });
        });
    }

    #[test]
    fn format_into_cidv0() {
        //// Given
        let bytes = VALID_HEX.parse::<B256>().unwrap();
        let expected_cid = VALID_CID;

        //// When
        let cid = format_cidv0(bytes);

        //// Then
        assert_eq!(cid, expected_cid);
    }

    #[test]
    fn deployment_id_equality() {
        //// Given
        let valid_cid = VALID_CID;
        let valid_hex = VALID_HEX;

        let expected_id = DeploymentId(VALID_HEX.parse().unwrap());
        let expected_repr = VALID_CID;

        //// When
        let parsed_id1 = DeploymentId::from_str(valid_cid);
        let parsed_id2 = DeploymentId::from_str(valid_hex);

        //// Then
        assert_matches!((parsed_id1, parsed_id2), (Ok(id1), Ok(id2)) => {
            // Assert the two IDs internal representation is correct
            assert_eq!(id1, expected_id);
            assert_eq!(id2, expected_id);

            // Assert the two IDs are equal and displayed in CIDv0 format
            assert_eq!(id1, id2);
            assert_eq!(id1.to_string(), expected_repr);
            assert_eq!(id2.to_string(), expected_repr);
        });
    }
}
