mod graphql;

pub use crate::graphql::*;

#[deprecated(since = "0.2.0", note = "Please use `graphql-http` crate instead")]
#[cfg(feature = "http")]
pub mod http;
