mod execution;
mod info;
mod queued;

pub use execution::ExecutionInfo;
pub use info::{
    CreationInfo, CustomInfo, Direction, SettingsChangeInfo, TransactionInfo, TransferInfo,
    TransferValue,
};
pub use queued::{ConflictType, QueuedItem};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Map, Value};
use strum::{Display, EnumString};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    #[serde(default)]
    pub tx_info: Option<TransactionInfo>,
    #[serde(default)]
    pub tx_hash: Option<String>,
    #[serde(default)]
    pub tx_status: Option<TxStatus>,
    #[serde(default, with = "chrono::serde::ts_milliseconds_option")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(default)]
    pub execution_info: Option<ExecutionInfo>,
    #[serde(default)]
    pub safe_app_info: Option<serde_json::Value>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[serde(from = "String")]
pub enum TxStatus {
    Success,
    Failed,
    Cancelled,
    AwaitingConfirmations,
    AwaitingExecution,
    #[strum(default)]
    Unknown(String),
}

impl From<String> for TxStatus {
    fn from(raw: String) -> Self {
        raw.parse().unwrap_or(Self::Unknown(raw))
    }
}
