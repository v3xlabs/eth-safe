mod address_info;
pub mod transactions;

pub use address_info::AddressInfo;

use url::Url;

pub const DEFAULT_BASE_URL: &str = "https://safe-client.safe.global";

/// The hosted gateway sits behind a bot filter that answers 403 to any agent that does not
/// look like a browser; `eth-safe/0.0.1` and an absent header are both rejected.
const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 \
                                  (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

/// Safe Client Gateway, the backend behind the Safe{Wallet} web interface.
///
/// <https://safe-client.safe.global/api>
pub struct SCGClient {
    base_url: Url,
    client: reqwest::Client,
}

impl SCGClient {
    pub fn new(base_url: Url) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent(DEFAULT_USER_AGENT)
            .build()?;

        Ok(Self::with_client(base_url, client))
    }

    pub fn with_client(base_url: Url, client: reqwest::Client) -> Self {
        Self { base_url, client }
    }
}

impl Default for SCGClient {
    fn default() -> Self {
        Self::new(Url::parse(DEFAULT_BASE_URL).expect("the default base url parses"))
            .expect("the default client builds")
    }
}
