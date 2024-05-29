use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use alloy_primitives::aliases::BlockNumber;
use serde::de::Deserialize;
use thegraph_graphql_http::graphql::IntoDocument;
use thegraph_graphql_http::http::request::IntoRequestParameters;
use thegraph_graphql_http::http_client::ResponseError;
use tracing::Instrument;
use url::Url;

use crate::types::BlockPointer;

use super::queries::{
    meta::send_bootstrap_meta_query,
    page::{send_subgraph_page_query, BlockHeight, SubgraphPageQueryResponseOpaqueEntry},
    send_subgraph_query,
};

/// Error message returned by the indexer typically when a reorg happens.
const SUBGRAPH_REORG_ERROR: &str = "no block with that hash found";

#[derive(Debug, Clone, thiserror::Error)]
pub enum PaginatedQueryError {
    /// The bootstrap meta query failed.
    #[error("bootstrap meta query failed: {0}")]
    BootstrapMetaQueryFailed(String),

    /// The response was empty.
    ///
    /// A page query response should always contain at least the meta query response. If the
    /// response is empty, it means that the subgraph is not returning any data.
    #[error("empty response")]
    EmptyResponse,

    /// A reorg was detected.
    ///
    /// The indexer responded with an error message indicating that a reorg was detected.
    #[error("reorg detected")]
    ReorgDetected,

    /// An error occurred while sending one of the requests.
    #[error("request error: {0}")]
    RequestError(String),

    /// An error occurred while processing the query.
    ///
    /// This error contains the error messages returned by the indexer when an error occurred while
    /// processing one of the page requests.
    #[error("response error: {0:?}")]
    ResponseError(Vec<String>),

    /// Response deserialization error.
    ///
    /// An error occurred while deserializing the response.
    #[error("deserialization error: {0}")]
    DeserializationError(String),
}

async fn send_paginated_query<T: for<'de> Deserialize<'de>>(
    client: &reqwest::Client,
    subgraph_url: &Url,
    query: impl IntoDocument + Clone,
    ticket: Option<&str>,
    page_size: usize,
    mut block_height: BlockHeight,
) -> Result<(Vec<T>, Option<BlockPointer>), PaginatedQueryError> {
    debug_assert_ne!(page_size, 0, "page size must be greater than 0");

    // Block at which the query is executed.
    let mut block_pointer: Option<BlockPointer> = None;

    // The last id of the previous batch.
    let mut last_id: Option<String> = None;

    // Vector to store the results of the paginated query.
    let mut results = Vec::new();

    loop {
        tracing::trace!(
            last_id = %last_id.as_deref().unwrap_or("none"),
            "sending page query request"
        );

        let response = send_subgraph_page_query(
            client,
            subgraph_url.clone(),
            ticket,
            query.clone(),
            block_height,
            page_size,
            last_id,
        )
        .await
        .map_err(PaginatedQueryError::RequestError)?;

        let resp = match response {
            Ok(data) if !data.results.is_empty() => data,
            Ok(_) if results.is_empty() => return Err(PaginatedQueryError::EmptyResponse),
            Ok(_) => break,
            Err(err) => {
                return match err {
                    ResponseError::Empty => Err(PaginatedQueryError::EmptyResponse),
                    ResponseError::Failure { errors } => {
                        // Check if the error message contains the reorg error message.
                        if errors
                            .iter()
                            .any(|err| err.message.contains(SUBGRAPH_REORG_ERROR))
                        {
                            tracing::debug!(errors=?errors, "reorg detected");
                            return Err(PaginatedQueryError::ReorgDetected);
                        }

                        let errors = errors
                            .into_iter()
                            .map(|err| err.message)
                            .collect::<Vec<String>>();
                        Err(PaginatedQueryError::ResponseError(errors))
                    }
                };
            }
        };

        last_id = {
            let raw_data = resp.results.last().unwrap().get();
            match serde_json::from_str::<SubgraphPageQueryResponseOpaqueEntry>(raw_data).ok() {
                Some(item) => Some(item.id),
                None => {
                    return Err(PaginatedQueryError::DeserializationError(
                        "failed to extract id for last entry".to_string(),
                    ))
                }
            }
        };

        tracing::trace!(
            block_number = %resp.meta.block.number,
            block_hash = %resp.meta.block.hash,
            page_items_count = %resp.results.len(),
            last_item_id = %last_id.as_deref().unwrap_or_default(),
            "received page query response"
        );

        block_height = BlockHeight::Hash(resp.meta.block.hash);
        block_pointer = Some(resp.meta.block);

        // Deserialize the response data and push them to the results vector
        for entity in resp.results {
            match serde_json::from_str::<T>(entity.get()) {
                Ok(data) => results.push(data),
                Err(err) => {
                    return Err(PaginatedQueryError::DeserializationError(err.to_string()));
                }
            }
        }
    }

    Ok((results, block_pointer))
}

/// A client for interacting with a subgraph.
#[derive(Clone)]
pub struct Client {
    pub http_client: reqwest::Client,
    pub subgraph_url: Url,

    /// The request authentication bearer token.
    ///
    /// This is token is inserted in the `Authentication` header.
    pub auth_token: Option<String>,

    /// The latest block number that the subgraph has progressed to.
    ///
    /// By default, this value is 0, and is updated after each paginated query.
    latest_block: Arc<AtomicU64>,
}

impl Client {
    /// Create a new client with default settings.
    ///
    /// The default settings are:
    /// - No authentication token
    /// - Latest block number of 0
    pub fn new(http_client: reqwest::Client, subgraph_url: Url) -> Self {
        Self {
            http_client,
            subgraph_url,
            auth_token: None,
            latest_block: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Create a new client builder.
    ///
    /// The builder allows for configuring the client before building it.
    ///
    /// Example:
    /// ```text
    /// let client = SubgraphClient::builder(http_client, subgraph_url)
    ///     .with_auth_token(Some(ticket))
    ///     .with_subgraph_latest_block(18627000)
    ///     .build();
    /// ```
    pub fn builder(http_client: reqwest::Client, subgraph_url: Url) -> ClientBuilder {
        ClientBuilder::new(http_client, subgraph_url)
    }

    /// Get the latest block number.
    fn latest_block(&self) -> BlockNumber {
        self.latest_block.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Update the client's latest block number.
    ///
    /// The function ensures that the latest block number is always increasing
    ///
    /// Returns the latest block number.
    fn update_latest_block(&self, new_value: BlockNumber) -> BlockNumber {
        // Ensure that the latest block number is always increasing
        self.latest_block
            .fetch_max(new_value, std::sync::atomic::Ordering::Relaxed)
            .max(new_value)
    }

    /// Send a query to the subgraph.
    pub async fn query<T: for<'de> Deserialize<'de>>(
        &self,
        query: impl IntoRequestParameters + Send,
    ) -> Result<T, String> {
        send_subgraph_query::<T>(
            &self.http_client,
            self.subgraph_url.clone(),
            self.auth_token.as_deref(),
            query,
        )
        .await
    }

    /// Send a paginated query to the subgraph.
    ///
    /// The query is sent with a page size of `page_size` and the latest block number that the
    /// subgraph has progressed to.
    ///
    /// In the case of a reorg, the function will return an error.
    pub async fn paginated_query<T: for<'de> Deserialize<'de>>(
        &self,
        query: impl IntoDocument + Clone,
        page_size: usize,
    ) -> Result<Vec<T>, PaginatedQueryError> {
        let _span = tracing::debug_span!(
            "paginated_query",
            url = %self.subgraph_url,
            page_size,
        )
        .entered();

        // Send a bootstrap meta query if the latest block number is 0.
        //
        // Graph-node is rejecting values of `number_gte:0` on subgraphs with a larger `startBlock`.
        // This forces us to request the latest block number from the subgraph before sending the
        // paginated query.
        let mut latest_block = self.latest_block();
        if latest_block == 0 {
            tracing::debug!("sending bootstrap meta query");
            let bootstrap_block = send_bootstrap_meta_query(
                &self.http_client,
                self.subgraph_url.clone(),
                self.auth_token.as_deref(),
            )
            .in_current_span()
            .await
            .map_err(PaginatedQueryError::BootstrapMetaQueryFailed)?;

            tracing::debug!(
                block_number = bootstrap_block.meta.block.number,
                block_hash = %bootstrap_block.meta.block.hash,
                "received bootstrap meta query response"
            );

            // Update the latest block number
            latest_block = self.update_latest_block(bootstrap_block.meta.block.number);
        }

        // Send the paginated query request
        tracing::debug!(block_number = %latest_block ,"sending request");

        let (results, block) = send_paginated_query(
            &self.http_client,
            &self.subgraph_url,
            query,
            self.auth_token.as_deref(),
            page_size,
            BlockHeight::NumberGte(latest_block),
        )
        .in_current_span()
        .await?;

        // Update the latest block number
        if let Some(block) = block {
            self.update_latest_block(block.number);
        }

        tracing::debug!(total_items_count = %results.len(), "received response");

        Ok(results)
    }
}
/// A builder for constructing a subgraph client.
pub struct ClientBuilder {
    http_client: reqwest::Client,
    subgraph_url: Url,
    auth_token: Option<String>,
    latest_block: BlockNumber,
}

impl ClientBuilder {
    fn new(http_client: reqwest::Client, subgraph_url: Url) -> Self {
        Self {
            http_client,
            subgraph_url,
            auth_token: None,
            latest_block: 0,
        }
    }

    /// Set request authentication token.
    ///
    /// By default all requests are issued non-authenticated.
    pub fn with_auth_token(mut self, token: Option<String>) -> Self {
        self.auth_token = token;
        self
    }

    /// Set the latest block number that the subgraph has progressed to.
    ///
    /// The default value is 0.
    pub fn with_subgraph_latest_block(mut self, latest_block: BlockNumber) -> Self {
        self.latest_block = latest_block;
        self
    }

    pub fn build(self) -> Client {
        Client {
            http_client: self.http_client,
            subgraph_url: self.subgraph_url,
            auth_token: self.auth_token,
            latest_block: Arc::new(AtomicU64::new(self.latest_block)),
        }
    }
}
