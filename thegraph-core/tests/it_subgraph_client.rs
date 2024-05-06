//! Integration tests for the subgraph client.
#![cfg(feature = "subgraph-client")]

use assert_matches::assert_matches;
use serde::Deserialize;
use std::time::Duration;
use url::Url;

use thegraph_core::client::queries::meta::{send_subgraph_meta_query, SubgraphMetaQueryResponse};
use thegraph_core::client::queries::page::{send_subgraph_page_query, BlockHeight};
use thegraph_core::client::Client as SubgraphClient;
use thegraph_core::types::{BlockPointer, SubgraphId};

/// Test helper to get the gateway base url from the environment.
fn test_gateway_base_url() -> Url {
    std::env::var("IT_TEST_MAINNET_GATEWAY_URL")
        .expect("Missing IT_TEST_MAINNET_GATEWAY_URL")
        .parse()
        .expect("Invalid IT_TEST_MAINNET_GATEWAY_URL")
}

/// Test helper to get the test auth token from the environment.
fn test_auth_token() -> String {
    std::env::var("IT_TEST_MAINNET_GATEWAY_AUTH").expect("Missing IT_TEST_MAINNET_GATEWAY_AUTH")
}

/// Test helper to build the subgraph url with the given subgraph ID.
fn test_subgraph_url(subgraph: impl AsRef<str>) -> Url {
    test_gateway_base_url()
        .join(&format!("api/subgraphs/id/{}", subgraph.as_ref()))
        .expect("Invalid URL")
}

/// The Graph Network Mainnet subgraph in the network.
///
/// https://thegraph.com/explorer/subgraphs/8yHBZUvXcKkZnZM7SDSgcRMtbtNwgUQfM37cA37h7cet?view=Overview&chain=mainnet
const GRAPH_NETWORK_MAINNET_SUBGRAPH_ID: &str = "8yHBZUvXcKkZnZM7SDSgcRMtbtNwgUQfM37cA37h7cet";

#[test_with::env(IT_TEST_MAINNET_GATEWAY_URL, IT_TEST_MAINNET_GATEWAY_AUTH)]
#[tokio::test]
async fn send_subgraph_meta_query_request() {
    //* Given
    let subgraph_url = test_subgraph_url(GRAPH_NETWORK_MAINNET_SUBGRAPH_ID);
    let auth_token = test_auth_token();

    let http_client = reqwest::Client::new();

    //* When
    let req_fut = send_subgraph_meta_query(&http_client, subgraph_url, Some(&auth_token));
    let res = tokio::time::timeout(Duration::from_secs(10), req_fut)
        .await
        .expect("Timeout on subgraph meta query");

    //* Then
    // Assert the query succeeded, and we get a non-empty block number and hash.
    assert_matches!(res, Ok(SubgraphMetaQueryResponse { meta }) => {
        assert!(meta.block.number > 0);
        assert!(!meta.block.hash.is_empty());
    });
}

#[test_with::env(IT_TEST_MAINNET_GATEWAY_URL, IT_TEST_MAINNET_GATEWAY_AUTH)]
#[tokio::test]
async fn send_subgraph_page_query_request() {
    //* Given
    const PAGE_REQUEST_BATCH_SIZE: usize = 6;

    let subgraph_url = test_subgraph_url(GRAPH_NETWORK_MAINNET_SUBGRAPH_ID);
    let auth_token = test_auth_token();

    let http_client = reqwest::Client::new();

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

    //* When
    let req_fut = send_subgraph_page_query(
        &http_client,
        subgraph_url,
        Some(&auth_token),
        SUBGRAPHS_QUERY_DOCUMENT,
        BlockHeight::NumberGte(18627000),
        PAGE_REQUEST_BATCH_SIZE,
        None,
    );
    let res = tokio::time::timeout(Duration::from_secs(10), req_fut)
        .await
        .expect("Timeout on subgraph meta query");

    //* Then
    assert_matches!(res, Ok(Ok(resp)) => {
        // Assert meta data is present and valid.
        assert!(resp.meta.block.number > 0);
        assert!(!resp.meta.block.hash.is_empty());

        // Assert the results are present and the correct size.
        assert_eq!(resp.results.len(), PAGE_REQUEST_BATCH_SIZE);
    });
}

#[test_with::env(IT_TEST_MAINNET_GATEWAY_URL, IT_TEST_MAINNET_GATEWAY_AUTH)]
#[tokio::test]
async fn client_send_query() {
    //* Given
    let subgraph_url = test_subgraph_url(GRAPH_NETWORK_MAINNET_SUBGRAPH_ID);
    let auth_token = test_auth_token();

    let http_client = reqwest::Client::new();
    let client = SubgraphClient::builder(http_client, subgraph_url)
        .with_auth_token(Some(auth_token))
        .build();

    // Subgraph meta query
    const SUBGRAPH_META_QUERY_DOCUMENT: &str = r#"{ meta: _meta { block { number hash } } }"#;

    #[derive(Debug, Deserialize)]
    struct Meta {
        block: BlockPointer,
    }

    #[derive(Debug, Deserialize)]
    struct SubgraphMetaQueryResponse {
        meta: Meta,
    }

    //* When
    let req_fut = client.query::<SubgraphMetaQueryResponse>(SUBGRAPH_META_QUERY_DOCUMENT);
    let res = tokio::time::timeout(Duration::from_secs(10), req_fut)
        .await
        .expect("Timeout on subgraph meta query");

    //* Then
    // Assert the query succeeded, and we get a non-empty block number and hash.
    assert_matches!(res, Ok(SubgraphMetaQueryResponse { meta }) => {
        assert!(meta.block.number > 0);
        assert!(!meta.block.hash.is_empty());
    });
}

#[test_with::env(IT_TEST_MAINNET_GATEWAY_URL, IT_TEST_MAINNET_GATEWAY_AUTH)]
#[tokio::test]
async fn send_subgraph_paginated() {
    //* Given
    let subgraph_url = test_subgraph_url(GRAPH_NETWORK_MAINNET_SUBGRAPH_ID);
    let auth_token = test_auth_token();

    let http_client = reqwest::Client::new();

    let mut client = SubgraphClient::builder(http_client, subgraph_url)
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

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Subgraph {
        pub id: SubgraphId,
    }

    //* When
    let req_fut = client.paginated_query::<Subgraph>(SUBGRAPHS_QUERY_DOCUMENT, 200);
    let res = tokio::time::timeout(Duration::from_secs(10), req_fut)
        .await
        .expect("Timeout on subgraph paginated query");

    //* Then
    // Assert the query succeeded, and we got a non-empty list of active subscriptions.
    assert_matches!(res, Ok(vec) => {
        assert!(!vec.is_empty());
    });
}
