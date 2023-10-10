//! GraphQL query type and related traits.
//!
//! This module contains the [`Document`] type and the [`IntoDocument`] conversion trait. The conversion
//! trait is implemented for string types: [`String`] and `&str`.
//!

/// A (raw) GraphQL request document.
///
/// This type is a wrapper around a string that represents a GraphQL request document. This type
/// does not perform any validation on the string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document(String);

impl Document {
    /// Create a new GraphQL [`Document`] instance from a `String`.
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Return a string slice to the document.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Document {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl serde::ser::Serialize for Document {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

/// A trait for types that can be converted into a [`Document`].
pub trait IntoDocument {
    /// Consumes `self` and returns a [`Document`].
    fn into_document(self) -> Document;
}

/// A trait for types that can be converted into a [`Document`] and variables tuple.
pub trait IntoDocumentWithVariables {
    type Variables: serde::Serialize;

    /// Consumes `self` and returns a query and variables tuple.
    fn into_document_with_variables(self) -> (Document, Self::Variables);
}

impl IntoDocument for Document {
    fn into_document(self) -> Document {
        self
    }
}

impl IntoDocument for String {
    fn into_document(self) -> Document {
        Document(self)
    }
}

impl IntoDocument for &str {
    fn into_document(self) -> Document {
        Document(self.to_owned())
    }
}

impl<T> IntoDocumentWithVariables for T
where
    T: IntoDocument,
{
    type Variables = ();

    fn into_document_with_variables(self) -> (Document, Self::Variables) {
        (self.into_document(), ())
    }
}
