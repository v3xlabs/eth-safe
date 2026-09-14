use alloy_primitives::Address;
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressInfo {
    pub value: Address,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub logo_uri: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
