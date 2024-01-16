/// The preferred type for GraphQL-over-HTTP server responses. As specified in the section
/// [4.1 Media Types](https://graphql.github.io/graphql-over-http/draft/#sec-Media-Types) of the
/// GraphQL-over-HTTP specification.
pub const GRAPHQL_RESPONSE_MEDIA_TYPE: &str = "application/graphql-response+json";

/// The legacy type for GraphQL-over-HTTP server responses. As specified in the section
/// [4.1 Media Types](https://graphql.github.io/graphql-over-http/draft/#sec-Media-Types) of the
/// GraphQL-over-HTTP specification.
pub const GRAPHQL_LEGACY_RESPONSE_MEDIA_TYPE: &str = "application/json";

/// The response error type for GraphQL-over-HTTP server responses. As specified in the section
/// [7.1.2 Errors](https://spec.graphql.org/draft/#sec-Errors) and the
/// [Error Result Format](https://spec.graphql.org/draft/#sec-Errors.Error-Result-Format) subsection
/// of the GraphQL specification.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Error {
    /// A short, human-readable description of the problem.
    ///
    /// From the [Error Result Format](https://spec.graphql.org/draft/#sec-Errors.Error-Result-Format)
    /// subsection of the GraphQL specification:
    ///
    /// > Every error MUST contain an entry with the key `message` with a string description of the
    /// > error intended for the developer as a guide to understand and correct the error.
    pub message: String,

    /// A list of locations describing the beginning of the associated syntax element causing the
    /// error.
    ///
    /// From the [Error Result Format](https://spec.graphql.org/draft/#sec-Errors.Error-Result-Format)
    /// subsection of the GraphQL specification:
    ///
    /// > If an error can be associated to a particular point in the requested GraphQL document, it
    /// > SHOULD contain an entry with the key `locations` with a list of locations, where each
    /// > location is a map with the keys `line` and `column`, both positive numbers starting from
    /// `1` which describe the beginning of an associated syntax element.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<ErrorLocation>,

    /// A list of path segments starting at the root of the response and ending with the field
    /// associated with the error.
    ///
    /// From the [Error Result Format](https://spec.graphql.org/draft/#sec-Errors.Error-Result-Format)
    /// subsection of the GraphQL specification:
    ///
    /// > If an error can be associated to a particular field in the GraphQL result, it must contain
    /// > an entry with the key `path` that details the path of the response field which experienced
    /// > the error. This allows clients to identify whether a `null` result is intentional or
    /// > caused by a runtime error.
    /// >
    /// > This field should be a list of path segments starting at the root of the response and
    /// > ending with the field associated with the error. Path segments that represent fields
    /// > should be strings, and path segments that represent list indices should be 0-indexed
    /// > integers. If the error happens in an aliased field, the path to the error should use the
    /// > aliased name, since it represents a path in the response, not in the request.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub path: Vec<String>,
}

impl Error {
    /// Convert a static string into an [`Error`].
    pub fn from_static(message: &'static str) -> Self {
        Self {
            message: message.to_string(),
            locations: vec![],
            path: vec![],
        }
    }
}

/// A trait for types that can be converted into [`Error`], a GraphQL HTTP Response error.
pub trait IntoError {
    /// Convert the type into [`Error`].
    fn into_error(self) -> Error;
}

impl IntoError for Error {
    #[inline]
    fn into_error(self) -> Error {
        self
    }
}

impl<T> IntoError for T
where
    T: std::error::Error,
{
    fn into_error(self) -> Error {
        Error {
            message: self.to_string(),
            locations: vec![],
            path: vec![],
        }
    }
}

/// A location describing the beginning of the associated syntax element causing the error.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ErrorLocation {
    pub line: usize,
    pub column: usize,
}

/// A response to a GraphQL request.
///
/// As specified in the section [7. Response](https://spec.graphql.org/draft/#sec-Response) of the
/// GraphQL specification.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ResponseBody<T> {
    /// The response will be the result of the execution of the requested operation.
    ///
    /// If the operation was a query, this output will be an object of the query root operation
    /// type; if the operation was a mutation, this output will be an object of the mutation root
    /// operation type.
    ///
    /// If an error was raised before execution begins, the data entry should not be present in the
    /// result; If an error was raised during the execution that prevented a valid response, the
    /// data entry in the response should be `null`. In both cases the field will be set to `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// The errors entry in the response is a non-empty list of [`Error`] raised during the request,
    /// where each error is a map of data described by the error result specified in the section
    /// [7.1.2. Errors](https://spec.graphql.org/draft/#sec-Errors) of the GraphQL specification.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Error>,
}

impl<T> ResponseBody<T> {
    /// Create a new response body with the given data.
    pub fn from_data(data: T) -> Self {
        Self {
            data: Some(data),
            errors: vec![],
        }
    }

    /// Create a new response body with the given error.
    pub fn from_error(error: impl IntoError) -> Self {
        Self {
            data: None,
            errors: vec![error.into_error()],
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::{Error, IntoError, ResponseBody};

    /// Deserialize the given string as a GraphQL response body.
    fn deserialize_response_body<T>(response_body: &str) -> serde_json::Result<ResponseBody<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_str(response_body)
    }

    /// Serialize the given data as a GraphQL response body.
    fn serialize_response_data_body<T>(data: T) -> String
    where
        T: serde::ser::Serialize,
    {
        let response_body = ResponseBody::from_data(data);
        serde_json::to_string(&response_body).unwrap()
    }

    /// Ensure that the response body is correctly serialized when the data is a string.
    #[test]
    fn serialize_response_with_string_data() {
        //* Given
        let data = "test data";

        //* When
        let response_body = serialize_response_data_body(data);

        //* Then
        // Ensure that the response body is a valid GraphQL response.
        assert_matches!(deserialize_response_body::<String>(&response_body), Ok(resp_body) => {
            // The data should be returned
            assert_eq!(resp_body.data, Some("test data".to_string()));

            // There should be no errors
            assert_eq!(resp_body.errors.len(), 0);
        });
    }

    /// Test data type implementing the serde traits.
    ///
    /// See [`serialize_response_with_struct_data`] test.
    #[derive(Debug, serde::Deserialize, serde::Serialize)]
    struct Data {
        field: String,
    }

    /// Ensure that the response body is correctly serialized when the data is a struct.
    #[test]
    fn serialize_response_with_struct_data() {
        //* Given
        let data = Data {
            field: "test data".to_string(),
        };

        //* When
        let response_body = serialize_response_data_body(data);

        //* Then
        // Ensure that the response body is a valid GraphQL response.
        assert_matches!(deserialize_response_body::<Data>(&response_body), Ok(resp_body) => {
            // There should be no errors
            assert_eq!(resp_body.errors.len(), 0);

            // The data should be returned
            assert_matches!(resp_body.data, Some(data) => {
                assert_eq!(data.field, "test data");
            });
        });
    }

    /// Serialize the given error as a GraphQL error response body.
    fn serialize_error_response_body(error: impl IntoError) -> String {
        let response_body = ResponseBody::<()>::from_error(error);
        serde_json::to_string(&response_body).unwrap()
    }

    /// Ensure that the error response body is correctly serialized when the error is a string.
    #[test]
    fn serialize_response_with_error_from_static() {
        //* Given
        let error = Error::from_static("test error message");

        //* When
        let response_body = serialize_error_response_body(error);

        //* Then
        // Ensure that the response body is a valid GraphQL error response.
        assert_matches!(deserialize_response_body::<()>(&response_body), Ok(resp_body) => {
            // No data should be returned
            assert_eq!(resp_body.data, None);

            // There should be one error
            assert_eq!(resp_body.errors.len(), 1);
            assert_eq!(resp_body.errors[0].message, "test error message");
        });
    }

    /// Test error type implementing the `std::error::Error` trait.
    ///
    /// Se [`serialize_response_with_struct_implementing_error_trait`] test.
    #[derive(Debug, thiserror::Error)]
    #[error("test error: {cause}")]
    struct TestError {
        cause: String,
    }

    /// Ensure that the error response body is correctly serialized when the error is an object
    /// implementing the `std::error::Error` trait.
    #[test]
    fn serialize_response_with_struct_implementing_error_trait() {
        //* Given
        let error = TestError {
            cause: "test message".to_string(),
        };

        //* When
        let response_body = serialize_error_response_body(error);

        //* Then
        // Ensure that the response body is a valid GraphQL error response.
        assert_matches!(deserialize_response_body::<()>(&response_body), Ok(resp_body) => {
            // No data should be returned
            assert_eq!(resp_body.data, None);

            // There should be one error
            assert_eq!(resp_body.errors.len(), 1);
            assert_eq!(resp_body.errors[0].message, "test error: test message");
        });
    }
}
