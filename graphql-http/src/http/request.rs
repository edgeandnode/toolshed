use crate::graphql::{Document, IntoDocumentWithVariables};

/// The media type for GraphQL-over-HTTP requests. As specified in the section
/// [4.1 Media Types](https://graphql.github.io/graphql-over-http/draft/#sec-Media-Types)
/// of the GraphQL-over-HTTP specification.
pub const GRAPHQL_REQUEST_MEDIA_TYPE: &str = "application/json";

/// The parameters of a GraphQL-over-HTTP request.
///
/// As specified in the section [5.1 Request Parameters](https://graphql.github.io/graphql-over-http/draft/#sec-Request-Parameters)
/// of the GraphQL-over-HTTP specification.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct RequestParameters {
    /// The string representation of the Source Text of a GraphQL Document as specified in [the
    /// Language section of the GraphQL specification](https://spec.graphql.org/draft/#sec-Language).
    pub query: Document,

    /// Optional name of the operation in the Document to execute.
    #[serde(rename = "operationName", skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,

    /// Values for any Variables defined by the Operation.
    #[serde(skip_serializing_if = "serde_json::Map::is_empty")]
    pub variables: serde_json::Map<String, serde_json::Value>,

    /// Reserved for implementors to extend the protocol however they see fit.
    #[serde(skip_serializing_if = "serde_json::Map::is_empty")]
    pub extensions: serde_json::Map<String, serde_json::Value>,
}

/// Convert `self` into a `RequestParameters` struct.
pub trait IntoRequestParameters {
    /// Consumes `self` and returns a `RequestParameters` struct.
    fn into_request_parameters(self) -> RequestParameters;
}

impl IntoRequestParameters for RequestParameters {
    fn into_request_parameters(self) -> RequestParameters {
        self
    }
}

// Any type implementing `IntoDocumentWithVariables` (or `IntoDocument`) can be converted into
// `RequestParameters`.
impl<T> IntoRequestParameters for T
where
    T: IntoDocumentWithVariables,
{
    fn into_request_parameters(self) -> RequestParameters {
        let (query, variables) = self.into_document_with_variables();

        // Do not send the `variables` field if the json serialization fails, or if the
        // serialization result is not a JSON object.
        let variables = match serde_json::to_value(variables) {
            Ok(serde_json::Value::Object(vars)) => Some(vars),
            _ => None,
        };

        RequestParameters {
            query,
            operation_name: None,
            variables: variables.unwrap_or_default(),
            extensions: Default::default(),
        }
    }
}
