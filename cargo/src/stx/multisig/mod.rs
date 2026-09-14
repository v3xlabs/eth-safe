mod confirmation;
mod operation;

pub use confirmation::Confirmation;
pub use operation::Operation;

use alloy_primitives::{Address, Bytes, U256};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Map, Value};

use crate::{Cursor, Error, NetworkIdOrSafeSlug, Page, http, stx::STXClient};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultisigTransaction {
    pub safe_tx_hash: String,
    #[serde(default)]
    pub safe: Option<Address>,
    #[serde(default)]
    pub to: Option<Address>,
    #[serde(default)]
    pub value: Option<U256>,
    #[serde(default)]
    pub data: Option<Bytes>,
    #[serde(default)]
    pub operation: Option<Operation>,
    #[serde(default)]
    pub nonce: Option<u64>,
    #[serde(default)]
    pub submission_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub execution_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub modified: Option<DateTime<Utc>>,
    #[serde(default)]
    pub block_number: Option<u64>,
    #[serde(default)]
    pub transaction_hash: Option<String>,
    #[serde(default)]
    pub proposer: Option<Address>,
    #[serde(default)]
    pub executor: Option<Address>,
    #[serde(default)]
    pub is_executed: Option<bool>,
    #[serde(default)]
    pub is_successful: Option<bool>,
    #[serde(default)]
    pub confirmations_required: Option<u64>,
    #[serde(default)]
    pub confirmations: Vec<Confirmation>,
    #[serde(default)]
    pub data_decoded: Option<Value>,
    #[serde(default)]
    pub trusted: Option<bool>,
    #[serde(default)]
    pub origin: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl STXClient {
    pub async fn multisig_transactions(
        &self,
        network: impl Into<NetworkIdOrSafeSlug>,
        safe: Address,
        cursor: Option<&Cursor>,
    ) -> Result<Page<MultisigTransaction>, Error> {
        let mut url = self.endpoint(
            &network.into(),
            &["safes", &safe.to_checksum(None), "multisig-transactions"],
        )?;

        if let Some(cursor) = cursor {
            url.set_query(Some(cursor.as_str()));
        }

        http::get_json(&self.client, url).await
    }
}
