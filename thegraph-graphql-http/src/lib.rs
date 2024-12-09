//! The `thegraph-graphql-http` crate provides an implementation of the GraphQL-over-HTTP protocol
//! for _The Graph_ network services.
//!
//! Additionally, this crate provides a GraphQL-over-HTTP client based on this crate types and
//! [`reqwest`]'s HTTP client. To enable this client extension, use the `reqwest` feature.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod compat;
pub mod graphql;
pub mod http;

#[cfg(feature = "reqwest")]
#[cfg_attr(docsrs, doc(cfg(feature = "reqwest")))]
pub mod http_client;
