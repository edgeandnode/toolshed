use std::time::Duration;

use super::testlib::{
    init_test_tracing, test_auth_token, test_subgraph_url, GRAPH_NETWORK_ARBITRUM_SUBGRAPH_ID,
};
use crate::queries::page::{send_subgraph_page_query, BlockHeight};

#[test_with::env(IT_TEST_ARBITRUM_GATEWAY_URL, IT_TEST_ARBITRUM_GATEWAY_AUTH)]
#[tokio::test]
async fn send_subgraph_page_query_request() {
    init_test_tracing();

    //* Given
    const PAGE_REQUEST_BATCH_SIZE: usize = 6;

    let subgraph_url = test_subgraph_url(GRAPH_NETWORK_ARBITRUM_SUBGRAPH_ID);
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
    let res = tokio::time::timeout(
        Duration::from_secs(30),
        send_subgraph_page_query(
            &http_client,
            subgraph_url,
            Some(&auth_token),
            SUBGRAPHS_QUERY_DOCUMENT,
            BlockHeight::NumberGte(18627000),
            PAGE_REQUEST_BATCH_SIZE,
            None,
        ),
    )
    .await
    .expect("Timeout on subgraph meta query");

    //* Then
    let page_res = res.expect("Failed to fetch subgraph page");
    let page_response = page_res.expect("Failed to fetch subgraph page");

    // Assert meta data is present and valid.
    assert!(page_response.meta.block.number > 0);
    assert!(!page_response.meta.block.hash.is_empty());

    // Assert the results are present and the correct size.
    assert_eq!(page_response.results.len(), PAGE_REQUEST_BATCH_SIZE);
}
