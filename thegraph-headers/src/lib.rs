//! Common HTTP _typed headers_ used across _The Graph_ network services.
//!
//! See _hyper_'s [`headers`][1] crate and _axum-extra_'s [`TypedHeader`][2] extractor
//! documentation for more information on how to use _typed headers_.
//!
//! [1]: https://docs.rs/headers/latest/headers/index.html
//! [2]: https://docs.rs/axum-extra/latest/axum_extra/typed_header/struct.TypedHeader.html

#![cfg_attr(docsrs, feature(doc_cfg))]

pub use headers;

pub mod graph_attestable;
#[cfg(feature = "attestation")]
#[cfg_attr(docsrs, doc(cfg(feature = "attestation")))]
pub mod graph_attestation;
pub mod graph_indexed;
mod http_ext;

pub use http_ext::HttpBuilderExt;
