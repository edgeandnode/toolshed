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
    pub path: Vec<String>,
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
#[derive(Debug, serde::Deserialize)]
pub struct ResponseBody<T> {
    /// The e response will be the result of the execution of the requested operation.
    ///
    /// If the operation was a query, this output will be an object of the query root operation
    /// type; if the operation was a mutation, this output will be an object of the mutation root
    /// operation type.
    ///
    /// If an error was raised before execution begins, the data entry should not be present in the
    /// result; If an error was raised during the execution that prevented a valid response, the
    /// data entry in the response should be `null`. In both cases the field will be set to `None`.
    pub data: Option<T>,

    /// The errors entry in the response is a non-empty list of [`Error`] raised during the request,
    /// where each error is a map of data described by the error result specified in the section
    /// [7.1.2. Errors](https://spec.graphql.org/draft/#sec-Errors) of the GraphQL specification.
    #[serde(default)]
    pub errors: Vec<Error>,
}
