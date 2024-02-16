use std::time::Duration;

use assert_matches::assert_matches;
use reqwest::Url;

use graphql_http::graphql::IntoDocument;
use graphql_http::http::request::{IntoRequestParameters, RequestParameters};
use graphql_http::http_client::{ReqwestExt, ResponseError};

/// The URL of the test server.
///
/// This a GraphQL server that implements the [Star Wars API](https://swapi.dev/). See
/// https://github.com/graphql/swapi-graphql for more information.
const TEST_SERVER_URL: &str = "https://swapi-graphql.netlify.app/.netlify/functions/index";

#[tokio::test]
async fn send_valid_graphql_request_no_variables() {
    //* Given
    let client = reqwest::Client::new();
    let server_url: Url = TEST_SERVER_URL.parse().unwrap();

    // GraphQL query (string slice)
    let query = indoc::indoc! {
        r#"{
            allFilms {
                films {
                    title
                    director
                    releaseDate
                }
            }
        }"#
    };

    // Response types
    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct QueryResponse {
        all_films: AllFilms,
    }

    #[derive(Debug, serde::Deserialize)]
    struct AllFilms {
        films: Vec<Film>,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Film {
        title: String,
        director: String,
        release_date: String,
    }

    //* When
    let req_fut = client.post(server_url).send_graphql::<QueryResponse>(query);
    let response = tokio::time::timeout(Duration::from_secs(30), req_fut)
        .await
        .expect("Request timed out")
        .expect("Request failed");

    //* Then
    assert_matches!(response, Ok(QueryResponse { all_films: AllFilms { films } }) => {
        assert_eq!(films.len(), 6);

        assert_matches!(films.iter().find(|film| film.title == "A New Hope"), Some(film) => {
            assert_eq!(film.title, "A New Hope");
            assert_eq!(film.director, "George Lucas");
            assert_eq!(film.release_date, "1977-05-25");
        });
        assert_matches!(films.iter().find(|film| film.title == "The Empire Strikes Back"), Some(film) => {
            assert_eq!(film.title, "The Empire Strikes Back");
            assert_eq!(film.director, "Irvin Kershner");
            assert_eq!(film.release_date, "1980-05-17");
        });
        assert_matches!(films.iter().find(|film| film.title == "Return of the Jedi"), Some(film) => {
            assert_eq!(film.title, "Return of the Jedi");
            assert_eq!(film.director, "Richard Marquand");
            assert_eq!(film.release_date, "1983-05-25");
        });
    });
}

#[tokio::test]
async fn send_valid_graphql_request_with_variables() {
    //* Given
    let client = reqwest::Client::new();
    let server_url: Url = TEST_SERVER_URL.parse().unwrap();

    // GraphQL request
    #[derive(Debug)]
    struct FilmRequest {
        id: String,
    }

    impl FilmRequest {
        fn new(id: u64) -> Self {
            Self { id: id.to_string() }
        }
    }

    impl IntoRequestParameters for FilmRequest {
        fn into_request_parameters(self) -> RequestParameters {
            // GraphQL query (string slice)
            let query = indoc::indoc! {
                r#"query filmByFilmId($id: ID!) {
                    film(filmID: $id) {
                        title
                        director
                        releaseDate
                    }
                }"#
            };

            RequestParameters {
                query: query.into_document(),
                operation_name: None,
                variables: serde_json::Map::from_iter([("id".to_string(), self.id.into())]),
                extensions: Default::default(),
            }
        }
    }

    // Response types
    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct QueryResponse {
        film: Film,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Film {
        title: String,
        director: String,
        release_date: String,
    }

    //* When
    let req_fut = client
        .post(server_url)
        .send_graphql::<QueryResponse>(FilmRequest::new(1));
    let response = tokio::time::timeout(Duration::from_secs(30), req_fut)
        .await
        .expect("Request timed out")
        .expect("Request failed");

    //* Then
    assert_matches!(response, Ok(QueryResponse { film }) => {
        assert_eq!(film.title, "A New Hope");
        assert_eq!(film.director, "George Lucas");
        assert_eq!(film.release_date, "1977-05-25");
    });
}

// https://graphql.github.io/graphql-over-http/draft/#sec-application-json.Examples.Document-parsing-failure
#[tokio::test]
async fn send_invalid_request_document_parsing_failure() {
    //* Given
    let client = reqwest::Client::new();
    let server_url: Url = TEST_SERVER_URL.parse().unwrap();

    // GraphQL query (string slice)
    let query = "{";

    // Response types (dummy)
    #[derive(Debug, serde::Deserialize)]
    struct QueryResponse {}

    //* When
    let req_fut = client.post(server_url).send_graphql::<QueryResponse>(query);
    let response = tokio::time::timeout(Duration::from_secs(30), req_fut)
        .await
        .expect("Request timed out")
        .expect("Request failed");

    //* Then
    assert_matches!(response, Err(err) => {
        assert!(err.to_string().contains("Syntax Error"));
    });
}

// https://graphql.github.io/graphql-over-http/draft/#sec-application-json.Examples.Field-errors-encountered-during-execution
#[tokio::test]
async fn send_invalid_request_field_errors_encountered_during_execution_failure() {
    //* Given
    let client = reqwest::Client::new();
    let server_url: Url = TEST_SERVER_URL.parse().unwrap();

    // GraphQL query (string slice)
    let query = indoc::indoc! {
        r#"{
            allFilms {
                films {
                    title
                    director
                    releaseDate
                    invalidField
                }
            }
        }"#
    };

    // Response types (dummy)
    #[derive(Debug, serde::Deserialize)]
    struct QueryResponse {}

    //* When
    let req_fut = client.post(server_url).send_graphql::<QueryResponse>(query);
    let response = tokio::time::timeout(Duration::from_secs(30), req_fut)
        .await
        .expect("Request timed out")
        .expect("Request failed");

    //* Then
    assert_matches!(response, Err(ResponseError::Failure { errors }) => {
        assert_eq!(errors.len(), 1);

        assert!(errors[0].message.contains(r#"Cannot query field "invalidField" on type "Film""#));
    });
}

// https://graphql.github.io/graphql-over-http/draft/#sec-application-json.Examples.Operation-cannot-be-determined
#[tokio::test]
async fn send_invalid_request_operation_cannot_be_determined_failure_null_operation_name() {
    //* Given
    let client = reqwest::Client::new();
    let server_url: Url = TEST_SERVER_URL.parse().unwrap();

    // GraphQL query (string slice)
    // Scenario: The operation name is null, but the document contains multiple operations.
    let query = indoc::indoc! {
        r#"
            query filmsWithDirector {
                allFilms {
                    films {
                        title
                        director
                    }
                }
            }
            
            query filsmWithReleaseDate {
                allFilms {
                    films {
                        title
                        releaseDate
                    }
                }
            }
        "#
    };

    let request_params = RequestParameters {
        query: query.into_document(),
        operation_name: None, // Null operation name
        variables: Default::default(),
        extensions: Default::default(),
    };

    // Response types (dummy)
    #[derive(Debug, serde::Deserialize)]
    struct QueryResponse {}

    //* When
    let req_fut = client
        .post(server_url)
        .send_graphql::<QueryResponse>(request_params);
    let response = tokio::time::timeout(Duration::from_secs(30), req_fut)
        .await
        .expect("Request timed out")
        .expect("Request failed");

    //* Then
    assert_matches!(response, Err(err) => {
        assert!(err.to_string().contains(r#"Must provide operation name if query contains multiple operations"#));
    });
}

// https://graphql.github.io/graphql-over-http/draft/#sec-application-json.Examples.Operation-cannot-be-determined
#[tokio::test]
async fn send_invalid_request_operation_cannot_be_determined_failure_invalid_operation_name() {
    //* Given
    let client = reqwest::Client::new();
    let server_url: Url = TEST_SERVER_URL.parse().unwrap();

    // GraphQL query (string slice)
    // Scenario: The operation name is not found in the document.
    let query = indoc::indoc! {
        r#"
            query filmsWithDirector {
                allFilms {
                    films {
                        title
                        director
                    }
                }
            }
            
            query filsmWithReleaseDate {
                allFilms {
                    films {
                        title
                        releaseDate
                    }
                }
            }
        "#
    };

    let request_params = RequestParameters {
        query: query.into_document(),
        operation_name: Some("invalidOperationName".to_string()), // Invalid operation name
        variables: Default::default(),
        extensions: Default::default(),
    };

    // Response types (dummy)
    #[derive(Debug, serde::Deserialize)]
    struct QueryResponse {}

    //* When
    let req_fut = client
        .post(server_url)
        .send_graphql::<QueryResponse>(request_params);
    let response = tokio::time::timeout(Duration::from_secs(30), req_fut)
        .await
        .expect("Request timed out")
        .expect("Request failed");

    //* Then
    println!("{:?}", response);
    assert_matches!(response, Err(ResponseError::Failure { errors }) => {
        assert_eq!(errors.len(), 1);

        assert!(errors[0].message.contains(r#"Unknown operation named "invalidOperationName""#));
    });
}

// https://graphql.github.io/graphql-over-http/draft/#sec-application-json.Examples.Variable-coercion-failure
#[tokio::test]
async fn send_invalid_request_variable_coercion_failure() {
    //* Given
    let client = reqwest::Client::new();
    let server_url: Url = TEST_SERVER_URL.parse().unwrap();

    // GraphQL request
    // GraphQL query (string slice)
    let query = indoc::indoc! {
        r#"query filmByFilmId($id: ID!) {
            film(filmID: $id) {
                title
                director
                releaseDate
            }
        }"#
    };

    let request_params = RequestParameters {
        query: query.into_document(),
        operation_name: None,
        variables: serde_json::Map::from_iter([("id".to_string(), serde_json::Value::Null)]),
        extensions: Default::default(),
    };

    // Response types (dummy)
    #[derive(Debug, serde::Deserialize)]
    struct QueryResponse {}

    //* When
    let req_fut = client
        .post(server_url)
        .send_graphql::<QueryResponse>(request_params);
    let response = tokio::time::timeout(Duration::from_secs(30), req_fut)
        .await
        .expect("Request timed out")
        .expect("Request failed");

    //* Then
    assert_matches!(response, Err(ResponseError::Failure { errors }) => {
        assert_eq!(errors.len(), 1);

        assert!(errors[0].message.contains(r#"Variable "$id" of non-null type "ID!" must not be null"#));
    });
}
