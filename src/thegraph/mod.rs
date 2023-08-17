use std::{fmt, str::FromStr};

use ethers_core::{
    abi::Hash,
    types::{Address, H256},
};
use serde::Deserialize;
use sha3::{
    digest::{Digest as _, Update as _},
    Keccak256,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockPointer {
    pub number: u64,
    pub hash: Hash,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubgraphId(pub H256);

impl FromStr for SubgraphId {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_v1(s: &str) -> Option<H256> {
            // Attempt to decode v1 format: '0x' <hex account_id> '-' <decimal sequence_id>
            let (account_id, sequence_id) = s.split_once('-')?;
            let account: Address = account_id.parse().ok()?;
            // Assuming u256 big-endian, since that's the word-size of the EVM
            let mut sequence_word = [0_u8; 32];
            let sequence_number = sequence_id.parse::<u64>().ok()?.to_be_bytes();
            sequence_word[24..].copy_from_slice(&sequence_number);
            let hash: [u8; 32] = Keccak256::default()
                .chain(account.as_ref())
                .chain(sequence_word)
                .finalize()
                .into();
            Some(hash.into())
        }
        fn parse_v2(s: &str) -> Option<H256> {
            // Attempt to decode v2 format: base58 of sha256 hash
            let mut hash = [0_u8; 32];
            let len = bs58::decode(s).onto(&mut hash).ok()?;
            if len < hash.len() {
                return None;
            }
            hash.rotate_right(32 - len);
            Some(hash.into())
        }
        if let Some(v2) = parse_v2(s) {
            return Ok(Self(v2));
        }
        parse_v1(s).map(Self).ok_or("invalid subgraph ID")
    }
}

impl fmt::Display for SubgraphId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&bs58::encode(self.0.as_bytes()).into_string())
    }
}

impl fmt::Debug for SubgraphId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// subgraph deployment hash, encoded/decoded using its CIDv0 representation
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeploymentId(pub H256);

impl FromStr for DeploymentId {
    type Err = bs58::decode::Error;
    fn from_str(cid_v0: &str) -> Result<Self, Self::Err> {
        let mut decoded = [0_u8; 34];
        bs58::decode(cid_v0).onto(&mut decoded)?;
        let mut bytes = [0_u8; 32];
        bytes.copy_from_slice(&decoded[2..]);
        Ok(Self(bytes.into()))
    }
}

impl fmt::Display for DeploymentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0_u8; 34];
        buf[0..2].copy_from_slice(&[0x12, 0x20]);
        buf[2..].copy_from_slice(self.0.as_bytes());
        f.write_str(&bs58::encode(buf).into_string())
    }
}

impl fmt::Debug for DeploymentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[test]
fn subgraph_id_encode() {
    let bytes: H256 = "0x67486e65165b1474898247760a4b852d70d95782c6325960e5b6b4fd82fed1bd"
        .parse()
        .unwrap();
    let v1 = "0xdeadbeef678b513255cea949017921c8c9f6ef82-1";
    let v2 = "7xB3yxxD8okmq4dZPky3eP1nYRgLfZrwMyUQBGo32t4U";

    let id1: SubgraphId = v1.parse().unwrap();
    let id2: SubgraphId = v2.parse().unwrap();

    assert_eq!(id1.0, bytes);
    assert_eq!(&id1.to_string(), v2);
    assert_eq!(id2.0, bytes);
    assert_eq!(&id2.to_string(), v2);
    assert_eq!(id1, id2);
}

#[test]
fn deployment_id_encode() {
    let ipfs_hash = "QmWmyoMoctfbAaiEs2G46gpeUmhqFRDW6KWo64y5r581Vz";
    let hash: H256 = "0x7d5a99f603f231d53a4f39d1521f98d2e8bb279cf29bebfd0687dc98458e7f89"
        .parse()
        .unwrap();

    let id1 = DeploymentId(hash);
    let id2: DeploymentId = ipfs_hash
        .parse()
        .expect("failed to create DeploymentId from CIDv0");

    assert_eq!(id1.to_string(), ipfs_hash);
    assert_eq!(id1.0, hash);
    assert_eq!(id2.to_string(), ipfs_hash);
    assert_eq!(id2.0, hash);
    assert_eq!(id1, id2);
}
