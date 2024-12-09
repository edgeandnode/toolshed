//! Integration tests for the `thegraph-graphql-http` crate for the `graphql-parser`
//! compatibility feature.
#![cfg(all(feature = "graphql-parser", feature = "reqwest"))]

use std::time::Duration;

use assert_matches::assert_matches;
use reqwest::Url;
use thegraph_graphql_http::{
    graphql::IntoDocument,
    http::request::{IntoRequestParameters, RequestParameters},
    http_client::ReqwestExt,
};

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

    // GraphQL query (graphql_parser::query::Document)
    let query = graphql_parser::parse_query::<&str>(indoc::indoc! {
        r#"{
            allFilms {
                films {
                    title
                    director
                    releaseDate
                }
            }
        }"#
    })
    .expect("Invalid GraphQL query");

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
            let query = graphql_parser::parse_query::<&str>(indoc::indoc! {
                r#"query filmByFilmId($id: ID!) {
                    film(filmID: $id) {
                        title
                        director
                        releaseDate
                    }
                }"#
            })
            .expect("Invalid GraphQL query");

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
