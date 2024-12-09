use headers::{Header, HeaderMapExt as _};

/// An extension trait adding "typed" methods to `http::request::Builder` and
/// `http::response::Builder`.
pub trait HttpBuilderExt: sealed::Sealed {
    /// Add a typed `Header` to the builder.
    ///
    /// This method is a convenience wrapper around `headers::HeaderMapExt::typed_insert`.
    ///
    /// # Example for `http::request::Builder`
    ///
    /// ```rust
    /// # use headers::ContentType;
    /// use thegraph_headers::HttpBuilderExt as _;
    ///
    /// let request = http::request::Builder::new()
    ///     .header_typed(ContentType::text())
    ///     .body(())
    ///     .expect("failed to build request");
    ///
    /// assert!(request.headers().get("content-type").is_some());
    /// ```
    ///
    /// # Example for `http::response::Builder`
    ///
    /// ```rust
    /// # use headers::ContentType;
    /// use thegraph_headers::HttpBuilderExt as _;
    ///
    /// let response = http::response::Builder::new()
    ///     .header_typed(ContentType::text())
    ///     .body(())
    ///     .expect("failed to build response");
    ///
    /// assert!(response.headers().get("content-type").is_some());
    /// ```
    fn header_typed<H: Header>(self, header: H) -> Self;
}

impl HttpBuilderExt for http::response::Builder {
    #[inline]
    fn header_typed<H: Header>(mut self, header: H) -> Self {
        // When the builder has error, skip adding the header
        if let Some(headers) = self.headers_mut() {
            headers.typed_insert(header);
        }

        self
    }
}

impl HttpBuilderExt for http::request::Builder {
    #[inline]
    fn header_typed<H: Header>(mut self, header: H) -> Self {
        // When the builder has error, skip adding the header
        if let Some(headers) = self.headers_mut() {
            headers.typed_insert(header);
        }

        self
    }
}

/// Sealed trait to prevent downstream implementations of `HttpBuilderExt`.
mod sealed {
    pub trait Sealed {}
    impl Sealed for http::request::Builder {}
    impl Sealed for http::response::Builder {}
}
