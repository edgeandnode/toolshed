//! An HTTP _typed header_ for the `graph-attestation` header.
//!
//! The `graph-attestation` header can contain a JSON-encoded [`Attestation`] struct, or an empty
//! string if no attestation is provided.
//!
//! # Using the `headers::HeaderMapExt` extension trait
//!
//! ```rust
//! # use fake::{Fake, Faker};
//! use headers::HeaderMapExt as _;
//! use thegraph_headers::graph_attestation::{GraphAttestation, HEADER_NAME};
//!
//! let mut header_map = http::HeaderMap::new();
//! # let value = Faker.fake::<thegraph_headers::graph_attestation::Attestation>();
//!
//! // Insert a `graph-attestation` HTTP header
//! header_map.typed_insert(GraphAttestation(value));
//!
//! // Get the `graph-attestation` HTTP header by name
//! let header_by_name = header_map.get(HEADER_NAME);
//! assert!(header_by_name.is_some());
//!
//! // Get the `graph-attestation` HTTP header by type
//! let header_typed = header_map.typed_get::<GraphAttestation>();
//! assert!(matches!(header_typed, Some(GraphAttestation(..))));
//! ```

use headers::{Error as HeaderError, HeaderName, HeaderValue};
use thegraph_core::alloy::primitives::B256;
pub use thegraph_core::attestation::Attestation;

/// The HTTP header name for the `graph-attestation` header.
pub const HEADER_NAME: &str = "graph-attestation";

/// An HTTP _typed header_ for the `graph-attestation` header.
///
/// The `graph-attestation` header can contain a JSON-encoded [`Attestation`] struct, or an empty
/// string if no attestation is provided.
#[derive(Debug, Clone)]
pub struct GraphAttestation(pub Attestation);

impl headers::Header for GraphAttestation {
    fn name() -> &'static HeaderName {
        static HTTP_HEADER_NAME: HeaderName = HeaderName::from_static(HEADER_NAME);
        &HTTP_HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, HeaderError>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        // Get the first header value, and try to deserialize it into an `Attestation`.
        let value = values.next().ok_or_else(HeaderError::invalid)?;
        let attestation = serde_json::from_slice::<'_, AttestationSerde>(value.as_bytes())
            .map_err(|_| HeaderError::invalid())?;
        Ok(Self(attestation.into()))
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        // Serialize the attestation as a JSON string, and convert it to a `HeaderValue`.
        let bytes =
            serde_json::to_vec(&AttestationSerde::from(&self.0)).expect("header to be valid json");
        let value = HeaderValue::from_bytes(&bytes).expect("header to be valid utf-8");
        values.extend(std::iter::once(value));
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AttestationSerde {
    #[serde(rename = "requestCID")]
    request_cid: B256,
    #[serde(rename = "responseCID")]
    response_cid: B256,
    #[serde(rename = "subgraphDeploymentID")]
    deployment: B256,
    r: B256,
    s: B256,
    v: u8,
}

impl From<AttestationSerde> for Attestation {
    fn from(value: AttestationSerde) -> Self {
        Self {
            request_cid: value.request_cid,
            response_cid: value.response_cid,
            deployment: value.deployment,
            r: value.r,
            s: value.s,
            v: value.v,
        }
    }
}

impl From<&Attestation> for AttestationSerde {
    fn from(value: &Attestation) -> Self {
        Self {
            request_cid: value.request_cid,
            response_cid: value.response_cid,
            deployment: value.deployment,
            r: value.r,
            s: value.s,
            v: value.v,
        }
    }
}

#[cfg(test)]
mod tests {
    use fake::{Fake, Faker};
    use headers::{Header, HeaderValue};
    use thegraph_core::attestation::Attestation;

    use super::{AttestationSerde, GraphAttestation};

    #[test]
    fn encode_attestation_into_header() {
        //* Given
        let attestation = Faker.fake::<Attestation>();

        let mut headers = vec![];

        //* When
        let header = GraphAttestation(attestation.clone());

        header.encode(&mut headers);

        //* Then
        let value = headers.first().expect("header to have been encoded");

        let att: AttestationSerde =
            serde_json::from_slice(value.as_bytes()).expect("header to be valid json");
        assert_eq!(attestation.request_cid, att.request_cid);
        assert_eq!(attestation.response_cid, att.response_cid);
        assert_eq!(attestation.deployment, att.deployment);
        assert_eq!(attestation.r, att.r);
        assert_eq!(attestation.s, att.s);
        assert_eq!(attestation.v, att.v);
    }

    #[test]
    fn decode_attestation_from_valid_header() {
        //* Given
        let attestation = Faker.fake::<Attestation>();

        let header = {
            let value = serde_json::to_string(&AttestationSerde::from(&attestation)).unwrap();
            HeaderValue::from_str(value.as_str()).unwrap()
        };
        let headers = [header];

        //* When
        let header = GraphAttestation::decode(&mut headers.iter());

        //* Then
        let GraphAttestation(att) = header.expect("header to be valid");

        assert_eq!(attestation.request_cid, att.request_cid);
        assert_eq!(attestation.response_cid, att.response_cid);
        assert_eq!(attestation.deployment, att.deployment);
        assert_eq!(attestation.r, att.r);
        assert_eq!(attestation.s, att.s);
        assert_eq!(attestation.v, att.v);
    }

    #[test]
    fn decode_attestation_from_first_header() {
        //* Given
        let attestation = Faker.fake::<Attestation>();

        let header = {
            let value = serde_json::to_string(&AttestationSerde::from(&attestation)).unwrap();
            HeaderValue::from_str(&value).unwrap()
        };
        let headers = [
            header,
            HeaderValue::from_static("invalid"),
            HeaderValue::from_static(""),
        ];

        //* When
        let result = GraphAttestation::decode(&mut headers.iter());

        //* Then
        assert!(result.is_ok());
    }

    #[test]
    fn fail_decode_attestation_from_empty_string_header() {
        //* Given
        let header = HeaderValue::from_static("");
        let headers = [header];

        //* When
        let result = GraphAttestation::decode(&mut headers.iter());

        //* Then
        assert!(result.is_err());
    }

    #[test]
    fn fail_decode_attestation_from_invalid_header() {
        //* Given
        let header = HeaderValue::from_static("invalid");
        let headers = [header];

        //* When
        let header = GraphAttestation::decode(&mut headers.iter());

        //* Then
        assert!(header.is_err());
    }

    #[test]
    fn fail_decode_attestation_if_no_headers() {
        //* Given
        let headers = [];

        //* When
        let header = GraphAttestation::decode(&mut headers.iter());

        //* Then
        assert!(header.is_err());
    }
}
