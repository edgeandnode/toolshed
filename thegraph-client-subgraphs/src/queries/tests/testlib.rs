//! Test helpers for the subgraph-client queries integration tests.

use tracing_subscriber::{fmt::TestWriter, EnvFilter};
use url::Url;

/// The Graph Network Arbitrum subgraph in the network.
///
/// https://thegraph.com/explorer/subgraphs/DZz4kDTdmzWLWsV373w2bSmoar3umKKH9y82SUKr5qmp?view=About&chain=arbitrum-one
pub(super) const GRAPH_NETWORK_ARBITRUM_SUBGRAPH_ID: &str =
    "DZz4kDTdmzWLWsV373w2bSmoar3umKKH9y82SUKr5qmp";

/// Initialize the tests tracing subscriber.
pub(super) fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .with_writer(TestWriter::default())
        .try_init();
}

/// Test helper to get the gateway base url from the environment.
pub(super) fn test_gateway_base_url() -> Url {
    std::env::var("IT_TEST_ARBITRUM_GATEWAY_URL")
        .expect("Missing IT_TEST_ARBITRUM_GATEWAY_URL")
        .parse()
        .expect("Invalid IT_TEST_ARBITRUM_GATEWAY_URL")
}

/// Test helper to get the test auth token from the environment.
pub(super) fn test_auth_token() -> String {
    std::env::var("IT_TEST_ARBITRUM_GATEWAY_AUTH").expect("Missing IT_TEST_ARBITRUM_GATEWAY_AUTH")
}

/// Test helper to build the subgraph url with the given subgraph ID.
pub(super) fn test_subgraph_url(subgraph: impl AsRef<str>) -> Url {
    test_gateway_base_url()
        .join(&format!("api/subgraphs/id/{}", subgraph.as_ref()))
        .expect("Invalid URL")
}
