use reqwest::StatusCode;
use url::Url;

use crate::NetworkIdOrSafeSlug;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("the configured base url cannot be a base")]
    BaseUrlCannotBeABase,

    #[error("network {0} is not in the built-in table")]
    UnknownNetwork(NetworkIdOrSafeSlug),

    #[error("request to {url} failed")]
    Request {
        url: Url,
        #[source]
        source: reqwest::Error,
    },

    #[error("{url} returned HTTP {status}: {body}")]
    Status {
        url: Url,
        status: StatusCode,
        body: String,
    },

    #[error("could not decode the response from {url}: {source} (body: {body})")]
    Decode {
        url: Url,
        body: String,
        #[source]
        source: serde_json::Error,
    },
}
