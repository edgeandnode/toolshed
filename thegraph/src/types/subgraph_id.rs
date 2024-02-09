use serde_with::{DeserializeFromStr, SerializeDisplay};

use super::primitives::B256;

#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub struct SubgraphId(B256);

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
        let mut hash = [0_u8; 32];
        let len = bs58::decode(value)
            .onto(&mut hash)
            .map_err(|_| "invalid subgraph ID")?;
        hash.rotate_right(32 - len);
        Ok(Self(hash.into()))
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

    use super::SubgraphId;

    const ID_V2: &str = "7xB3yxxD8okmq4dZPky3eP1nYRgLfZrwMyUQBGo32t4U";

    const EXPECTED_ID_BYTES: &str =
        "0x67486e65165b1474898247760a4b852d70d95782c6325960e5b6b4fd82fed1bd";

    #[test]
    fn parse_valid_v2_id() {
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
        let expected_id = SubgraphId(EXPECTED_ID_BYTES.parse().unwrap());
        let expected_repr = ID_V2;

        //// When
        let parsed_id = ID_V2.parse::<SubgraphId>();

        //// Then
        assert_matches!(parsed_id, Ok(id) => {
            // Assert the IDs internal representation is correct
            assert_eq!(id, expected_id);

            // Assert the ID is displayed in v2 format
            assert_eq!(id.to_string(), expected_repr);
        });
    }
}
