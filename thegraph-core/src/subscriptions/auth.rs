//! A subscription auth token is an authorization token that allows a user to send queries to the graph network.
//!
//! # Auth Token format
//!
//! An auth token has 2 parts:
//!
//!  1. Payload (containing auth token claims)
//!  2. Signature
//!
//! The signature is always the last 65 bytes of the auth token. The auth token should be Base64Url encoded when they are sent
//! to gateways along with queries.
//!
//! ## Payload
//!
//! The auth token payload must be a [CBOR](https://www.rfc-editor.org/rfc/rfc7049)-encoded map and MUST containing the
//! following auth token claims:
//!
//!  1. *chain_id:* Chain ID (EIP-155) for the chain on which the subscriptions contract is deployed.
//!  2. *contract:* Address of the subscriptions contract.
//!  3. *signer:* Signer address that is authorized to sign the auth token. This address should be the user's address or
//!               one of the user's authorized signers.
//!  4. *user (optional):* User address associated with the subscription. Required to when the authorized `signer` is
//!                        not the `user` associated with a subscription. When omitted, the `signer` is implied to be
//!                        equal to the `user`.
//!
//! Other optional fields may be supported at the gateway operator's discretion. See [`AuthTokenClaims`] for other
//! supported fields.
//!
//! Note that the gateway address is implied to be the owner of the subscriptions contract. In the future, we may need
//! to explicitly identify the intended recipient gateway to prevent attacks where a gateway proxies requests.
//!
//! ## Signature
//!
//! Signing and verification of auth tokens uses an Ethereum signed message ([EIP-191](https://eips.ethereum.org/EIPS/eip-191),
//! `personal_sign`) constructed from the auth token claims.
//!  
//!  - The message MUST be UTF-8 encoded.
//!  - Fields MUST be ordered lexicographically by field name.
//!  - Each field MUST be immediately followed by an ASCII LF character (`0x0a`).
//!  - Each field name and value MUST be separated by `": "`.
//!  - Any byte string value MUST be formatted as `0x` followed by its hex-encoded bytes.
//!
//! See [`AuthTokenClaims::to_verification_message`] method for the implementation details.

use std::io::{Cursor, Write as _};

pub use alloy_chains::Chain;
use alloy_primitives::Address;
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine as _};
use ethers::signers::Wallet;
use ethers_core::k256::ecdsa::SigningKey;
use ethers_core::types::Signature;
use serde_with::formats::CommaSeparator;
use serde_with::{FromInto, StringWithSeparator};

use crate::types::{DeploymentId, SubgraphId};

/// Claims that are encoded in an auth token.
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AuthTokenClaims {
    /// Chain ID (EIP-155) of the chain on which the subscriptions contract is deployed.
    #[serde_as(as = "FromInto<u64>")]
    #[serde(rename = "chain_id")]
    pub chain: Chain,

    /// Address of the subscriptions contract.
    pub contract: Address,

    /// Signer address that is authorized to sign the auth token.
    ///
    /// This address should be the user's address or one of the user's authorized signers.
    pub signer: Address,

    /// User address associated with the subscription.
    ///
    /// Required when the authorized `signer` is not the `user` associated with a subscription. When
    /// omitted, the `signer` is implied to be equal to the `user`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user: Option<Address>,

    /// Optional name of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,

    /// List of subgraphs that can be queried with this auth token.
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, SubgraphId>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub allowed_subgraphs: Vec<SubgraphId>,

    /// List of subgraph deployments that can be queried with this auth token.
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, DeploymentId>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub allowed_deployments: Vec<DeploymentId>,

    /// List of origin domains that can send queries with this auth token.
    #[serde_as(as = "StringWithSeparator::<CommaSeparator, String>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub allowed_domains: Vec<String>,
}

impl AuthTokenClaims {
    /// Returns the EIP-155 [Chain] on which the subscriptions contract is deployed.
    pub fn chain(&self) -> Chain {
        self.chain
    }

    /// Returns the chain ID (EIP-155) of the [Chain] on which the subscriptions contract is deployed.
    pub fn chain_id(&self) -> u64 {
        self.chain.id()
    }

    /// Returns the address of the subscriptions contract.
    pub fn contract(&self) -> Address {
        self.contract
    }

    /// Returns the signer address that is authorized to sign the auth token.
    pub fn signer(&self) -> Address {
        self.signer
    }

    /// Returns the user address that the auth token is for.
    ///
    /// If the `user` field is not set, the `signer` is address is returned.
    pub fn user(&self) -> Address {
        self.user.unwrap_or(self.signer)
    }

    /// Returns the name of the subscription.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the list of subgraphs that can be queried with this auth token.
    pub fn allowed_subgraphs(&self) -> &[SubgraphId] {
        &self.allowed_subgraphs
    }

    /// Returns the list of subgraph deployments that can be queried with this auth token.
    pub fn allowed_deployments(&self) -> &[DeploymentId] {
        &self.allowed_deployments
    }

    /// Returns the list of origin domains that can send queries with this auth token.
    pub fn allowed_domains(&self) -> &[String] {
        &self.allowed_domains
    }

    /// Returns the verification message that must be signed by the signer address.
    ///
    /// The verification message is a string that contains all the claims serialized in a human-readable format
    /// that follows these rules:
    ///
    ///  - The message must be UTF-8 encoded.
    ///  - Fields must be ordered lexicographically by field name.
    ///  - Each field must be immediately followed by an ASCII LF character (`0x0a`).
    ///  - Each field name and value must be separated by `": "`.
    ///  - Any byte string value must be formatted as `0x` followed by its hex-encoded bytes.
    ///
    /// The returned serialized message is a `String` ready to be hashed and signed.
    fn to_verification_message(&self) -> String {
        let mut cursor = Cursor::<Vec<u8>>::default();

        if !self.allowed_deployments.is_empty() {
            let allowed_deployments = self
                .allowed_deployments
                .iter()
                .map(|deployment| deployment.to_string())
                .collect::<Vec<_>>()
                .join(",");
            writeln!(&mut cursor, "allowed_deployments: {}", allowed_deployments).unwrap();
        }
        if !self.allowed_domains.is_empty() {
            let allowed_domains = self
                .allowed_domains
                .iter()
                .map(|domain| domain.to_string())
                .collect::<Vec<_>>()
                .join(",");
            writeln!(&mut cursor, "allowed_domains: {}", allowed_domains).unwrap();
        }
        if !self.allowed_subgraphs.is_empty() {
            let allowed_subgraphs = self
                .allowed_subgraphs
                .iter()
                .map(|subgraph| subgraph.to_string())
                .collect::<Vec<_>>()
                .join(",");
            writeln!(&mut cursor, "allowed_subgraphs: {}", allowed_subgraphs).unwrap();
        }

        writeln!(&mut cursor, "chain_id: {}", self.chain.id()).unwrap();
        writeln!(&mut cursor, "contract: {:?}", self.contract).unwrap();

        if let Some(name) = self.name.as_deref() {
            writeln!(&mut cursor, "name: {}", name).unwrap();
        }

        writeln!(&mut cursor, "signer: {:?}", self.signer).unwrap();

        if let Some(user) = self.user.as_deref() {
            writeln!(&mut cursor, "user: {:?}", user).unwrap();
        }

        String::from_utf8(cursor.into_inner()).unwrap()
    }
}

/// Errors that can occur when encoding an auth token.
#[derive(Debug, thiserror::Error)]
pub enum EncodingError {
    /// The auth token claims could not be signed.
    #[error("failed to sign the auth token claims")]
    ClaimsSigningError,

    /// The claims could not be encoded as CBOR.
    #[error("failed to encode claims as CBOR")]
    ClaimsEncodingError,
}

/// Generates an auth token from the given claims and private key.
pub fn encode_auth_token(
    claims: &AuthTokenClaims,
    wallet: &Wallet<SigningKey>,
) -> Result<String, EncodingError> {
    // Generate the claims hash and sign it.
    let claims_hash = ethers_core::utils::hash_message(claims.to_verification_message());
    let signature = wallet
        .sign_hash(claims_hash)
        .map_err(|_| EncodingError::ClaimsSigningError)?;

    // Encode the claims and signature as CBOR.
    let auth_token_bytes = {
        let mut buf =
            serde_cbor_2::ser::to_vec(claims).map_err(|_| EncodingError::ClaimsEncodingError)?;
        buf.append(&mut signature.to_vec());
        buf
    };

    // Encode the auth token as base64.
    let auth_token = BASE64_URL_SAFE_NO_PAD.encode(auth_token_bytes);

    Ok(auth_token)
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// The auth token is not encoded in a valid base64 format.
    ///
    /// The auth token must be encoded in base64 with the URL-safe alphabet and without padding.
    #[error("invalid base64 encoding")]
    InvalidBase64Encoding,

    /// The signature is not a valid 65-byte ECDSA signature.
    #[error("invalid signature")]
    InvalidSignature,

    /// The claims are not a valid CBOR-encoded value.
    #[error("invalid claims encoding")]
    InvalidClaimsEncoding,
}

/// Parses an auth token from a base64-encoded string.
///
/// The auth token must contain the CBOR-encoded claims followed by a 65-byte ECDSA signature. Then the auth token
/// is encoded in base64 with the URL-safe alphabet and without padding.
pub fn parse_auth_token(value: &str) -> Result<(AuthTokenClaims, Signature), ParseError> {
    // Decode the auth token from base64.
    let auth_token = BASE64_URL_SAFE_NO_PAD
        .decode(value)
        .map_err(|_| ParseError::InvalidBase64Encoding)?;

    // Extract the signature from the end of the auth token.
    let signature_start = auth_token.len().saturating_sub(65);
    let signature = auth_token[signature_start..]
        .try_into()
        .map(Signature::from)
        .map_err(|_| ParseError::InvalidSignature)?;

    // Decode the claims from the start of the auth token.
    let claims: AuthTokenClaims = serde_cbor_2::de::from_reader(&auth_token[..signature_start])
        .map_err(|_| ParseError::InvalidClaimsEncoding)?;

    Ok((claims, signature))
}

#[derive(Debug, thiserror::Error)]
pub enum SignatureVerificationError {
    /// The Ethereum address which was used to sign the given message could not be recovered.
    #[error("failed to recover signer public key")]
    SignerPublicKeyRecoveryError,

    /// The recovered signer address does not match the signer address in the claims.
    #[error("invalid signature")]
    VerificationError,
}

/// Verifies that the given signature was signed by the signer address in the claims.
pub fn verify_auth_token_claims(
    claims: &AuthTokenClaims,
    signature: &Signature,
) -> Result<(), SignatureVerificationError> {
    let recovery_message = ethers_core::utils::hash_message(claims.to_verification_message());
    let signer = signature
        .recover(recovery_message)
        .map_err(|_| SignatureVerificationError::SignerPublicKeyRecoveryError)?;

    // Verify that the recovered signer address matches the signer address in the claims.
    if signer.as_bytes() != claims.signer {
        return Err(SignatureVerificationError::VerificationError);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy_primitives::Address;
    use assert_matches::assert_matches;
    use ethers::prelude::Wallet;
    use ethers::signers::Signer;

    use super::{
        encode_auth_token, parse_auth_token, verify_auth_token_claims, AuthTokenClaims, Chain,
        ParseError, SignatureVerificationError,
    };

    #[test]
    fn serialize_claims_into_verification_message() {
        //* Given
        let chain = Chain::dev();
        let contract: Address = "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512"
            .parse()
            .expect("invalid contract address");

        let wallet =
            Wallet::from_str("0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d")
                .expect("invalid private key");
        let wallet_address: Address = wallet.address().as_fixed_bytes().into();

        let claims = AuthTokenClaims {
            chain,
            contract,
            signer: wallet_address,
            user: None,
            name: None,
            allowed_subgraphs: vec![],
            allowed_deployments: vec![],
            allowed_domains: vec![],
        };

        let expected_message = indoc::indoc! {"
            chain_id: 1337
            contract: 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512
            signer: 0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1
        "};

        //* When
        let result = claims.to_verification_message();

        //* Then
        assert_eq!(result, expected_message);
    }

    #[test]
    fn serialize_claims_with_extras_into_verification_message() {
        //* Given
        let chain = Chain::dev();
        let contract: Address = "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512"
            .parse()
            .expect("invalid contract address");

        let wallet =
            Wallet::from_str("0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d")
                .expect("invalid private key");
        let wallet_address: Address = wallet.address().as_fixed_bytes().into();

        let allowed_subgraphs = vec![
            // https://thegraph.com/explorer/subgraphs/8yHBZUvXcKkZnZM7SDSgcRMtbtNwgUQfM37cA37h7cet?view=Overview&chain=mainnet
            "8yHBZUvXcKkZnZM7SDSgcRMtbtNwgUQfM37cA37h7cet"
                .parse()
                .expect("invalid subgraph id"),
            // https://thegraph.com/explorer/subgraphs/AQHJdvUMkPfSxi6Q2LxXYjWXjGvfCST8DFFYE4VUKtU6?view=Overview&chain=arbitrum-one
            "AQHJdvUMkPfSxi6Q2LxXYjWXjGvfCST8DFFYE4VUKtU6"
                .parse()
                .expect("invalid subgraph id"),
        ];
        let allowed_deployments = vec![
            // https://thegraph.com/explorer/subgraphs/8yHBZUvXcKkZnZM7SDSgcRMtbtNwgUQfM37cA37h7cet?view=Overview&chain=mainnet
            "QmRbgjyzEgfxGbodu6itfkXCQ5KA9oGxKscrcQ9QuF88oT"
                .parse()
                .expect("invalid deployment id"),
            // https://thegraph.com/explorer/subgraphs/AQHJdvUMkPfSxi6Q2LxXYjWXjGvfCST8DFFYE4VUKtU6?view=Overview&chain=arbitrum-one
            "QmZ9kr5Cjepmdrj5EJnmfsneiJtok7rVpk81KUmZzFzkvp"
                .parse()
                .expect("invalid deployment id"),
        ];
        let allowed_domains = vec!["thegraph.com".into(), "testnet.thegraph.com".into()];

        let claims = AuthTokenClaims {
            chain,
            contract,
            signer: wallet_address,
            user: None,
            name: None,
            allowed_subgraphs,
            allowed_deployments,
            allowed_domains,
        };

        let expected_message = indoc::indoc! {"
            allowed_deployments: QmRbgjyzEgfxGbodu6itfkXCQ5KA9oGxKscrcQ9QuF88oT,QmZ9kr5Cjepmdrj5EJnmfsneiJtok7rVpk81KUmZzFzkvp
            allowed_domains: thegraph.com,testnet.thegraph.com
            allowed_subgraphs: 8yHBZUvXcKkZnZM7SDSgcRMtbtNwgUQfM37cA37h7cet,AQHJdvUMkPfSxi6Q2LxXYjWXjGvfCST8DFFYE4VUKtU6
            chain_id: 1337
            contract: 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512
            signer: 0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1
        "};

        //* When
        let result = claims.to_verification_message();

        //* Then
        assert_eq!(result, expected_message);
    }

    #[test]
    fn encode_required_claims_and_sign_the_auth_token() {
        //* Given
        let chain = Chain::dev();
        let contract: Address = "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512"
            .parse()
            .expect("invalid contract address");

        let wallet =
            Wallet::from_str("0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d")
                .expect("invalid private key");
        let wallet_address: Address = wallet.address().as_fixed_bytes().into();

        let claims = AuthTokenClaims {
            chain,
            contract,
            signer: wallet_address,
            user: None,
            name: None,
            allowed_subgraphs: vec![],
            allowed_deployments: vec![],
            allowed_domains: vec![],
        };

        // NOTE: CBOR serialization output manually verified using https://cbor.nemo157.com/
        let expected_auth_token = "o2hjaGFpbl9pZBkFOWhjb250cmFjdFTn8XJedzTOKI-DZ-G7FD6Quz8FEmZzaWd\
                                        uZXJUkPi_akefMg6tB0QRpLDnlE6oycGJ0BmGU8gFyjO7ELgWvEc4WV1LpCUNpL\
                                        4MGJTUXtzR9gktyGqD-yln-rRyPh9Pkfem5vXcgbLeni0Vdg--Gf14HA";

        //* When
        let result = encode_auth_token(&claims, &wallet);

        //* Then
        assert_matches!(result, Ok(auth_token) => {
            assert_eq!(auth_token, expected_auth_token);
        });
    }

    #[test]
    fn encode_extra_claims_and_sign_the_auth_token() {
        //* Given
        let chain = Chain::dev();
        let contract: Address = "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512"
            .parse()
            .expect("invalid contract address");

        let wallet =
            Wallet::from_str("0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d")
                .expect("invalid private key");
        let wallet_address: Address = wallet.address().as_fixed_bytes().into();

        let allowed_subgraphs = vec![
            // https://thegraph.com/explorer/subgraphs/8yHBZUvXcKkZnZM7SDSgcRMtbtNwgUQfM37cA37h7cet?view=Overview&chain=mainnet
            "8yHBZUvXcKkZnZM7SDSgcRMtbtNwgUQfM37cA37h7cet"
                .parse()
                .expect("invalid subgraph id"),
            // https://thegraph.com/explorer/subgraphs/AQHJdvUMkPfSxi6Q2LxXYjWXjGvfCST8DFFYE4VUKtU6?view=Overview&chain=arbitrum-one
            "AQHJdvUMkPfSxi6Q2LxXYjWXjGvfCST8DFFYE4VUKtU6"
                .parse()
                .expect("invalid subgraph id"),
        ];
        let allowed_deployments = vec![
            // https://thegraph.com/explorer/subgraphs/8yHBZUvXcKkZnZM7SDSgcRMtbtNwgUQfM37cA37h7cet?view=Overview&chain=mainnet
            "QmRbgjyzEgfxGbodu6itfkXCQ5KA9oGxKscrcQ9QuF88oT"
                .parse()
                .expect("invalid deployment id"),
            // https://thegraph.com/explorer/subgraphs/AQHJdvUMkPfSxi6Q2LxXYjWXjGvfCST8DFFYE4VUKtU6?view=Overview&chain=arbitrum-one
            "QmZ9kr5Cjepmdrj5EJnmfsneiJtok7rVpk81KUmZzFzkvp"
                .parse()
                .expect("invalid deployment id"),
        ];
        let allowed_domains = vec!["thegraph.com".into(), "testnet.thegraph.com".into()];

        let claims = AuthTokenClaims {
            chain,
            contract,
            signer: wallet_address,
            user: None,
            name: None,
            allowed_subgraphs,
            allowed_deployments,
            allowed_domains,
        };

        // NOTE: CBOR serialization output manually verified using https://cbor.nemo157.com/
        let expected_auth_token = "pmhjaGFpbl9pZBkFOWhjb250cmFjdFTn8XJedzTOKI-DZ-G7FD6Quz8FEmZzaWd\
                                        uZXJUkPi_akefMg6tB0QRpLDnlE6oycFxYWxsb3dlZF9zdWJncmFwaHN4WTh5SE\
                                        JaVXZYY0trWm5aTTdTRFNnY1JNdGJ0TndnVVFmTTM3Y0EzN2g3Y2V0LEFRSEpkd\
                                        lVNa1BmU3hpNlEyTHhYWWpXWGpHdmZDU1Q4REZGWUU0VlVLdFU2c2FsbG93ZWRf\
                                        ZGVwbG95bWVudHN4XVFtUmJnanl6RWdmeEdib2R1Nml0ZmtYQ1E1S0E5b0d4S3N\
                                        jcmNROVF1Rjg4b1QsUW1aOWtyNUNqZXBtZHJqNUVKbm1mc25laUp0b2s3clZwaz\
                                        gxS1VtWnpGemt2cG9hbGxvd2VkX2RvbWFpbnN4IXRoZWdyYXBoLmNvbSx0ZXN0b\
                                        mV0LnRoZWdyYXBoLmNvbTZ01E75a5j_z_9HBYABzMwIqsXbQJBaM3gRJ6HALqeZ\
                                        Xj-pASMlOboCUCqhoFVpucKycQ4oL54zL2jnasIHwekb";

        //* When
        let result = encode_auth_token(&claims, &wallet);

        //* Then
        assert_matches!(result, Ok(auth_token) => {
            println!("auth_token: {}", auth_token);
            assert_eq!(auth_token, expected_auth_token);
        });
    }

    #[test]
    fn parse_invalid_auth_token_with_invalid_base64_encoding() {
        //* Given
        let auth_token = "X2ludmFsaWRfYXV0aF90b2tlbl8="; // base64("_invalid_auth_token_")

        //* When
        let result = parse_auth_token(auth_token);

        //* Then
        assert_matches!(result, Err(ParseError::InvalidBase64Encoding));
    }

    #[test]
    fn parse_invalid_auth_token_with_valid_base64_encoding() {
        //* Given
        let auth_token = "X2ludmFsaWRfYXV0aF90b2tlbl8"; // base64URL("_invalid_auth_token_")

        //* When
        let result = parse_auth_token(auth_token);

        //* Then
        assert_matches!(result, Err(ParseError::InvalidSignature));
    }

    #[test]
    fn parse_valid_auth_token() {
        //* Given
        let expected_chain = Chain::dev();
        let expected_contract: Address = "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512"
            .parse()
            .expect("invalid contract address");

        let expected_user: Address = "0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1"
            .parse()
            .expect("invalid address");

        let auth_token = "o2hjaGFpbl9pZBkFOWhjb250cmFjdFTn8XJedzTOKI-DZ-G7FD6Quz8FEmZzaWduZXJUkPi_akefMg6tB0QRpLDnlE6oycGJ0BmGU8gFyjO7ELgWvEc4WV1LpCUNpL4MGJTUXtzR9gktyGqD-yln-rRyPh9Pkfem5vXcgbLeni0Vdg--Gf14HA";

        //* When
        let result = parse_auth_token(auth_token);

        //* Then
        assert_matches!(result, Ok((claims, _signature)) => {
            // Assert auth_token claims
            assert_eq!(claims.chain, expected_chain);
            assert_eq!(claims.contract, expected_contract);
            assert_eq!(claims.signer, expected_user);
            assert_eq!(claims.user, None);

            assert_eq!(claims.user(), expected_user);
        });
    }

    #[test]
    fn verify_invalid_auth_token_claims() {
        //* Given
        let chain = Chain::dev();
        let contract: Address = "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512"
            .parse()
            .expect("invalid contract address");

        let wallet =
            Wallet::from_str("0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d")
                .expect("invalid private key");

        // Set a different signer address in the claims
        let signer: Address = "0xf3515b9472fA4bd4a2e4D0B30E9B4a0ab2A68B19"
            .parse()
            .expect("invalid signer address");

        let claims = AuthTokenClaims {
            chain,
            contract,
            signer,
            user: None,
            name: None,
            allowed_subgraphs: vec![],
            allowed_deployments: vec![],
            allowed_domains: vec![],
        };

        // Encode the auth_token and parse it back.
        let auth_token = encode_auth_token(&claims, &wallet).expect("failed to encode auth_token");
        let (claims, signature) =
            parse_auth_token(&auth_token).expect("failed to parse auth_token");

        //* When
        let result = verify_auth_token_claims(&claims, &signature);

        //* Then
        assert_matches!(result, Err(SignatureVerificationError::VerificationError));
    }

    #[test]
    fn verify_valid_auth_token_claims() {
        //* Given
        let chain = Chain::dev();
        let contract: Address = "0xe7f1725e7734ce288f8367e1bb143e90bb3f0512"
            .parse()
            .expect("invalid contract address");

        let wallet =
            Wallet::from_str("0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d")
                .expect("invalid private key");
        let wallet_address: Address = wallet.address().as_fixed_bytes().into();

        let claims = AuthTokenClaims {
            chain,
            contract,
            signer: wallet_address,
            user: None,
            name: None,
            allowed_subgraphs: vec![],
            allowed_deployments: vec![],
            allowed_domains: vec![],
        };

        // Encode the auth_token and parse it back.
        let auth_token = encode_auth_token(&claims, &wallet).expect("failed to encode auth_token");
        let (claims, signature) =
            parse_auth_token(&auth_token).expect("failed to parse auth_token");

        //* When
        let result = verify_auth_token_claims(&claims, &signature);

        //* Then
        assert_matches!(result, Ok(()));
    }
}
