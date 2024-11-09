use std::time::Duration;

use super::testlib::{
    init_test_tracing, test_auth_token, test_subgraph_url, GRAPH_NETWORK_ARBITRUM_SUBGRAPH_ID,
};
use crate::queries::bootstrap::send_bootstrap_meta_query;

#[test_with::env(IT_TEST_ARBITRUM_GATEWAY_URL, IT_TEST_ARBITRUM_GATEWAY_AUTH)]
#[tokio::test]
async fn send_subgraph_bootstrap_query_request() {
    init_test_tracing();

    //* Given
    let subgraph_url = test_subgraph_url(GRAPH_NETWORK_ARBITRUM_SUBGRAPH_ID);
    let auth_token = test_auth_token();

    let http_client = reqwest::Client::new();

    //* When
    let res = tokio::time::timeout(
        Duration::from_secs(30),
        send_bootstrap_meta_query(&http_client, subgraph_url, Some(&auth_token)),
    )
    .await
    .expect("Timeout on subgraph meta query");

    //* Then
    // Assert the query succeeded, and we get a non-empty block number and hash.
    let response = res.expect("Failed to fetch subgraph meta");

    assert!(response.meta.block.number > 0);
    assert!(!response.meta.block.hash.is_empty());
}
