//! Attestation types and functions for verifying attestations.

use alloy_primitives::{b256, keccak256, Address, FixedBytes, B256, U256};
use alloy_sol_types::{Eip712Domain, SolStruct};
use ethers_core::{
    k256::ecdsa::SigningKey,
    types::{RecoveryMessage, Signature},
};

use super::deployment_id::DeploymentId;

/// Attestation EIP-712 domain salt
const ATTESTATION_EIP712_DOMAIN_SALT: B256 =
    b256!("a070ffb1cd7409649bf77822cce74495468e06dbfaef09556838bf188679b9c2");

/// An attestation of a request-response pair.
///
/// An attestation is an EIP-712 signature over a request, `request_cid`, and response,
/// `response_cid`, keccak-256 hash, and the subgraph deployment ID, `deployment`, being queried.
///
/// The attestation is signed by the indexer that processed the request. The indexer signs the
/// allocation by signing the EIP-712 hash with the private key of the allocation associated with
/// the deployment being queried.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Attestation {
    /// The keccak-256 hash of the request being attested.
    #[cfg_attr(feature = "serde", serde(rename = "requestCID"))]
    pub request_cid: B256,
    /// The keccak-256 hash of the response being attested.
    #[cfg_attr(feature = "serde", serde(rename = "responseCID"))]
    pub response_cid: B256,
    /// The subgraph deployment ID being queried.
    #[cfg_attr(feature = "serde", serde(rename = "subgraphDeploymentID"))]
    pub deployment: B256,
    /// The `r` component of the attestation signature.
    pub r: B256,
    /// The `s` component of the attestation signature.
    pub s: B256,
    /// The parity indicator of the attestation signature.
    pub v: u8,
}

alloy_sol_types::sol! {
    /// EIP-712 receipt struct for attestation signing.
    struct Receipt {
        bytes32 requestCID;
        bytes32 responseCID;
        bytes32 subgraphDeploymentID;
    }
}

/// Create an EIP-712 domain given a chain ID and dispute manager address.
pub fn eip712_domain(chain_id: U256, dispute_manager: Address) -> Eip712Domain {
    Eip712Domain {
        name: Some("Graph Protocol".into()),
        version: Some("0".into()),
        chain_id: Some(chain_id),
        verifying_contract: Some(dispute_manager),
        salt: Some(ATTESTATION_EIP712_DOMAIN_SALT),
    }
}

/// Create an attestation.
///
/// Signs the attestation with the signer's private key.
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
    let signing_hash = msg.eip712_signing_hash(domain);

    let (signature, recovery) = signer.sign_prehash_recoverable(&signing_hash.0).unwrap();

    Attestation {
        request_cid: msg.requestCID,
        response_cid: msg.responseCID,
        deployment: deployment.into(),
        r: FixedBytes(signature.r().to_bytes().into()),
        s: FixedBytes(signature.s().to_bytes().into()),
        v: recovery.to_byte(),
    }
}

/// Errors that can occur when verifying an attestation
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, thiserror::Error)]
pub enum VerificationError {
    /// The request hash in the attestation does not match the expected request hash
    #[error("invalid request hash")]
    InvalidRequestHash,

    /// The response hash in the attestation does not match the expected response hash
    #[error("invalid response hash")]
    InvalidResponseHash,

    /// Failed to recover the signer addres (allocation address) from the attestation signature
    #[error("failed to recover signer")]
    FailedSignerRecovery,

    /// The recovered signer address does not match the expected signer address
    #[error("recovered signer is not expected")]
    RecoveredSignerNotExpected,
}

/// Recover the signer's allocation address from the attestation
pub fn recover_allocation(
    domain: &Eip712Domain,
    attestation: &Attestation,
) -> Result<Address, VerificationError> {
    let signature = Signature {
        r: attestation.r.0.into(),
        s: attestation.s.0.into(),
        v: attestation.v.into(),
    };

    // Calculate the signing hash
    let msg = Receipt {
        requestCID: attestation.request_cid,
        responseCID: attestation.response_cid,
        subgraphDeploymentID: attestation.deployment,
    };
    let signing_hash = msg.eip712_signing_hash(domain);

    // Recover the address from the signature
    signature
        .recover(RecoveryMessage::Hash(signing_hash.0.into()))
        .map_err(|_| VerificationError::FailedSignerRecovery)
        .map(|bytes| Address::from(bytes.0))
}

/// Verify an attestation.
///
/// Checks that the request and response hashes match the attestation, and the address recovered
/// from the signature of the attestation matches the expected signer.
pub fn verify(
    domain: &Eip712Domain,
    attestation: &Attestation,
    expected_signer: &Address,
    request: &str,
    response: &str,
) -> Result<(), VerificationError> {
    // Check that the request and response hashes match the attestation
    if attestation.request_cid != keccak256(request) {
        return Err(VerificationError::InvalidRequestHash);
    }

    // Check that the request and response hashes match the attestation
    if attestation.response_cid != keccak256(response) {
        return Err(VerificationError::InvalidResponseHash);
    }

    // Recover the attestation signer public address (the allocation address) from the attestation
    // and check that it matches the expected signer address
    let signer = recover_allocation(domain, attestation)?;
    if &signer != expected_signer {
        return Err(VerificationError::RecoveredSignerNotExpected);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{b256, B256, U256};
    use alloy_sol_types::Eip712Domain;
    use ethers_core::k256::ecdsa::SigningKey;

    use super::{create, eip712_domain, verify, Attestation};
    use crate::{
        address, deployment_id,
        types::{Address, DeploymentId},
    };

    const DISPUTE_MANAGER_ADDRESS: Address = address!("16def7e0108a5467a106DBd7537F8591F470342e");
    const ALLOCATION_ADDRESS: Address = address!("90f8bf6a479f320ead074411a4b0e7944ea8c9c1");
    const ALLOCATION_PRIVATE_KEY: B256 =
        b256!("4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d");
    const DEPLOYMENT: DeploymentId =
        deployment_id!("QmeVg9Da6uyBvjUEy5JqCgw2VKdkTxjPvcYuE5riGpkqw1");

    /// Create a domain for testing:
    /// - `chain_id`: `1337`
    /// - `dispute_manager`: `0x16DEF7E0108A5467A106dbD7537f8591f470342E`
    fn domain() -> Eip712Domain {
        eip712_domain(
            U256::from_str_radix("1337", 10).unwrap(),
            DISPUTE_MANAGER_ADDRESS,
        )
    }

    /// Create a signer for testing
    ///
    /// - `private_key`: `0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d`
    /// - `address`: `0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1`
    ///
    /// Returns the allocation address and signer.
    fn signer() -> (Address, SigningKey) {
        (
            ALLOCATION_ADDRESS,
            SigningKey::from_slice(ALLOCATION_PRIVATE_KEY.as_slice()).unwrap(),
        )
    }

    /// Verify an attestation (created by old indexer-native module from TS indexer implementation)
    #[test]
    fn verify_attestation() {
        //* Given
        let domain = domain();
        let (address, _signer) = signer();
        let deployment = DEPLOYMENT;

        let request = "foo";
        let response = "bar";

        let attestation = Attestation {
            request_cid: b256!("41b1a0649752af1b28b3dc29a1556eee781e4a4c3a1f7f53f90fa834de098c4d"),
            response_cid: b256!("435cd288e3694b535549c3af56ad805c149f92961bf84a1c647f7d86fc2431b4"),
            deployment: deployment.into(),
            r: b256!("e1fb47e7f0b278d4c88564c3a3b46180e476edcb2b783f253f3eec3b36f8fd4f"),
            s: b256!("467a881937edf2faf76e2e497085caf370c9689a1d83b245030757f70a1f64de"),
            v: 28,
        };

        //* When
        let result = verify(&domain, &attestation, &address, request, response);

        //* Then
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn create_and_sign_an_attestation() {
        //* Given
        let domain = domain();
        let (address, signer) = signer();
        let deployment = DEPLOYMENT;

        let request = "foo";
        let response = "bar";

        //* When
        let attestation = create(&domain, &signer, &deployment, request, response);

        //* Then
        let result = verify(&domain, &attestation, &address, request, response);
        assert_eq!(result, Ok(()));
    }
}
