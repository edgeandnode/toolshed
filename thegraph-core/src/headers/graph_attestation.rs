use alloy::primitives::B256;
use headers::{Error as HeaderError, HeaderName, HeaderValue};

use crate::attestation::Attestation;

static HEADER_NAME: HeaderName = HeaderName::from_static("graph-attestation");

/// An HTTP _typed header_ for the `graph-attestation` header.
///
/// The `graph-attestation` header can contain a JSON-encoded [`Attestation`] struct, or an empty
/// string if no attestation is provided.
///
/// When deserializing the header value, if the value is empty, it will be deserialized as `None`.
/// If the value is not empty, but cannot be deserialized as an [`Attestation`], the header is
/// considered invalid.
#[derive(Debug, Clone)]
pub struct GraphAttestation(pub Option<Attestation>);

impl headers::Header for GraphAttestation {
    fn name() -> &'static HeaderName {
        &HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, HeaderError>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        // Get the first header value, and convert it to a string.
        // If it's not present, or it's an invalid string, return an error
        let value = values
            .next()
            .ok_or_else(HeaderError::invalid)?
            .to_str()
            .map_err(|_| HeaderError::invalid())?;

        // If the value is empty, return None, otherwise try to deserialize it
        let attestation = if value.is_empty() {
            None
        } else {
            let attestation = serde_json::from_str::<'_, AttestationSerde>(value)
                .map_err(|_| HeaderError::invalid())?;
            Some(attestation.into())
        };

        Ok(Self(attestation))
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        // Serialize the attestation as a JSON string, and convert it to a `HeaderValue`.
        // If the attestation is `None`, serialize an empty string.
        let value = self
            .0
            .as_ref()
            .and_then(|att| {
                let att = serde_json::to_string(&AttestationSerde::from(att)).ok()?;
                Some(HeaderValue::from_str(att.as_str()).expect("header to be valid utf-8"))
            })
            .unwrap_or_else(|| HeaderValue::from_static(""));

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

#[cfg(all(test, feature = "fake"))]
mod tests {
    use fake::{Fake, Faker};
    use headers::{Header, HeaderValue};

    use super::{AttestationSerde, GraphAttestation};
    use crate::attestation::Attestation;

    #[test]
    fn encode_attestation_into_header() {
        //* Given
        let attestation = Faker.fake::<Attestation>();

        let mut headers = vec![];

        //* When
        let header = GraphAttestation(Some(attestation.clone()));

        header.encode(&mut headers);

        //* Then
        let value = headers
            .first()
            .expect("header to have been encoded")
            .to_str()
            .expect("header to be valid utf8");

        let att: AttestationSerde = serde_json::from_str(value).expect("header to be valid json");
        assert_eq!(attestation.request_cid, att.request_cid);
        assert_eq!(attestation.response_cid, att.response_cid);
        assert_eq!(attestation.deployment, att.deployment);
        assert_eq!(attestation.r, att.r);
        assert_eq!(attestation.s, att.s);
        assert_eq!(attestation.v, att.v);
    }

    #[test]
    fn encode_empty_attestation_header() {
        //* Given
        let mut headers = vec![];

        //* When
        let header = GraphAttestation(None);

        header.encode(&mut headers);

        //* Then
        let value = headers
            .first()
            .expect("header to have been encoded")
            .to_str()
            .expect("header to be valid utf-8");

        assert_eq!(value, "");
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
        let att = att.expect("attestation to be present");

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
        let header = GraphAttestation::decode(&mut headers.iter());

        //* Then
        let GraphAttestation(attestation) = header.expect("header to be valid");
        assert!(attestation.is_some());
    }

    #[test]
    fn decode_empty_attestation_from_valid_header() {
        //* Given
        let header = HeaderValue::from_static("");
        let headers = [header];

        //* When
        let header = GraphAttestation::decode(&mut headers.iter());

        //* Then
        let GraphAttestation(attestation) = header.expect("header to be valid");
        assert!(attestation.is_none());
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
