//! Integration tests for the subgraph-client.

use std::time::Duration;

use thegraph_client_subgraphs::Client as SubgraphClient;
use thegraph_core::{BlockPointer, SubgraphId};
use tracing_subscriber::{EnvFilter, fmt::TestWriter};
use url::Url;

/// Initialize the tests tracing subscriber.
fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .with_writer(TestWriter::default())
        .try_init();
}

/// Test helper to get the gateway base url from the environment.
fn test_gateway_base_url() -> Url {
    std::env::var("IT_TEST_ARBITRUM_GATEWAY_URL")
        .expect("Missing IT_TEST_ARBITRUM_GATEWAY_URL")
        .parse()
        .expect("Invalid IT_TEST_ARBITRUM_GATEWAY_URL")
}

/// Test helper to get the test auth token from the environment.
fn test_auth_token() -> String {
    std::env::var("IT_TEST_ARBITRUM_GATEWAY_AUTH").expect("Missing IT_TEST_ARBITRUM_GATEWAY_AUTH")
}

/// Test helper to build the subgraph url with the given subgraph ID.
fn test_subgraph_url(subgraph: impl AsRef<str>) -> Url {
    test_gateway_base_url()
        .join(&format!("api/subgraphs/id/{}", subgraph.as_ref()))
        .expect("Invalid URL")
}

/// The Graph Network Arbitrum subgraph in the network.
///
/// https://thegraph.com/explorer/subgraphs/DZz4kDTdmzWLWsV373w2bSmoar3umKKH9y82SUKr5qmp?view=About&chain=arbitrum-one
const GRAPH_NETWORK_ARBITRUM_SUBGRAPH_ID: &str = "DZz4kDTdmzWLWsV373w2bSmoar3umKKH9y82SUKr5qmp";

#[test_with::env(IT_TEST_ARBITRUM_GATEWAY_URL, IT_TEST_ARBITRUM_GATEWAY_AUTH)]
#[tokio::test]
async fn client_send_query() {
    init_test_tracing();

    //* Given
    let subgraph_url = test_subgraph_url(GRAPH_NETWORK_ARBITRUM_SUBGRAPH_ID);
    let auth_token = test_auth_token();

    let http_client = reqwest::Client::new();
    let client = SubgraphClient::builder(http_client, subgraph_url)
        .with_auth_token(Some(auth_token))
        .build();

    // Subgraph meta query
    const SUBGRAPH_META_QUERY_DOCUMENT: &str = r#"{ meta: _meta { block { number hash } } }"#;

    #[derive(Debug, serde::Deserialize)]
    struct Meta {
        block: BlockPointer,
    }

    #[derive(Debug, serde::Deserialize)]
    struct SubgraphMetaQueryResponse {
        meta: Meta,
    }

    //* When
    let res = tokio::time::timeout(
        Duration::from_secs(30),
        client.query::<SubgraphMetaQueryResponse>(SUBGRAPH_META_QUERY_DOCUMENT),
    )
    .await
    .expect("Timeout on subgraph meta query");

    //* Then
    // Assert the query succeeded, and we get a non-empty block number and hash.
    let response = res.expect("Failed to fetch subgraph meta");
    assert!(response.meta.block.number > 0);
    assert!(!response.meta.block.hash.is_empty());
}

#[test_with::env(IT_TEST_ARBITRUM_GATEWAY_URL, IT_TEST_ARBITRUM_GATEWAY_AUTH)]
#[tokio::test]
async fn send_subgraph_paginated() {
    init_test_tracing();

    //* Given
    let subgraph_url = test_subgraph_url(GRAPH_NETWORK_ARBITRUM_SUBGRAPH_ID);
    let auth_token = test_auth_token();

    let http_client = reqwest::Client::new();

    let client = SubgraphClient::builder(http_client, subgraph_url)
        .with_auth_token(Some(auth_token))
        .build();

    // Query all subgraph ids.
    const SUBGRAPHS_QUERY_DOCUMENT: &str = r#"
        subgraphs(
            block: $block
            orderBy: id, orderDirection: asc
            first: $first
            where: {
                id_gt: $last
                entityVersion: 2
            }
        ) {
            id
        }
        "#;

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Subgraph {
        #[allow(dead_code)]
        pub id: SubgraphId,
    }

    //* When
    let res = tokio::time::timeout(
        Duration::from_secs(30),
        client.paginated_query::<Subgraph>(SUBGRAPHS_QUERY_DOCUMENT, 200),
    )
    .await
    .expect("Timeout on subgraph paginated query");

    //* Then
    // Assert the query succeeded, and we got a non-empty list of active subscriptions.
    let response = res.expect("Failed to fetch subgraphs");
    assert!(!response.is_empty());
}

#[test_with::env(IT_TEST_ARBITRUM_GATEWAY_URL, IT_TEST_ARBITRUM_GATEWAY_AUTH)]
#[tokio::test]
async fn send_subgraph_paginated_empty_response() {
    init_test_tracing();

    //* Given
    let subgraph_url = test_subgraph_url(GRAPH_NETWORK_ARBITRUM_SUBGRAPH_ID);
    let auth_token = test_auth_token();

    let http_client = reqwest::Client::new();

    let client = SubgraphClient::builder(http_client, subgraph_url)
        .with_auth_token(Some(auth_token))
        .build();

    // Query all subgraph ids. As 'entityVersion' is set to 9999, we expect no results.
    const SUBGRAPHS_QUERY_DOCUMENT: &str = r#"
        subgraphs(
            block: $block
            orderBy: id, orderDirection: asc
            first: $first
            where: {
                id_gt: $last
                entityVersion: 9999
            }
        ) {
            id
        }
        "#;

    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Subgraph {
        #[allow(dead_code)]
        pub id: SubgraphId,
    }

    //* When
    let res = tokio::time::timeout(
        Duration::from_secs(30),
        client.paginated_query::<Subgraph>(SUBGRAPHS_QUERY_DOCUMENT, 200),
    )
    .await
    .expect("Timeout on subgraph paginated query");

    //* Then
    // Assert the query succeeded, and we got a non-empty list of active subscriptions.
    let response = res.expect("Failed to fetch subgraphs");
    assert!(response.is_empty());
}
