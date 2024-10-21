//! EIP712 message signing and verification.
//!
//! This module contains the `SignedMessage` struct which is used to sign and verify messages
//! using the [EIP-712] standard.
//!
//! # API
//!
//! The module provides the following functions:
//!
//! - `sign`: Signs a message using the [EIP-712] standard
//! - `recover_signer_address`: Recovers the signer's address of a signed message
//! - `verify`: Verifies the signer's address of a signed message
//!
//! The module also provides the `Eip712Signer` struct which wraps a ECDSA signer and an [EIP-712]
//! domain separator for convenient message signing and verification.
//!
//! ## Example
//! ```rust
//! # use thegraph_core::address;
//! # use thegraph_core::signed_message::{b256, eip712_domain, Eip712Domain};
//! use thegraph_core::signed_message::{sign, verify};
//!
//! // Create a signer instance
//! let signer = alloy_signer_local::PrivateKeySigner::random();
//!
//! // Define the EIP712 domain separator
//! const EIP712_DOMAIN: Eip712Domain = eip712_domain! {
//!      name: "Test domain",
//!      version: "1",
//!      chain_id: 1,
//!      verifying_contract: address!("a83682bbe91c0d2d48a13fd751b2da8e989fe421"),
//!      salt: b256!("66eb090e6dbb9668c7d32c0ee7ba5e8f08d84385804485d316dd5f5692273593"),
//! };
//!
//! // Define the message struct
//! thegraph_core::signed_message::sol! {
//!    struct Message {
//!       address data;
//!    }
//! }
//!
//! // Create a message instance with some data
//! let message = Message {
//!    data: address!("03f6d2a3d8c3413de72c193386f1894e1ddc2b6b"),
//! };
//!
//! // Sign the message
//! let signed_message = sign(&signer, &EIP712_DOMAIN, message).expect("sign_message failed");
//!
//! // Verify the signed message
//! assert!(verify( &EIP712_DOMAIN,&signed_message, &signer.address()).is_ok());
//! ```
//!
//! # The `Eip712Signer` struct
//!
//! The `Eip712Signer` struct wraps a ECDSA signer and an [EIP-712] domain separator for convenient
//! message signing and verification.
//!
//! ## Example
//!
//! ```rust
//! # use thegraph_core::address;
//! # use thegraph_core::signed_message::{b256, eip712_domain, Eip712Domain};
//! use thegraph_core::signed_message::Eip712Signer;
//!
//! // Create a signer instance and define the EIP-712 domain separator
//! let signer = alloy_signer_local::PrivateKeySigner::random();
//! let signer_address = signer.address();
//!
//! const EIP712_DOMAIN: Eip712Domain = eip712_domain! {
//!     name: "Test domain",
//!     version: "1",
//!     chain_id: 1,
//!     verifying_contract: address!("a83682bbe91c0d2d48a13fd751b2da8e989fe421"),
//!     salt: b256!("79ae210ebdaa728c415644dddfdd8050d94cc832af6892c51f7e218e85e32260"),
//! };
//!
//! // Create an Eip712Signer instance
//! let eip712_signer = Eip712Signer::new(signer, signer_address, EIP712_DOMAIN);
//!
//! // Define the message struct
//! thegraph_core::signed_message::sol! {
//!     struct Message {
//!         address data;
//!     }
//! }
//!
//! // Create a message instance with some data
//! let message = Message {
//!     data: address!("c7267b097d5ee3fd61eed65f3d48498714776af3"),
//! };
//!
//! // Sign the message
//! let signed_message = eip712_signer.sign(message).expect("message signing failed");
//!
//! // Verify the signed message
//! assert!(eip712_signer.verify(&signed_message, &eip712_signer.address()).is_ok());
//! ```
//!
//! [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"

use alloy_primitives::Address;
pub use alloy_primitives::{b256, Signature, SignatureError};
pub use alloy_signer::{k256::ecdsa::Error as EcdsaError, SignerSync, UnsupportedSignerOperation};
pub use alloy_sol_types::{eip712_domain, sol, Eip712Domain, SolStruct};

/// Errors that can occur when signing a message.
#[derive(Debug, thiserror::Error)]
pub enum SigningError {
    /// The signer does not support the operation
    #[error("operation `{0}` is not supported by the signer")]
    UnsupportedOperation(UnsupportedSignerOperation),

    /// The ECDSA signature failed
    #[error(transparent)]
    Ecdsa(#[from] EcdsaError),

    /// Generic error
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync + 'static>),
}

/// Errors that can occur when recovering the signer's address of a message.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct RecoverSignerError(#[from] SignatureError);

/// Errors that can occur when verifying the signer's address of a message.
#[derive(Debug, thiserror::Error)]
pub enum VerificationError {
    /// Errors in signature parsing or verification
    #[error(transparent)]
    SignatureError(#[from] SignatureError),

    /// The signer's address does not match the expected address
    #[error("expected signer `{expected}` but received `{received}`")]
    InvalidSigner {
        /// The expected signer's address
        expected: Address,
        /// The received signer's address
        received: Address,
    },
}

/// EIP712 signed message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedMessage<M> {
    /// Message payload
    pub message: M,
    /// ECDSA signature of EIP-712 hash of message
    pub signature: Signature,
}

/// Signs a message using the [EIP-712] standard
///
/// Returns a [`SignedMessage`] containing the message and the ECDSA signature of the message
///
/// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"
pub fn sign<S, M>(
    signer: &S,
    domain: &Eip712Domain,
    message: M,
) -> Result<SignedMessage<M>, SigningError>
where
    S: SignerSync,
    M: SolStruct,
{
    let signature = signer
        .sign_typed_data_sync(&message, domain)
        .map_err(|err| match err {
            alloy_signer::Error::UnsupportedOperation(err) => {
                SigningError::UnsupportedOperation(err)
            }
            alloy_signer::Error::DynAbiError(_) => {
                unreachable!("sign_typed_data_sync should not return DynAbiError")
            }
            alloy_signer::Error::Ecdsa(err) => SigningError::Ecdsa(err),
            alloy_signer::Error::HexError(_) => {
                unreachable!("sign_typed_data_sync should not return HexError")
            }
            alloy_signer::Error::Other(err) => SigningError::Other(err),
            alloy_signer::Error::TransactionChainIdMismatch { .. } => {
                unreachable!("sign_typed_data_sync should not return TransactionChainIdMismatch")
            }
            alloy_signer::Error::SignatureError(_) => {
                unreachable!("sign_typed_data_sync should not return SignatureError")
            }
        })?;
    Ok(SignedMessage { message, signature })
}

/// Recover the signer's address  an [EIP-712] signed message
///
/// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"
pub fn recover_signer_address<M>(
    domain: &Eip712Domain,
    signed_message: &SignedMessage<M>,
) -> Result<Address, RecoverSignerError>
where
    M: SolStruct,
{
    let recovery_message_hash = signed_message.message.eip712_signing_hash(domain);
    let recovered_address = signed_message
        .signature
        .recover_address_from_prehash(&recovery_message_hash)?;
    Ok(recovered_address)
}

/// Verify the signer's address of an [EIP-712] signed message
///
/// Returns `Ok(())` if the signer's address matches the expected address
///
/// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"
pub fn verify<M>(
    domain: &Eip712Domain,
    signed_message: &SignedMessage<M>,
    expected_address: &Address,
) -> Result<(), VerificationError>
where
    M: SolStruct,
{
    let recovered_address =
        recover_signer_address(domain, signed_message).map_err(|RecoverSignerError(err)| err)?;

    if recovered_address != *expected_address {
        Err(VerificationError::InvalidSigner {
            expected: expected_address.to_owned(),
            received: recovered_address,
        })
    } else {
        Ok(())
    }
}

/// An [`Eip712Signer`] wraps a ECDSA signer and an [EIP-712] domain separator.
///
/// It provides a convenient way to sign and verify messages using the [EIP-712] standard.
///
/// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"
pub struct Eip712Signer<S> {
    /// The ECDSA signer
    signer: S,
    /// The signer's address
    address: Address,
    /// The EIP-712 domain separator
    domain: Eip712Domain,
}

impl<S> Eip712Signer<S>
where
    S: SignerSync,
{
    /// Create a new [`Eip712Signer`] instance
    pub fn new(signer: S, address: Address, domain: Eip712Domain) -> Self {
        Self {
            signer,
            address,
            domain,
        }
    }

    /// Get the signer's address
    pub fn address(&self) -> &Address {
        &self.address
    }

    /// Sign a message using the [EIP-712] standard
    ///
    /// Returns a [`SignedMessage`] containing the message and the ECDSA signature of the message
    ///
    /// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"
    pub fn sign<M>(&self, message: M) -> Result<SignedMessage<M>, SigningError>
    where
        M: SolStruct,
    {
        sign(&self.signer, &self.domain, message)
    }

    /// Recover the signer's address  an [EIP-712] signed message
    ///
    /// Returns the signer's address
    ///
    /// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"
    pub fn recover_signer<M>(
        &self,
        signed_message: &SignedMessage<M>,
    ) -> Result<Address, RecoverSignerError>
    where
        M: SolStruct,
    {
        recover_signer_address(&self.domain, signed_message)
    }

    /// Verify the signer's address of an [EIP-712] signed message
    ///
    /// Returns `Ok(())` if the signer's address matches the expected address
    ///
    /// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"
    pub fn verify<M>(
        &self,
        signed_message: &SignedMessage<M>,
        expected_address: &Address,
    ) -> Result<(), VerificationError>
    where
        M: SolStruct,
    {
        verify(&self.domain, signed_message, expected_address)
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{address, b256, keccak256, Signature};
    use alloy_signer_local::PrivateKeySigner;
    use alloy_sol_types::{eip712_domain, Eip712Domain};

    use super::{
        recover_signer_address, sign, verify, Eip712Signer, SignedMessage, VerificationError,
    };

    /// Test EIP712 domain separator
    const EIP712_DOMAIN: Eip712Domain = eip712_domain! {
        name: "Test domain",
        version: "1",
        chain_id: 1,
        verifying_contract: address!("a83682bbe91c0d2d48a13fd751b2da8e989fe421"),
        salt: b256!("66eb090e6dbb9668c7d32c0ee7ba5e8f08d84385804485d316dd5f5692273593")
    };

    alloy_sol_types::sol! {
        /// Test struct for EIP712 message
        struct Message {
            bytes32 data;
        }
    }

    /// Test utility method generating a random wallet
    fn wallet() -> PrivateKeySigner {
        PrivateKeySigner::random()
    }

    #[test]
    fn sign_message_with_private_key_signer() {
        //* Given
        let signer = wallet();
        let domain = EIP712_DOMAIN;

        // Create a message with some data
        let message = Message {
            data: keccak256(b"Hello, world!"),
        };

        //* When
        // Sign the message
        let result = sign(&signer, &domain, message);

        //* Then
        // The message should be signed
        assert!(result.is_ok());
    }

    #[test]
    fn recover_signer_from_signed_message() {
        //* Given
        let signer = wallet();

        let domain = EIP712_DOMAIN;

        // Create a message with some data
        let message = Message {
            data: keccak256(b"Hello, world!"),
        };

        // Sign the message
        let signed_message = sign(&signer, &domain, message).unwrap();

        //* When
        // Recover the signer's address
        let result = recover_signer_address(&domain, &signed_message);

        //* Then
        // The address should be recovered
        let signer_address = result.expect("recover_signer failed");

        // The signer should be the wallet's address
        assert_eq!(signer_address, signer_address);
    }

    #[test]
    fn recover_signer_should_fail_with_invalid_signature() {
        //* Given
        let domain = EIP712_DOMAIN;

        // Create a message with some data
        let message = Message {
            data: keccak256(b"Hello, world!"),
        };

        // Create a signed message with an invalid signature (random values)
        let invalid_signature_signed_message = SignedMessage {
            message,
            signature: Signature::from_scalars_and_parity(
                b256!("ca457b3f821e5c03545944e0318868a783d0e6b438c85a82537d52a619decfe2"),
                b256!("26a9f36fcf89431476aa556021ee77959dc480fb3458054f26d068b52d525cc4"),
                false,
            )
            .expect("invalid signature format"),
        };

        //* When
        // Recover the signer's address
        let result = recover_signer_address(&domain, &invalid_signature_signed_message);

        //* Then
        // The address should not be recovered
        assert!(result.is_err());
    }

    #[test]
    fn verify_signed_message() {
        //* Given
        let signer = wallet();
        let signer_address = signer.address();

        let domain = EIP712_DOMAIN;

        let message = Message {
            data: keccak256(b"Hello, world!"),
        };

        // Sign the message
        let signed_message = sign(&signer, &domain, message).unwrap();

        //* When
        // Verify the signed message
        let result = verify(&domain, &signed_message, &signer_address);

        //* Then
        // The signature should be valid
        assert!(result.is_ok());
    }

    #[test]
    fn signed_message_verification_should_fail_with_invalid_signer() {
        //* Given
        let signer = wallet();
        let domain = EIP712_DOMAIN;

        // Create a message with some data
        let message = Message {
            data: keccak256(b"Hello, world!"),
        };

        // Sign the message
        let signed_message = sign(&signer, &domain, message).unwrap();

        // Create a different signer
        let different_signer = wallet();
        let different_signer_address = different_signer.address();

        //* When
        // Verify the signed message
        let result = verify(&domain, &signed_message, &different_signer_address);

        //* Then
        // The signature should be invalid
        let error = result.expect_err("verify_signature should fail");
        if let VerificationError::InvalidSigner { expected, received } = error {
            assert_eq!(expected, different_signer_address);
            assert_eq!(received, signer.address());
        } else {
            panic!("unexpected error: {:?}", error);
        }
    }

    #[test]
    fn signer_sing_and_verify() {
        //* Given
        let signer = wallet();
        let signer_address = signer.address();
        let domain = EIP712_DOMAIN;

        // Create a message with some data
        let message = Message {
            data: keccak256(b"Hello, world!"),
        };

        // Create an Eip712Signer instance
        let eip712_signer = Eip712Signer::new(signer, signer_address, domain);

        //* When
        // Sign the message
        let signed_message = eip712_signer.sign(message).expect("message signing failed");

        // Verify the signed message
        let result = eip712_signer.verify(&signed_message, eip712_signer.address());

        //* Then
        // The signature should be valid
        assert!(result.is_ok());
    }
}
