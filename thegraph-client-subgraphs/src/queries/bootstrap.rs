//! The bootstrap query is used to determine the latest synced block of a subgraph.
//!
//! Subgraphs sometimes fall behind, be it due to failing or the Graph Node may be having issues.
//! The `_meta` field can now be added to any query so that it is possible to determine against
//! which block the query was effectively executed.
use url::Url;

use super::common::{Meta, send_query};

const SUBGRAPH_META_QUERY_DOCUMENT: &str = r#"{ meta: _meta { block { number hash } } }"#;

#[derive(Debug, serde::Deserialize)]
pub struct SubgraphMetaQueryResponse {
    pub meta: Meta,
}

pub async fn send_bootstrap_meta_query(
    client: &reqwest::Client,
    subgraph_url: Url,
    auth: Option<&str>,
) -> Result<SubgraphMetaQueryResponse, String> {
    send_query(client, subgraph_url, auth, SUBGRAPH_META_QUERY_DOCUMENT)
        .await
        .map_err(|err| format!("Error sending subgraph meta query: {}", err))?
        .map_err(|err| err.to_string())
}
