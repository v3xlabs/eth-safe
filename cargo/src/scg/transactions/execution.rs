use serde::Deserialize;
use serde_json::{Map, Value};

use crate::scg::AddressInfo;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum ExecutionInfo {
    #[serde(rename = "MULTISIG", rename_all = "camelCase")]
    Multisig {
        #[serde(default)]
        nonce: Option<u64>,
        #[serde(default)]
        confirmations_required: Option<u64>,
        #[serde(default)]
        confirmations_submitted: Option<u64>,
        #[serde(default)]
        missing_signers: Option<Vec<AddressInfo>>,
        #[serde(flatten)]
        extra: Map<String, Value>,
    },
    #[serde(rename = "MODULE")]
    Module {
        #[serde(default)]
        address: Option<AddressInfo>,
        #[serde(flatten)]
        extra: Map<String, Value>,
    },
    #[serde(untagged)]
    Unknown(serde_json::Value),
}
