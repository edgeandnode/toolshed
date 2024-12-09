//! Compatibility layer for third-party GraphQL libraries.

#[cfg(feature = "graphql-parser")]
pub mod compat_graphql_parser {
    use graphql_parser::query::Text;

    use crate::graphql::{Document, IntoDocument};

    // Implement `IntoRequestParameters` for `graphql_parser::query::Document` so that we can use
    // `graphql_parser` to parse GraphQL queries.
    //
    // As any type implementing `IntoQuery` also implements `IntoRequestParameters`, this allows us to
    // seamlessly support `graphql_parser` generated queries.
    impl<'a, T: Text<'a>> IntoDocument for graphql_parser::query::Document<'a, T> {
        fn into_document(self) -> Document {
            Document::new(self.to_string())
        }
    }
}

#[cfg(feature = "graphql-client")]
pub mod compat_graphql_client {
    use graphql_client::QueryBody;

    use crate::{
        graphql::IntoDocument,
        http::request::{IntoRequestParameters, RequestParameters},
    };

    // Implement `IntoRequestParameters` for `graphql_client::QueryBody` so that we can seamlessly
    // support `graphql_client` generated queries.
    impl<V> IntoRequestParameters for QueryBody<V>
    where
        V: serde::ser::Serialize,
    {
        fn into_request_parameters(self) -> RequestParameters {
            let query = self.query.into_document();

            // Do not send the `operation_name` field if it is empty.
            let operation_name = if !self.operation_name.is_empty() {
                Some(self.operation_name.to_owned())
            } else {
                None
            };

            // Do not send the `variables` field if the json serialization fails, or if the
            // serialization result is not a JSON object.
            let variables = match serde_json::to_value(self.variables) {
                Ok(serde_json::Value::Object(vars)) => Some(vars),
                _ => None,
            };

            RequestParameters {
                query,
                operation_name,
                variables: variables.unwrap_or_default(),
                extensions: Default::default(),
            }
        }
    }
}

#[cfg(feature = "async-graphql")]
pub mod compat_async_graphql {
    use async_graphql::Request;

    use crate::{
        graphql::IntoDocument,
        http::request::{IntoRequestParameters, RequestParameters},
    };

    // Implement `IntoRequestParameters` for `async_graphql::Request` so that we can seamlessly
    // support `async_graphql` requests.
    impl IntoRequestParameters for Request {
        fn into_request_parameters(self) -> RequestParameters {
            let query = self.query.into_document();

            // Serialize the `async_graphql::Request` _variables_ and _extensions_ fields to JSON
            // objects. If the serialization fails, or if the result is not a JSON object, avoid
            // sending an invalid JSON to the server.
            let variables = match serde_json::to_value(self.variables) {
                Ok(serde_json::Value::Object(vars)) => vars,
                _ => Default::default(),
            };

            let extensions = match serde_json::to_value(self.extensions) {
                Ok(serde_json::Value::Object(vars)) => vars,
                _ => Default::default(),
            };

            RequestParameters {
                query,
                operation_name: self.operation_name,
                variables,
                extensions,
            }
        }
    }
}
