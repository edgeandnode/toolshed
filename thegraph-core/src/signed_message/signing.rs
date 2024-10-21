use alloy::{
    primitives::{Address, SignatureError},
    signers::{
        k256::ecdsa::Error as EcdsaError, Error as SignerError, SignerSync,
        UnsupportedSignerOperation,
    },
    sol_types::{Eip712Domain, SolStruct},
};

use super::message::{SignedMessage, ToSolStruct};

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

/// Signs a message using the [EIP-712] standard
///
/// Returns a [`SignedMessage`] containing the message and the ECDSA signature of the message
///
/// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"
pub fn sign<S, M, MSol>(
    signer: &S,
    domain: &Eip712Domain,
    message: M,
) -> Result<SignedMessage<M>, SigningError>
where
    S: SignerSync,
    M: ToSolStruct<MSol>,
    MSol: SolStruct,
{
    let message_sol = message.to_sol_struct();
    let signature = signer
        .sign_typed_data_sync(&message_sol, domain)
        .map_err(|err| match err {
            SignerError::UnsupportedOperation(err) => SigningError::UnsupportedOperation(err),
            SignerError::TransactionChainIdMismatch { .. } => {
                unreachable!("sign_typed_data_sync should not return TransactionChainIdMismatch")
            }
            SignerError::DynAbiError(_) => {
                unreachable!("sign_typed_data_sync should not return DynAbiError")
            }
            SignerError::Ecdsa(err) => SigningError::Ecdsa(err),
            SignerError::HexError(_) => {
                unreachable!("sign_typed_data_sync should not return HexError")
            }
            SignerError::SignatureError(_) => {
                unreachable!("sign_typed_data_sync should not return SignatureError")
            }
            SignerError::Other(err) => SigningError::Other(err),
        })?;
    Ok(SignedMessage { message, signature })
}

/// Recover the signer's address  an [EIP-712] signed message
///
/// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"
pub fn recover_signer_address<M, MSol>(
    domain: &Eip712Domain,
    signed_message: &SignedMessage<M>,
) -> Result<Address, RecoverSignerError>
where
    M: ToSolStruct<MSol>,
    MSol: SolStruct,
{
    let message_sol = signed_message.message.to_sol_struct();
    let recovery_message_hash = message_sol.eip712_signing_hash(domain);
    let recovered_address = signed_message
        .signature
        .recover_address_from_prehash(&recovery_message_hash)?;
    Ok(recovered_address)
}

/// Verify the signer's address of an [EIP-712] signed message
///
/// Returns `Ok(())` if the  signer's address retrieved from the signature matches the expected
/// address. Otherwise, returns a [`VerificationError`] with details about the mismatch.
///
/// [EIP-712]: https://eips.ethereum.org/EIPS/eip-712 "EIP-712"
pub fn verify<M, MSol>(
    domain: &Eip712Domain,
    signed_message: &SignedMessage<M>,
    expected_address: &Address,
) -> Result<(), VerificationError>
where
    M: ToSolStruct<MSol>,
    MSol: SolStruct,
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
