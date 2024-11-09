use serde::ser::SerializeMap as _;
use serde_json::value::RawValue;
use thegraph_core::alloy::primitives::{BlockHash, BlockNumber};
use thegraph_graphql_http::{
    graphql::{Document, IntoDocument, IntoDocumentWithVariables},
    http_client::ResponseResult,
};
use url::Url;

use super::common::{send_query, Meta};

/// The block at which the query should be executed.
///
/// This is part of the input arguments of the [`SubgraphPageQuery`].
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub enum BlockHeight {
    #[default]
    Latest,
    Hash(BlockHash),
    Number(BlockNumber),
    NumberGte(BlockNumber),
}

impl serde::Serialize for BlockHeight {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut obj = s.serialize_map(Some(1))?;
        match self {
            Self::Latest => (),
            Self::Hash(hash) => obj.serialize_entry("hash", hash)?,
            Self::Number(number) => obj.serialize_entry("number", number)?,
            Self::NumberGte(number) => obj.serialize_entry("number_gte", number)?,
        }
        obj.end()
    }
}

/// The arguments of the [`SubgraphPageQuery`] query.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SubgraphPageQueryVars {
    /// The block at which the query should be executed.
    block: BlockHeight,
    /// The maximum number of entities to fetch.
    first: usize,
    /// The ID of the last entity fetched.
    last: String,
}

pub struct SubgraphPageQuery {
    query: Document,
    vars: SubgraphPageQueryVars,
}

impl SubgraphPageQuery {
    pub fn new(query: impl IntoDocument, block: BlockHeight, first: usize, last: String) -> Self {
        Self {
            query: query.into_document(),
            vars: SubgraphPageQueryVars { block, first, last },
        }
    }
}

impl IntoDocumentWithVariables for SubgraphPageQuery {
    type Variables = SubgraphPageQueryVars;

    fn into_document_with_variables(self) -> (Document, Self::Variables) {
        let query = indoc::formatdoc!(
            r#"query ($block: Block_height!, $first: Int!, $last: String!) {{
                    meta: _meta(block: $block) {{ block {{ number hash }} }}
                    results: {query}
                }}"#,
            query = self.query
        );

        (query.into_document(), self.vars)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct SubgraphPageQueryResponse {
    pub meta: Meta,
    pub results: Vec<Box<RawValue>>,
}

/// An opaque entry in the response of a subgraph page query.
///
/// This is used to determine the ID of the last entity fetched.
#[derive(Debug, serde::Deserialize)]
pub struct SubgraphPageQueryResponseOpaqueEntry {
    pub id: String,
}

pub async fn send_subgraph_page_query(
    client: &reqwest::Client,
    subgraph_url: Url,
    auth: Option<&str>,
    query: impl IntoDocument,
    block_height: BlockHeight,
    batch_size: usize,
    last: Option<String>,
) -> Result<ResponseResult<SubgraphPageQueryResponse>, String> {
    send_query(
        client,
        subgraph_url,
        auth,
        SubgraphPageQuery::new(query, block_height, batch_size, last.unwrap_or_default()),
    )
    .await
    .map_err(|err| format!("Error sending subgraph graphql query: {}", err))
}
