use std::fmt;

/// An EIP-155 chain identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NetworkId(pub u64);

impl From<u64> for NetworkId {
    fn from(raw: u64) -> Self {
        Self(raw)
    }
}

impl From<NetworkId> for u64 {
    fn from(id: NetworkId) -> Self {
        id.0
    }
}

impl fmt::Display for NetworkId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
