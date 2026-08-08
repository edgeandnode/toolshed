//! HTTP client extensions.

#[cfg(feature = "reqwest")]
#[cfg_attr(docsrs, doc(cfg(feature = "reqwest")))]
pub use reqwest_ext::ReqwestExt;

use crate::http::response::{Error, ResponseBody};

/// The error type returned by `ReqwestExt`
#[cfg(feature = "reqwest")]
#[cfg_attr(docsrs, doc(cfg(feature = "reqwest")))]
#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    /// An error occurred while serializing the GraphQL request.
    #[error("Error serializing GraphQL request parameters: {0}")]
    RequestSerializationError(serde_json::Error),

    /// An error occurred while making the HTTP request.
    #[error("Error making HTTP request: {0}")]
    RequestSendError(#[from] reqwest::Error),

    /// An error occurred while receiving the HTTP response.
    #[error("Error receiving HTTP response ({0}): {1}")]
    ResponseRecvError(reqwest::StatusCode, String),

    /// An error occurred while deserializing the GraphQL response.
    #[error(
        "Error deserializing GraphQL response. Unexpected response: {response}. Error: {error}"
    )]
    ResponseDeserializationError {
        error: serde_json::Error,
        response: String,
    },
}

/// The possible errors results of a GraphQL-over-HTTP response.
#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    /// The GraphQL response is empty.
    #[error("Empty response")]
    Empty,

    /// The GraphQL request failed.
    #[error("GraphQL request failed: {errors:?}")]
    Failure {
        /// A list of errors returned by the server.
        errors: Vec<Error>,
    },
}

/// The result type of GraphQL-over-HTTP request.
pub type ResponseResult<ResponseData> = Result<ResponseData, ResponseError>;

/// Process the GraphQL response body.
fn process_response_body<ResponseData>(
    resp: ResponseBody<ResponseData>,
) -> ResponseResult<ResponseData>
where
    ResponseData: serde::de::DeserializeOwned,
{
    // [7.1.2 Errors](https://spec.graphql.org/draft/#sec-Errors)
    //
    // > If present, the `errors` entry in the response must contain at least one error. If no
    // > `errors` were raised during the request, the errors entry must not be present in the
    // > result.
    //
    // > If the `data` entry in the response is not present, the `errors` entry MUST be present. It
    // > MUST contain at least one _request error_ indicating why no data was able to be returned.
    //
    // > If the data entry in the response is present (including if it is the value **null**), the
    // > `errors` entry MUST be present if and only if one or more _field error_ was raised during
    // > execution.
    match (resp.data, resp.errors) {
        (Some(data), errors) if errors.is_empty() => Ok(data),
        (None, errors) if errors.is_empty() => Err(ResponseError::Empty),
        // Do not consider partial responses
        (_, errors) => Err(ResponseError::Failure { errors }),
    }
}

#[cfg(feature = "reqwest")]
mod reqwest_ext {
    use async_trait::async_trait;
    use reqwest::header::{ACCEPT, CONTENT_TYPE};

    use super::{RequestError, ResponseResult, process_response_body};
    use crate::http::{
        request::{GRAPHQL_REQUEST_MEDIA_TYPE, IntoRequestParameters},
        response::{GRAPHQL_LEGACY_RESPONSE_MEDIA_TYPE, GRAPHQL_RESPONSE_MEDIA_TYPE},
    };

    /// An extension trait for reqwest::RequestBuilder.
    #[cfg(feature = "reqwest")]
    #[cfg_attr(docsrs, doc(cfg(feature = "reqwest")))]
    #[async_trait]
    pub trait ReqwestExt {
        /// Sets the `Content-Type` and `Accept` headers to the GraphQL-over-HTTP media types and
        /// serializes the GraphQL request.
        ///
        /// If the GraphQL request cannot be serialized, an error is returned.
        fn graphql(self, req: impl IntoRequestParameters) -> Result<Self, serde_json::Error>
        where
            Self: Sized;

        /// Runs a GraphQL query with the parameters in RequestBuilder, deserializes
        /// the body and returns the result.
        async fn send_graphql<ResponseData>(
            self,
            req: impl IntoRequestParameters + Send,
        ) -> Result<ResponseResult<ResponseData>, RequestError>
        where
            ResponseData: serde::de::DeserializeOwned;
    }

    #[cfg(feature = "reqwest")]
    #[cfg_attr(docsrs, doc(cfg(feature = "reqwest")))]
    #[async_trait]
    impl ReqwestExt for reqwest::RequestBuilder {
        fn graphql(self, req: impl IntoRequestParameters) -> Result<Self, serde_json::Error>
        where
            Self: Sized,
        {
            let gql_request = req.into_request_parameters();
            let gql_request_body = serde_json::to_vec(&gql_request)?;

            let builder = self
                // Set `Content-Type` header to `application/json` as specified in the section
                // [5.4 POST](https://graphql.github.io/graphql-over-http/draft/#sec-POST) of the
                // GraphQL-over-HTTP specification.
                .header(CONTENT_TYPE, GRAPHQL_REQUEST_MEDIA_TYPE)
                // Set `Accept` header to `application/json` and `application/graphql-response+json` to
                // support both the legacy and the current GraphQL-over-HTTP media types. As specified in
                // the section [5.2.1 Legacy Watershed](https://graphql.github.io/graphql-over-http/draft/#sec-Legacy-Watershed)
                // of the GraphQL-over-HTTP specification.
                .header(
                    ACCEPT,
                    format!(
                        "{GRAPHQL_RESPONSE_MEDIA_TYPE}; charset=utf-8, {GRAPHQL_LEGACY_RESPONSE_MEDIA_TYPE}; charset=utf-8"
                    ),
                )
                .body(gql_request_body);

            Ok(builder)
        }

        async fn send_graphql<ResponseData>(
            self,
            req: impl IntoRequestParameters + Send,
        ) -> Result<ResponseResult<ResponseData>, RequestError>
        where
            ResponseData: serde::de::DeserializeOwned,
        {
            let builder = self
                .graphql(req)
                .map_err(RequestError::RequestSerializationError)?;

            match builder.send().await {
                Ok(response) => {
                    // Process a GraphQL-over-HTTP response.
                    if !is_legacy_response(&response) {
                        process_graphql_response(response).await
                    } else {
                        process_legacy_graphql_response(response).await
                    }
                }
                Err(e) => Err(RequestError::RequestSendError(e)),
            }
        }
    }

    /// Determine if the response is a GraphQL-over-HTTP response using the legacy media type.
    fn is_legacy_response(response: &reqwest::Response) -> bool {
        let content_type = response.headers().get(CONTENT_TYPE);
        match content_type {
            // If the `Content-Type` header is present, check if it is the legacy response media
            // type or the current GraphQL-over-HTTP response media type.
            Some(header) => header
                .as_bytes()
                .eq_ignore_ascii_case(GRAPHQL_LEGACY_RESPONSE_MEDIA_TYPE.as_bytes()),
            // If no `Content-Type` header is present, the response SHOULD be interpreted as if the
            // header field had the value `application/json` (legacy media type).
            None => true,
        }
    }

    /// Process the GraphQL-over-HTTP response when the media type, `application/graphql-response+json`,
    /// is used.
    ///
    /// See the section [6.4.2 application/graphql-response+json](
    /// https://graphql.github.io/graphql-over-http/draft/#sec-application-graphql-response-json)
    /// of the GraphQL-over-HTTP specification for more information.
    async fn process_graphql_response<ResponseData>(
        resp: reqwest::Response,
    ) -> Result<ResponseResult<ResponseData>, RequestError>
    where
        ResponseData: serde::de::DeserializeOwned,
    {
        let status = resp.status();

        // [6.4.2 application/graphql-response+json](https://graphql.github.io/graphql-over-http/draft/#sec-application-graphql-response-json)
        //
        // > Clients should process a response using the `application/graphql-response+json` media
        // > type as a well-formed GraphQL response independent of the HTTP status code.
        //
        // > In case of errors that completely prevent the generation of a well-formed GraphQL
        // > response, the server SHOULD respond with the appropriate HTTP `4xx` or `5xx` status
        // > code depending on the concrete error condition, and MUST NOT use the
        // > `application/graphql-response+json` media type.
        //
        // Since this response already declared the `application/graphql-response+json` media type,
        // its body is guaranteed by the spec to be a well-formed GraphQL response regardless of the
        // HTTP status code, so unlike the legacy media type there is no need to gate on it here.
        read_and_process_response_body(status, resp).await
    }

    /// Process the GraphQL-over-HTTP response when the legacy media type, `application/json`, is used.
    ///
    /// See the section [6.4.1 application/json](https://graphql.github.io/graphql-over-http/draft/#sec-application-json)
    /// of the GraphQL-over-HTTP specification for more information.
    async fn process_legacy_graphql_response<ResponseData>(
        resp: reqwest::Response,
    ) -> Result<ResponseResult<ResponseData>, RequestError>
    where
        ResponseData: serde::de::DeserializeOwned,
    {
        let status = resp.status();

        // [6.4.1 application/json](https://graphql.github.io/graphql-over-http/draft/#sec-application-json)
        //
        // > The server SHOULD use the 200 status code for every response to a well-formed
        // > GraphQL-over-HTTP request, independent of any GraphQL request error or GraphQL field error
        // > raised.
        //
        // > For compatibility with legacy servers, this specification allows the use of `4xx` or `5xx`
        // > status codes for a failed well-formed GraphQL-over-HTTP request where the response uses
        // > the `application/json` media type, but it is **strongly discouraged**.
        if !status.is_success() && !status.is_client_error() && !status.is_server_error() {
            return Err(RequestError::ResponseRecvError(
                status,
                resp.text()
                    .await
                    .unwrap_or_else(|_| "Empty response body".to_string()),
            ));
        }

        read_and_process_response_body(status, resp).await
    }

    /// Read and deserialize the GraphQL-over-HTTP response body, then process it.
    async fn read_and_process_response_body<ResponseData>(
        status: reqwest::StatusCode,
        resp: reqwest::Response,
    ) -> Result<ResponseResult<ResponseData>, RequestError>
    where
        ResponseData: serde::de::DeserializeOwned,
    {
        // Receive the response body.
        let response = resp.bytes().await.map_err(|err| {
            RequestError::ResponseRecvError(status, format!("Error reading response body: {err}"))
        })?;

        if response.is_empty() {
            return Err(RequestError::ResponseRecvError(
                status,
                "Empty response body".to_string(),
            ));
        }

        // Deserialize the response body.
        let response = serde_json::from_slice(&response).map_err(|error| {
            RequestError::ResponseDeserializationError {
                error,
                response: String::from_utf8_lossy(&response).to_string(),
            }
        })?;

        Ok(process_response_body(response))
    }

    #[cfg(test)]
    mod tests {
        use assert_matches::assert_matches;

        use super::*;
        use crate::http::response::{
            GRAPHQL_LEGACY_RESPONSE_MEDIA_TYPE, GRAPHQL_RESPONSE_MEDIA_TYPE,
        };
        use crate::http_client::ResponseError;

        /// Build a `reqwest::Response` in-memory (no network I/O) with the given status,
        /// `Content-Type` header and body.
        fn build_response(
            status: u16,
            content_type: Option<&str>,
            body: &str,
        ) -> reqwest::Response {
            let mut builder = http::Response::builder().status(status);
            if let Some(content_type) = content_type {
                builder = builder.header(CONTENT_TYPE, content_type);
            }
            let response = builder.body(body.to_string()).unwrap();
            reqwest::Response::from(response)
        }

        #[derive(Debug, serde::Deserialize, PartialEq)]
        struct Data {
            value: String,
        }

        #[test]
        fn legacy_response_detected_from_content_type() {
            let resp = build_response(200, Some(GRAPHQL_LEGACY_RESPONSE_MEDIA_TYPE), "{}");
            assert!(is_legacy_response(&resp));

            let resp = build_response(200, Some(GRAPHQL_RESPONSE_MEDIA_TYPE), "{}");
            assert!(!is_legacy_response(&resp));

            // No `Content-Type` header SHOULD be interpreted as the legacy media type.
            let resp = build_response(200, None, "{}");
            assert!(is_legacy_response(&resp));
        }

        #[tokio::test]
        async fn graphql_response_media_type_with_data_is_processed() {
            let body = r#"{"data":{"value":"hello"}}"#;
            let resp = build_response(200, Some(GRAPHQL_RESPONSE_MEDIA_TYPE), body);

            let result = process_graphql_response::<Data>(resp).await;

            assert_matches!(result, Ok(Ok(data)) => {
                assert_eq!(data, Data { value: "hello".to_string() });
            });
        }

        /// Per spec §6.4.2, a response using the `application/graphql-response+json` media type is
        /// guaranteed to carry a well-formed GraphQL response body regardless of the HTTP status
        /// code, so the body must still be parsed even on a `4xx`/`5xx` status.
        #[tokio::test]
        async fn graphql_response_media_type_ignores_status_code() {
            let body = r#"{"errors":[{"message":"not found"}]}"#;
            let resp = build_response(404, Some(GRAPHQL_RESPONSE_MEDIA_TYPE), body);

            let result = process_graphql_response::<Data>(resp).await;

            assert_matches!(result, Ok(Err(ResponseError::Failure { errors })) => {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].message, "not found");
            });
        }

        /// Unlike the graphql-response+json path, the legacy media type bails out on statuses that
        /// are neither success, client error nor server error (e.g. a redirect) without attempting
        /// to parse the body, per the compatibility rationale in spec §6.4.1.
        #[tokio::test]
        async fn legacy_response_media_type_rejects_unexpected_status_class() {
            let body = r#"{"data":{"value":"hello"}}"#;
            let resp = build_response(304, Some(GRAPHQL_LEGACY_RESPONSE_MEDIA_TYPE), body);

            let result = process_legacy_graphql_response::<Data>(resp).await;

            assert_matches!(result, Err(RequestError::ResponseRecvError(status, _)) => {
                assert_eq!(status.as_u16(), 304);
            });
        }

        #[tokio::test]
        async fn graphql_response_media_type_empty_body_errors() {
            let resp = build_response(200, Some(GRAPHQL_RESPONSE_MEDIA_TYPE), "");

            let result = process_graphql_response::<Data>(resp).await;

            assert_matches!(result, Err(RequestError::ResponseRecvError(status, _)) => {
                assert_eq!(status.as_u16(), 200);
            });
        }
    }
}
