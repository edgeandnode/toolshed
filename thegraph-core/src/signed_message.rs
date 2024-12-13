//! EIP-712 message signing and verification.
//!
//! This module provides the [`SignedMessage`] struct for signing and verifying messages according
//! to the [EIP-712] standard.
//!
//! Available functions for interacting with messages:
//!
//! - [`sign`]: Signs a message using the EIP-712 standard.
//! - [`recover_signer_address`]: Recovers the signer's address from a signed message.
//! - [`verify`]: Convenience wrapper over [`recover_signer_address`] to verify the signer's
//!   address.
//!
//! To use a Rust struct as a message, it must implement the [`ToSolStruct`] trait.
//! Refer to the example below for more details.
//!
//! ## Example
//! ```rust
//! # use thegraph_core::alloy::{
//! #    primitives::{Address, B256, address, b256, keccak256},
//! #    sol_types::{eip712_domain, Eip712Domain},
//! # };
//! use thegraph_core::signed_message::{sign, verify, ToSolStruct};
//!
//! // Create a signer instance
//! let signer = thegraph_core::alloy::signers::local::PrivateKeySigner::random();
//!
//! // Define the EIP-712 domain separator
//! const DOMAIN: Eip712Domain = eip712_domain! {
//!      name: "Example domain",
//!      version: "1",
//!      chain_id: 1,
//!      verifying_contract: address!("a83682bbe91c0d2d48a13fd751b2da8e989fe421"),
//!      salt: b256!("66eb090e6dbb9668c7d32c0ee7ba5e8f08d84385804485d316dd5f5692273593"),
//! };
//!
//! // Define a message struct
//! #[derive(Clone, Debug)]
//! struct Message {
//!    addr: Address,
//!    hash: [u8; 32],
//! }
//!
//! // Define the message equivalent solidity struct
//! thegraph_core::alloy::sol! {
//!     struct MessageSol {
//!         address addr;
//!         bytes32 hash;
//!     }
//! }
//!
//! // Implement the ToSolStruct trait for the message struct
//! impl ToSolStruct<MessageSol> for Message {
//!     fn to_sol_struct(&self) -> MessageSol {
//!         MessageSol {
//!            addr: self.addr,
//!            hash: self.hash.into(),
//!        }
//!     }
//! }
//!
//! // Create a message instance with some data
//! let message = Message {
//!    addr: address!("03f6d2a3d8c3413de72c193386f1894e1ddc2b6b"),
//!    hash: *keccak256(b"Hello, world!"),
//! };
//!
//! // Sign the message
//! let signed_message = sign(&signer, &DOMAIN, message).expect("sign_message failed");
//!
//! // Verify the signed message
//! assert!(verify(&DOMAIN, &signed_message, &signer.address()).is_ok());
//! ```
//!
//! [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"

mod message;
mod signing;

pub use message::{MessageHash, SignatureBytes, SignedMessage, ToSolStruct};
pub use signing::{
    recover_signer_address, sign, verify, RecoverSignerError, SigningError, VerificationError,
};

#[cfg(test)]
mod tests {
    use alloy::{
        primitives::{address, b256, keccak256, PrimitiveSignature as Signature},
        signers::local::PrivateKeySigner,
        sol_types::{eip712_domain, Eip712Domain},
    };

    use super::{message::SignedMessage, signing, signing::VerificationError};

    /// Test EIP712 domain separator
    const EIP712_DOMAIN: Eip712Domain = eip712_domain! {
        name: "Test domain",
        version: "1",
        chain_id: 1,
        verifying_contract: address!("a83682bbe91c0d2d48a13fd751b2da8e989fe421"),
        salt: b256!("66eb090e6dbb9668c7d32c0ee7ba5e8f08d84385804485d316dd5f5692273593")
    };

    alloy::sol! {
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
        let result = signing::sign(&signer, &domain, message);

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
        let signed_message = signing::sign(&signer, &domain, message).unwrap();

        //* When
        // Recover the signer's address
        let result = signing::recover_signer_address(&domain, &signed_message);

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
            ),
        };

        //* When
        // Recover the signer's address
        let result = signing::recover_signer_address(&domain, &invalid_signature_signed_message);

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
        let signed_message = signing::sign(&signer, &domain, message).unwrap();

        //* When
        // Verify the signed message
        let result = signing::verify(&domain, &signed_message, &signer_address);

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
        let signed_message = signing::sign(&signer, &domain, message).unwrap();

        // Create a different signer
        let different_signer = wallet();
        let different_signer_address = different_signer.address();

        //* When
        // Verify the signed message
        let result = signing::verify(&domain, &signed_message, &different_signer_address);

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
}
