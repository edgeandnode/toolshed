use alloy_primitives::keccak256;
pub use alloy_sol_types::Eip712Domain;
use alloy_sol_types::SolStruct;
pub use ethers_core::k256::ecdsa::SigningKey;
use ethers_core::types::{RecoveryMessage, Signature};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::deployment_id::DeploymentId;
use super::{Address, FixedBytes, B256, U256};

lazy_static! {
    static ref ATTESTATION_EIP712_DOMAIN_SALT: B256 =
        "a070ffb1cd7409649bf77822cce74495468e06dbfaef09556838bf188679b9c2"
            .parse()
            .expect("invalid eip712 domain salt");
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Attestation {
    #[serde(rename = "requestCID")]
    pub request_cid: B256,
    #[serde(rename = "responseCID")]
    pub response_cid: B256,
    #[serde(rename = "subgraphDeploymentID")]
    pub deployment: B256,
    pub r: B256,
    pub s: B256,
    pub v: u8,
}

alloy_sol_types::sol! {
    struct Receipt {
        bytes32 requestCID;
        bytes32 responseCID;
        bytes32 subgraphDeploymentID;
    }
}

pub fn eip712_domain(chain_id: U256, dispute_manager: Address) -> Eip712Domain {
    Eip712Domain {
        name: Some("Graph Protocol".into()),
        version: Some("0".into()),
        chain_id: Some(chain_id),
        verifying_contract: Some(dispute_manager),
        salt: Some(*ATTESTATION_EIP712_DOMAIN_SALT),
    }
}

pub fn create(
    domain: &Eip712Domain,
    signer: &SigningKey,
    deployment: &DeploymentId,
    request: &str,
    response: &str,
) -> Attestation {
    let msg = Receipt {
        requestCID: keccak256(request),
        responseCID: keccak256(response),
        subgraphDeploymentID: deployment.into(),
    };
    let hash = msg.eip712_signing_hash(domain);
    let (signature, recovery) = signer.sign_prehash_recoverable(&hash.0).unwrap();
    Attestation {
        request_cid: msg.requestCID,
        response_cid: msg.responseCID,
        deployment: deployment.into(),
        r: FixedBytes(signature.r().to_bytes().into()),
        s: FixedBytes(signature.s().to_bytes().into()),
        v: recovery.to_byte(),
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, thiserror::Error)]
pub enum VerificationError {
    #[error("invalid request hash")]
    InvalidRequestHash,
    #[error("invalid response hash")]
    InvalidResponseHash,
    #[error("failed to recover signer")]
    FailedSignerRecovery,
    #[error("recovered signer is not expected")]
    RecoveredSignerNotExpected,
}

pub fn recover_allocation(
    domain: &Eip712Domain,
    attestation: &Attestation,
) -> Result<Address, VerificationError> {
    let msg = Receipt {
        requestCID: attestation.request_cid,
        responseCID: attestation.response_cid,
        subgraphDeploymentID: attestation.deployment,
    };
    let signing_hash: B256 = msg.eip712_signing_hash(domain);
    let signature = Signature {
        r: attestation.r.0.into(),
        s: attestation.s.0.into(),
        v: attestation.v.into(),
    };
    signature
        .recover(RecoveryMessage::Hash(signing_hash.0.into()))
        .map_err(|_| VerificationError::FailedSignerRecovery)
        .map(|bytes| Address::from(bytes.0))
}

pub fn verify(
    domain: &Eip712Domain,
    attestation: &Attestation,
    expected_signer: &Address,
    request: &str,
    response: &str,
) -> Result<(), VerificationError> {
    if attestation.request_cid != keccak256(request) {
        return Err(VerificationError::InvalidRequestHash);
    }
    if attestation.response_cid != keccak256(response) {
        return Err(VerificationError::InvalidResponseHash);
    }

    let signer = recover_allocation(domain, attestation)?;
    if &signer != expected_signer {
        return Err(VerificationError::RecoveredSignerNotExpected);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn domain() -> Eip712Domain {
        eip712_domain(
            U256::from_str_radix("1337", 10).unwrap(),
            "16DEF7E0108A5467A106dbD7537f8591f470342E".parse().unwrap(),
        )
    }

    fn signer() -> (Address, SigningKey) {
        let address = "90f8bf6a479f320ead074411a4b0e7944ea8c9c1"
            .parse::<Address>()
            .unwrap();
        let signing_key = SigningKey::from_slice(
            "4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d"
                .parse::<B256>()
                .unwrap()
                .as_slice(),
        )
        .unwrap();
        (address, signing_key)
    }

    fn deployment() -> DeploymentId {
        "QmeVg9Da6uyBvjUEy5JqCgw2VKdkTxjPvcYuE5riGpkqw1"
            .parse()
            .unwrap()
    }

    #[test]
    fn verify_attestation() {
        let request = "foo";
        let response = "bar";
        let attestation = create(&domain(), &signer().1, &deployment(), request, response);
        let check = verify(&domain(), &attestation, &signer().0, request, response);
        assert_eq!(check, Ok(()));
    }

    /// verify attestation created by old indexer-native module from TS indexer implementation
    #[test]
    fn verfiy_old_attestation() {
        let attestation = Attestation {
            request_cid: "41b1a0649752af1b28b3dc29a1556eee781e4a4c3a1f7f53f90fa834de098c4d"
                .parse()
                .unwrap(),
            response_cid: "435cd288e3694b535549c3af56ad805c149f92961bf84a1c647f7d86fc2431b4"
                .parse()
                .unwrap(),
            deployment: deployment().into(),
            r: "e1fb47e7f0b278d4c88564c3a3b46180e476edcb2b783f253f3eec3b36f8fd4f"
                .parse()
                .unwrap(),
            s: "467a881937edf2faf76e2e497085caf370c9689a1d83b245030757f70a1f64de"
                .parse()
                .unwrap(),
            v: 28,
        };
        let check = verify(&domain(), &attestation, &signer().0, "foo", "bar");
        assert_eq!(check, Ok(()));
    }
}
