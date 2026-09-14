mod id;
mod safe_slug;

pub use id::NetworkId;
pub use safe_slug::NetworkSafeSlug;

use std::fmt;

/// The gateway addresses a network by id and the transaction service by slug, so whichever
/// one a caller has, the other is a table lookup away.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NetworkIdOrSafeSlug {
    Id(NetworkId),
    SafeSlug(NetworkSafeSlug),
}

impl NetworkIdOrSafeSlug {
    pub fn network_id(&self) -> Option<NetworkId> {
        match self {
            Self::Id(id) => Some(*id),
            Self::SafeSlug(slug) => slug.network_id(),
        }
    }

    pub fn safe_slug(&self) -> Option<NetworkSafeSlug> {
        match self {
            Self::Id(id) => id.safe_slug(),
            Self::SafeSlug(slug) => Some(slug.clone()),
        }
    }
}

impl From<NetworkId> for NetworkIdOrSafeSlug {
    fn from(id: NetworkId) -> Self {
        Self::Id(id)
    }
}

impl From<u64> for NetworkIdOrSafeSlug {
    fn from(id: u64) -> Self {
        Self::Id(NetworkId(id))
    }
}

impl From<NetworkSafeSlug> for NetworkIdOrSafeSlug {
    fn from(slug: NetworkSafeSlug) -> Self {
        Self::SafeSlug(slug)
    }
}

impl From<&str> for NetworkIdOrSafeSlug {
    fn from(slug: &str) -> Self {
        Self::SafeSlug(slug.into())
    }
}

impl From<String> for NetworkIdOrSafeSlug {
    fn from(slug: String) -> Self {
        Self::SafeSlug(slug.into())
    }
}

impl fmt::Display for NetworkIdOrSafeSlug {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(id) => id.fmt(formatter),
            Self::SafeSlug(slug) => slug.fmt(formatter),
        }
    }
}
