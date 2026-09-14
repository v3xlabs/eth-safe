use alloy_primitives::Address;
use serde::Deserialize;
use serde_json::{Map, Value};
use strum::{Display, EnumString};

use super::Transaction;
use crate::{Cursor, Error, NetworkIdOrSafeSlug, Page, http, scg::SCGClient};

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum QueuedItem {
    #[serde(rename = "TRANSACTION", rename_all = "camelCase")]
    Transaction {
        transaction: Box<Transaction>,
        #[serde(default)]
        conflict_type: Option<ConflictType>,
        #[serde(flatten)]
        extra: Map<String, Value>,
    },
    /// Groups the transactions that follow it. `Next` marks the ones executable at the safe's
    /// current nonce, `Queued` marks later nonces, so dropping these loses that distinction.
    #[serde(rename = "LABEL")]
    Label {
        label: String,
        #[serde(flatten)]
        extra: Map<String, Value>,
    },
    #[serde(rename = "CONFLICT_HEADER")]
    ConflictHeader {
        nonce: u64,
        #[serde(flatten)]
        extra: Map<String, Value>,
    },
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Display, EnumString)]
#[serde(from = "String")]
pub enum ConflictType {
    None,
    HasNext,
    End,
    #[strum(default)]
    Unknown(String),
}

impl From<String> for ConflictType {
    fn from(raw: String) -> Self {
        raw.parse().unwrap_or(Self::Unknown(raw))
    }
}

impl SCGClient {
    pub async fn queued_transactions(
        &self,
        network: impl Into<NetworkIdOrSafeSlug>,
        safe: Address,
        cursor: Option<&Cursor>,
    ) -> Result<Page<QueuedItem>, Error> {
        let network = network.into();
        let Some(network_id) = network.network_id() else {
            return Err(Error::UnknownNetwork(network));
        };

        let mut url = http::endpoint(
            &self.base_url,
            &[
                "v1",
                "chains",
                &network_id.to_string(),
                "safes",
                &safe.to_checksum(None),
                "transactions",
                "queued",
            ],
        )?;

        if let Some(cursor) = cursor {
            url.set_query(Some(cursor.as_str()));
        }

        http::get_json(&self.client, url).await
    }
}
