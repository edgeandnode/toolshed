//! An HTTP _typed header_ for the `graph-indexed` header.
//!
//! The `graph-indexed` header contains a JSON-encoded [`BlockInfo`] struct indicating the latest
//! indexed block information.
//!
//! # Using the `headers::HeaderMapExt` extension trait
//!
//! ```rust
//! use headers::HeaderMapExt as _;
//! use thegraph_headers::graph_indexed::{GraphIndexed, HEADER_NAME};
//!
//! let mut header_map = http::HeaderMap::new();
//! # let value = thegraph_headers::graph_indexed::BlockInfo {
//! #     hash: thegraph_core::alloy::primitives::BlockHash::new([0x55; 32]),
//! #     number: 42,
//! #     timestamp: None,
//! # };
//!
//! // Insert a `graph-indexed` HTTP header
//! header_map.typed_insert(GraphIndexed(value));
//!
//! // Get the `graph-indexed` HTTP header by name
//! let header_by_name = header_map.get(HEADER_NAME);
//! assert!(header_by_name.is_some());
//!
//! // Get the `graph-indexed` HTTP header by type
//! let header_typed = header_map.typed_get::<GraphIndexed>();
//! assert!(matches!(header_typed, Some(GraphIndexed(..))));
//! ```

use headers::{Error as HeaderError, Header, HeaderName, HeaderValue};
use thegraph_core::alloy::primitives::{BlockHash, BlockNumber};

/// The HTTP header name for the `graph-indexed` header.
pub const HEADER_NAME: &str = "graph-indexed";

/// An HTTP _typed header_ for the `graph-indexed` header.
///
/// The `graph-indexed` header contains a JSON-encoded [`BlockInfo`] struct indicating the latest
/// indexed block information.
pub struct GraphIndexed(pub BlockInfo);

impl Header for GraphIndexed {
    fn name() -> &'static HeaderName {
        static HTTP_HEADER_NAME: HeaderName = HeaderName::from_static(HEADER_NAME);
        &HTTP_HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, HeaderError>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(HeaderError::invalid)?;
        let info = serde_json::from_slice::<'_, BlockInfo>(value.as_bytes())
            .map_err(|_| HeaderError::invalid())?;
        Ok(Self(info))
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let bytes = serde_json::to_vec(&self.0).expect("header to be valid json");
        let value = HeaderValue::from_bytes(&bytes).expect("header to be valid utf-8");
        values.extend(std::iter::once(value));
    }
}

/// A struct containing information about the latest block.
///
/// Type ported from the Graph Node.
///
/// See Graph Node's [`LatestBlockInfo`][1].
///
/// [1]: https://github.com/graphprotocol/graph-node/blob/a8b590f7d3fbabf2968ce7ced30bfd1485ce5f31/graph/src/data/query/result.rs#L68-L74
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlockInfo {
    /// The hash of the latest block.
    pub hash: BlockHash,
    /// The number of the latest block.
    pub number: BlockNumber,
    /// The timestamp of the latest block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
}
