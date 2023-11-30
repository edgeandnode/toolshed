use alloy_primitives::Address;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use sha3::{Digest as _, Keccak256};

use super::primitives::B256;

/// Attempt to parse a Subgraph ID in v1 format:
/// ```text
/// 0x <hex account_id> - <decimal sequence_id>
/// ```
fn parse_v1(value: &str) -> Option<B256> {
    let (account_id, sequence_id) = value.split_once('-')?;
    let account = account_id.parse::<Address>().ok()?;

    // Assuming u256 big-endian, since that's the word-size of the EVM
    let mut sequence_word = [0_u8; 32];
    let sequence_number = sequence_id.parse::<u64>().ok()?.to_be_bytes();
    sequence_word[24..].copy_from_slice(&sequence_number);

    let hash: [u8; 32] = {
        let mut hasher = Keccak256::default();
        hasher.update(account.0);
        hasher.update(sequence_word);
        hasher.finalize().into()
    };

    Some(hash.into())
}

/// Attempt to parse a Subgraph ID in v2 format:
///
/// ```text
/// base58(sha256(<subgraph_id>))
/// ```
///
/// If the input is not valid base58, or the decoded hash is not 32 bytes, returns `None`.
fn parse_v2(value: &str) -> Option<B256> {
    let mut hash = [0_u8; 32];
    let len = bs58::decode(value).onto(&mut hash).ok()?;
    hash.rotate_right(32 - len);
    Some(hash.into())
}

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub struct SubgraphId(B256);

impl From<B256> for SubgraphId {
    fn from(bytes: B256) -> Self {
        Self(bytes)
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
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(v2) = parse_v2(value) {
            return Ok(Self(v2));
        }
        if let Some(v1) = parse_v1(value) {
            return Ok(Self(v1));
        }
        Err("invalid subgraph ID")
    }
}

impl std::fmt::Display for SubgraphId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&bs58::encode(self.0.as_slice()).into_string())
    }
}

impl std::fmt::Debug for SubgraphId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::B256;
    use assert_matches::assert_matches;

    use super::{parse_v1, parse_v2, SubgraphId};

    const ID_V1: &str = "0xdeadbeef678b513255cea949017921c8c9f6ef82-1";
    const ID_V2: &str = "7xB3yxxD8okmq4dZPky3eP1nYRgLfZrwMyUQBGo32t4U";

    const EXPECTED_ID_BYTES: &str =
        "0x67486e65165b1474898247760a4b852d70d95782c6325960e5b6b4fd82fed1bd";

    #[test]
    fn parse_valid_v1_id() {
        //// Given
        let valid_id = ID_V1;
        let expected_id = EXPECTED_ID_BYTES.parse::<B256>().unwrap();

        //// When
        let parsed_id = parse_v1(valid_id);

        //// Then
        assert_matches!(parsed_id, Some(id) => {
            assert_eq!(id, expected_id);
        });
    }

    #[test]
    fn parse_invalid_v1_id() {
        //// Given
        let invalid_id = ID_V2;

        //// When
        let parsed_id = parse_v1(invalid_id);

        //// Then
        assert_matches!(parsed_id, None);
    }

    #[test]
    fn parse_valid_v2_id() {
        //// Given
        let valid_id = ID_V2;
        let expected_id = EXPECTED_ID_BYTES.parse::<B256>().unwrap();

        //// When
        let parsed_id = parse_v2(valid_id);

        //// Then
        assert_matches!(parsed_id, Some(id) => {
            assert_eq!(id, expected_id);
        });
    }

    #[test]
    fn decode_subgraph_id_from_v1_string() {
        //// Given
        let valid_id = ID_V1;
        let expected_id = EXPECTED_ID_BYTES.parse::<B256>().unwrap();

        //// When
        let parsed_id = valid_id.parse::<SubgraphId>();

        //// Then
        assert_matches!(parsed_id, Ok(id) => {
            assert_eq!(id.0, expected_id);
        });
    }

    #[test]
    fn decode_subgraph_id_from_v2_string() {
        //// Given
        let valid_id = ID_V2;
        let expected_id = EXPECTED_ID_BYTES.parse::<B256>().unwrap();

        //// When
        let parsed_id = valid_id.parse::<SubgraphId>();

        //// Then
        assert_matches!(parsed_id, Ok(id) => {
            assert_eq!(id.0, expected_id);
        });
    }

    #[test]
    fn decode_failure_on_invalid_string() {
        //// Given
        let invalid_id = "invalid";

        //// When
        let parsed_id = invalid_id.parse::<SubgraphId>();

        //// Then
        assert_matches!(parsed_id, Err(err) => {
            assert_eq!(err, "invalid subgraph ID");
        });
    }

    #[test]
    fn subgraph_equality() {
        //// Given
        let valid_v1 = ID_V1;
        let valid_v2 = ID_V2;

        let expected_id = SubgraphId(EXPECTED_ID_BYTES.parse().unwrap());
        let expected_repr = ID_V2;

        //// When
        let parsed_id1 = valid_v1.parse::<SubgraphId>();
        let parsed_id2 = valid_v2.parse::<SubgraphId>();

        //// Then
        assert_matches!((parsed_id1, parsed_id2), (Ok(id1), Ok(id2)) => {
            // Assert the two IDs internal representation is correct
            assert_eq!(id1, expected_id);
            assert_eq!(id2, expected_id);

            // Assert the two IDs are equal and displayed in v2 format
            assert_eq!(id1, id2);
            assert_eq!(id1.to_string(), expected_repr);
            assert_eq!(id2.to_string(), expected_repr);
        });
    }
}
