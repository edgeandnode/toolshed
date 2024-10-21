use alloy::{primitives::PrimitiveSignature as Signature, sol_types::SolStruct};

/// EIP-712 signed message
///
/// This struct contains a message and the ECDSA signature of the message according to the
/// EIP-712 standard.
///
/// For the message to be signed, it must either:
/// - To be a _Solidity struct_, i.e., implement the `SolStruct` trait.
/// - To be convertible into a _Solidity struct_, i.e., implement the `ToSolStruct` trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedMessage<M> {
    /// Message payload
    pub message: M,
    /// ECDSA message signature
    pub signature: Signature,
}

impl<M> SignedMessage<M> {
    /// Get the EIP-712 signature bytes.
    ///
    /// The ECDSA signature bytes can be used as a key in a [`BTreeMap`] (or [`HashMap`]) to
    /// deduplicate signed messages based on their signature.
    ///
    /// [`BTreeMap`]: std::collections::BTreeMap
    /// [`HashMap`]: std::collections::HashMap
    pub fn signature_bytes(&self) -> SignatureBytes {
        SignatureBytes(self.signature.as_bytes())
    }

    /// Hash the message struct according to [EIP-712 `hashStruct`](https://eips.ethereum.org/EIPS/eip-712#definition-of-hashstruct).
    ///
    /// The resulting hash can be used to deduplicate messages. As the hash does not include the
    /// signature, it is unique for a given message payload. This means that two [`SignedMessage`]s,
    /// signed by two different signers, will have the same hash.
    pub fn message_hash<MSol>(&self) -> MessageHash
    where
        M: ToSolStruct<MSol>,
        MSol: SolStruct,
    {
        MessageHash(*self.message.to_sol_struct().eip712_hash_struct())
    }
}

/// The EIP-712 ECDSA signature bytes.
///
/// See: [`SignedMessage::signature_bytes`]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignatureBytes([u8; 65]);

impl SignatureBytes {
    /// Get the signature bytes
    pub fn as_bytes(&self) -> [u8; 65] {
        self.0
    }
}

impl std::ops::Deref for SignatureBytes {
    type Target = [u8; 65];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Message hash according to [EIP-712 `hashStruct`](https://eips.ethereum.org/EIPS/eip-712#definition-of-hashstruct).
///
/// See: [`SignedMessage::message_hash`]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageHash([u8; 32]);

impl MessageHash {
    /// Get the message hash bytes
    pub fn as_bytes(&self) -> [u8; 32] {
        self.0
    }
}

impl std::ops::Deref for MessageHash {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A conversion trait for converting a type into a solidity struct representation
///
/// This trait is used to convert a Rust type into a struct implementing the `SolStruct` trait.
pub trait ToSolStruct<T: SolStruct> {
    /// Convert into the solidity struct representation
    fn to_sol_struct(&self) -> T;
}

impl<T> ToSolStruct<T> for T
where
    T: SolStruct + Clone,
{
    /// Convert into the solidity struct representation.
    ///
    /// If the type already implements the `SolStruct` trait, this method will return a clone of
    /// the type without performing any conversion.
    fn to_sol_struct(&self) -> T {
        self.clone()
    }
}
