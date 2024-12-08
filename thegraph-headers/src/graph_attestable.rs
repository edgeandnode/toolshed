//! An HTTP _typed header_ for the `graph-attestable` header.
//!
//! This HTTP header is used to indicate whether a response is _attestable_ or not.
//!
//! # Using the `headers::HeaderMapExt` extension trait
//!
//! ```rust
//! use headers::HeaderMapExt as _;
//! use thegraph_headers::graph_attestable::{GraphAttestable, HEADER_NAME};
//!
//! let mut header_map = http::HeaderMap::new();
//! # let value = true;
//!
//! // Insert a `graph-attestable` HTTP header
//! header_map.typed_insert(GraphAttestable(value));
//!
//! // Get the `graph-attestable` HTTP header by name
//! let header_by_name = header_map.get(HEADER_NAME);
//! assert!(header_by_name.is_some());
//!
//! // Get the `graph-attestable` HTTP header by type
//! let header_typed = header_map.typed_get::<GraphAttestable>();
//! assert!(matches!(header_typed, Some(GraphAttestable(..))));
//! ```

use headers::{Error as HeaderError, Header, HeaderName, HeaderValue};

/// The HTTP header name for the `graph-attestable` header.
pub const HEADER_NAME: &str = "graph-attestable";

/// An HTTP _typed header_ for the `graph-attestable` header.
///
/// This HTTP header is used to indicate whether a response is attestable or not.
///
/// The `graph-attestable` header can contain a boolean value, either `true` or `false`.
#[derive(Debug, Clone)]
pub struct GraphAttestable(pub bool);

impl Header for GraphAttestable {
    fn name() -> &'static HeaderName {
        static HTTP_HEADER_NAME: HeaderName = HeaderName::from_static(HEADER_NAME);
        &HTTP_HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, HeaderError>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(HeaderError::invalid)?;
        if value == "true" {
            Ok(Self(true))
        } else if value == "false" {
            Ok(Self(false))
        } else {
            Err(HeaderError::invalid())
        }
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let value = if self.0 {
            HeaderValue::from_static("true")
        } else {
            HeaderValue::from_static("false")
        };
        values.extend(std::iter::once(value));
    }
}
