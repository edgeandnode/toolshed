//! Integration tests for the `thegraph-graphql-http` crate for the `compat-graphql-client`
//! compatibility feature.
#![cfg(feature = "compat-graphql-client")]

use std::time::Duration;

use assert_matches::assert_matches;
use graphql_client::GraphQLQuery;
use reqwest::Url;

use thegraph_graphql_http::http_client::ReqwestExt;

/// The URL of the test server.
///
/// This a GraphQL server that implements the [Star Wars API](https://swapi.dev/). See
/// https://github.com/graphql/swapi-graphql for more information.
const TEST_SERVER_URL: &str = "https://swapi-graphql.netlify.app/.netlify/functions/index";

// As `graphql_client` generates code that specifies the query, variables and response types, we
// need to cage them under a module to avoid name collisions.
mod test_queries {
    #[derive(graphql_client::GraphQLQuery)]
    #[graphql(
        schema_path = "tests/assets/schema.graphql",
        query_path = "tests/assets/queries.graphql"
    )]
    pub struct AllFilms;

    #[derive(graphql_client::GraphQLQuery)]
    #[graphql(
        schema_path = "tests/assets/schema.graphql",
        query_path = "tests/assets/queries.graphql"
    )]
    pub struct FilmByFilmId;
}

#[tokio::test]
async fn send_valid_graphql_request_no_variables() {
    //* Given
    let client = reqwest::Client::new();
    let server_url: Url = TEST_SERVER_URL.parse().unwrap();

    // Request types
    // > Here we use `graphql_client::GraphQLQuery` derive macro to generate the GraphQL query
    // > types. See the `test_queries` module above for the request types code.
    // >
    // > Note that when calling `GraphQLQuery::build_query` on a query type, the `operationName`
    // > field is set automatically to the name of the query. This is important as the GraphQL
    // > server will use this field to determine which query to execute, since the `query` field
    // > will contain the full query document, i.e., the content of the file pointed by the
    // derive macro's `query_path` argument.

    // Response types
    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct QueryResponse {
        all_films: QueryResponseAllFilms,
    }

    #[derive(Debug, serde::Deserialize)]
    struct QueryResponseAllFilms {
        films: Vec<QueryResponseFilm>,
    }

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct QueryResponseFilm {
        title: String,
        director: String,
        release_date: String,
    }

    //* When
    // Build the GraphQL query
    let query = test_queries::AllFilms::build_query(test_queries::all_films::Variables {});

    let req_fut = client.post(server_url).send_graphql::<QueryResponse>(query);
    let response = tokio::time::timeout(Duration::from_secs(30), req_fut)
        .await
        .expect("Request timed out")
        .expect("Request failed");

    //* Then
    assert_matches!(response, Ok(QueryResponse { all_films: QueryResponseAllFilms { films } }) => {
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

    // Request types
    // > Here we use `graphql_client::GraphQLQuery` derive macro to generate the GraphQL query
    // > types. See the `test_queries` module above for the request types code.
    // >
    // > Note that when calling `GraphQLQuery::build_query` on a query type, the `operationName`
    // > field is set automatically to the name of the query. This is important as the GraphQL
    // > server will use this field to determine which query to execute, since the `query` field
    // > will contain the full query document, i.e., the content of the file pointed by the
    // derive macro's `query_path` argument.

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
    // Build the GraphQL query
    let request =
        test_queries::FilmByFilmId::build_query(test_queries::film_by_film_id::Variables {
            id: "1".to_string(),
        });

    let req_fut = client
        .post(server_url)
        .send_graphql::<QueryResponse>(request);
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
