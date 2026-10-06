mod info;
mod multisig;

pub use info::SafeInfo;
pub use multisig::{Confirmation, MultisigTransaction, Operation};

use std::collections::HashMap;

use url::Url;

use crate::prelude::*;

pub const DEFAULT_BASE_URL: &str = "https://api.safe.global/tx-service";

const DEFAULT_USER_AGENT: &str = concat!("eth-safe/", env!("CARGO_PKG_VERSION"));

/// Safe Transaction Service, the indexer the gateway itself reads from.
pub struct STXClient {
    base_url: Url,
    client: reqwest::Client,
    slugs: HashMap<NetworkId, NetworkSafeSlug>,
}

impl STXClient {
    pub fn new(base_url: Url) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent(DEFAULT_USER_AGENT)
            .build()?;

        Ok(Self::with_client(base_url, client))
    }

    pub fn with_client(base_url: Url, client: reqwest::Client) -> Self {
        Self {
            base_url,
            client,
            slugs: HashMap::new(),
        }
    }

    /// Teach this client a slug the built-in table does not carry, or override one it does.
    pub fn set_slug(&mut self, network_id: impl Into<NetworkId>, slug: impl Into<NetworkSafeSlug>) {
        self.slugs.insert(network_id.into(), slug.into());
    }

    fn slug(&self, network: &NetworkIdOrSafeSlug) -> Result<NetworkSafeSlug, Error> {
        match network {
            NetworkIdOrSafeSlug::SafeSlug(slug) => Ok(slug.clone()),
            NetworkIdOrSafeSlug::Id(id) => self
                .slugs
                .get(id)
                .cloned()
                .or_else(|| id.safe_slug())
                .ok_or_else(|| Error::UnknownNetwork(network.clone())),
        }
    }

    fn endpoint(&self, network: &NetworkIdOrSafeSlug, segments: &[&str]) -> Result<Url, Error> {
        let slug = self.slug(network)?;
        let mut path = vec![slug.as_str(), "api", "v1"];
        path.extend_from_slice(segments);

        http::endpoint(&self.base_url, &path)
    }
}

impl Default for STXClient {
    fn default() -> Self {
        Self::new(Url::parse(DEFAULT_BASE_URL).expect("the default base url parses"))
            .expect("the default client builds")
    }
}
