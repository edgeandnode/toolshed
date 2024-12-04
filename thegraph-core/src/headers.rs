//! Common HTTP _typed headers_ used across _The Graph_ network services.

#[cfg(feature = "attestation")]
mod graph_attestation;

#[cfg(feature = "attestation")]
#[cfg_attr(docsrs, doc(cfg(feature = "attestation"), inline))]
pub use graph_attestation::GraphAttestation;
