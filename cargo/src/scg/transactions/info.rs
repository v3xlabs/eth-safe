use alloy_primitives::{Address, U256};
use serde::Deserialize;
use serde_json::{Map, Value};
use strum::{Display, EnumString};

use crate::scg::AddressInfo;

/// The `txInfo` union. Every field other than the discriminator is optional on purpose: an
/// upstream field rename would otherwise collapse a whole known variant into `Unknown` and
/// hand the caller an empty result instead of an error.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionInfo {
    Transfer(TransferInfo),
    /// Also carries multiSend batches, which the gateway reports as `Custom` with
    /// `action_count` populated rather than under a discriminator of their own.
    Custom(CustomInfo),
    SettingsChange(SettingsChangeInfo),
    Creation(CreationInfo),
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferInfo {
    #[serde(default)]
    pub human_description: Option<String>,
    #[serde(default)]
    pub sender: Option<AddressInfo>,
    #[serde(default)]
    pub recipient: Option<AddressInfo>,
    #[serde(default)]
    pub direction: Option<Direction>,
    #[serde(default)]
    pub transfer_info: Option<TransferValue>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum TransferValue {
    #[serde(rename = "NATIVE_COIN", rename_all = "camelCase")]
    NativeCoin {
        #[serde(default)]
        value: Option<U256>,
        #[serde(flatten)]
        extra: Map<String, Value>,
    },
    #[serde(rename = "ERC20", rename_all = "camelCase")]
    Erc20 {
        #[serde(default)]
        token_address: Option<Address>,
        #[serde(default)]
        value: Option<U256>,
        #[serde(default)]
        token_name: Option<String>,
        #[serde(default)]
        token_symbol: Option<String>,
        #[serde(default)]
        logo_uri: Option<String>,
        #[serde(default)]
        decimals: Option<u8>,
        #[serde(default)]
        trusted: Option<bool>,
        #[serde(default)]
        imitation: Option<bool>,
        #[serde(flatten)]
        extra: Map<String, Value>,
    },
    #[serde(rename = "ERC721", rename_all = "camelCase")]
    Erc721 {
        #[serde(default)]
        token_address: Option<Address>,
        #[serde(default)]
        token_id: Option<String>,
        #[serde(default)]
        token_name: Option<String>,
        #[serde(default)]
        token_symbol: Option<String>,
        #[serde(default)]
        logo_uri: Option<String>,
        #[serde(default)]
        trusted: Option<bool>,
        #[serde(flatten)]
        extra: Map<String, Value>,
    },
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomInfo {
    #[serde(default)]
    pub human_description: Option<String>,
    #[serde(default)]
    pub to: Option<AddressInfo>,
    #[serde(default)]
    pub data_size: Option<String>,
    #[serde(default)]
    pub value: Option<U256>,
    #[serde(default)]
    pub is_cancellation: Option<bool>,
    #[serde(default)]
    pub method_name: Option<String>,
    #[serde(default)]
    pub action_count: Option<u64>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsChangeInfo {
    #[serde(default)]
    pub human_description: Option<String>,
    #[serde(default)]
    pub data_decoded: Option<serde_json::Value>,
    #[serde(default)]
    pub settings_info: Option<serde_json::Value>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreationInfo {
    #[serde(default)]
    pub human_description: Option<String>,
    #[serde(default)]
    pub creator: Option<AddressInfo>,
    #[serde(default)]
    pub transaction_hash: Option<String>,
    #[serde(default)]
    pub implementation: Option<AddressInfo>,
    #[serde(default)]
    pub factory: Option<AddressInfo>,
    #[serde(default)]
    pub salt_nonce: Option<String>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Display, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
#[serde(from = "String")]
pub enum Direction {
    Incoming,
    Outgoing,
    #[strum(default)]
    Unknown(String),
}

impl From<String> for Direction {
    fn from(raw: String) -> Self {
        raw.parse().unwrap_or(Self::Unknown(raw))
    }
}
