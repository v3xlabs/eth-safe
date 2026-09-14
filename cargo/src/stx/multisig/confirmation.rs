use alloy_primitives::{Address, Bytes};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Confirmation {
    #[serde(default)]
    pub owner: Option<Address>,
    #[serde(default)]
    pub submission_date: Option<DateTime<Utc>>,
    #[serde(default)]
    pub transaction_hash: Option<String>,
    #[serde(default)]
    pub signature: Option<Bytes>,
    #[serde(default)]
    pub signature_type: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
