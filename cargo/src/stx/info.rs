use alloy_primitives::{Address, U256};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::prelude::*;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafeInfo {
    pub address: Address,
    #[serde(default)]
    pub nonce: Option<U256>,
    #[serde(default)]
    pub threshold: Option<u64>,
    #[serde(default)]
    pub owners: Vec<Address>,
    #[serde(default)]
    pub master_copy: Option<Address>,
    #[serde(default)]
    pub modules: Vec<Address>,
    #[serde(default)]
    pub fallback_handler: Option<Address>,
    #[serde(default)]
    pub guard: Option<Address>,
    #[serde(default)]
    pub module_guard: Option<Address>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl STXClient {
    pub async fn safe(
        &self,
        network: impl Into<NetworkIdOrSafeSlug>,
        safe: Address,
    ) -> Result<SafeInfo, Error> {
        let url = self.endpoint(&network.into(), &["safes", &safe.to_checksum(None)])?;

        http::get_json(&self.client, url).await
    }
}
