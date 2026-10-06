pub mod page;

use serde::de::DeserializeOwned;
use url::Url;

use crate::prelude::*;

const BODY_EXCERPT_CHARS: usize = 512;

/// Append path segments to a base url, keeping any path the base already carries.
///
/// A self-hosted deployment may live under a prefix such as `https://internal/safe-cgw/`,
/// which [`Url::join`] would discard when given an absolute path.
pub fn endpoint(base: &Url, segments: &[&str]) -> Result<Url, Error> {
    let mut url = base.clone();
    url.path_segments_mut()
        .map_err(|()| Error::BaseUrlCannotBeABase)?
        .pop_if_empty()
        .extend(segments);
    Ok(url)
}

/// Read a json body, keeping an excerpt of it in [`Error::Decode`] and [`Error::Status`].
pub async fn get_json<T: DeserializeOwned>(client: &reqwest::Client, url: Url) -> Result<T, Error> {
    let response = client
        .get(url.clone())
        .send()
        .await
        .map_err(|source| Error::Request {
            url: url.clone(),
            source,
        })?;

    let status = response.status();
    let body = response.bytes().await.map_err(|source| Error::Request {
        url: url.clone(),
        source,
    })?;

    if !status.is_success() {
        return Err(Error::Status {
            url,
            status,
            body: excerpt(&body),
        });
    }

    serde_json::from_slice(&body).map_err(|source| Error::Decode {
        body: excerpt(&body),
        url,
        source,
    })
}

fn excerpt(body: &[u8]) -> String {
    String::from_utf8_lossy(body)
        .chars()
        .take(BODY_EXCERPT_CHARS)
        .collect()
}
