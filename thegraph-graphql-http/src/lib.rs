//! The `thegraph-graphql-http` crate provides an implementation of the GraphQL-over-HTTP protocol
//! for The Graph network services.
//!
//! Additionally, this crate provides a GraphQL-over-HTTP client based on this crate types and
//! [`reqwest`]'s HTTP client. To enable this client extension, use the
//! `http-client-reqwest` feature.

mod compat;
pub mod graphql;
pub mod http;

#[cfg(feature = "http-client-reqwest")]
pub mod http_client;
