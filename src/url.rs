pub use url;

use std::{fmt, ops::Deref, str::FromStr};

/// A Url wrapper that is Debug formatted using Display
#[repr(transparent)]
#[derive(Clone)]
pub struct Url(pub url::Url);

impl fmt::Debug for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Url {
    type Err = <url::Url as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        url::Url::from_str(s).map(Self)
    }
}

impl Deref for Url {
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
