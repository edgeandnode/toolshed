//! An HTTP _typed header_ for the `graph-attestable` header.
//!
//! This HTTP header is used to indicate whether a response is _attestable_ or not.

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
