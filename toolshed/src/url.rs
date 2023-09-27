// Re-export the `url` crate
pub use url;

/// A [`url::Url`] wrapper that is [`std::fmt::Debug`] formatted using [`std::fmt::Display`].
#[repr(transparent)]
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Url(pub url::Url);

impl std::fmt::Debug for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Url {
    type Err = <url::Url as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        url::Url::from_str(s).map(Self)
    }
}

impl std::ops::Deref for Url {
    type Target = url::Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<url::Url> for Url {
    fn from(from: url::Url) -> Self {
        Url(from)
    }
}
