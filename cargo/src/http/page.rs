use serde::{Deserialize, Deserializer, de};
use url::Url;

#[derive(Debug, Clone, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct Page<T> {
    #[serde(default)]
    pub count: Option<u64>,
    #[serde(default, deserialize_with = "cursor_from_url")]
    pub next: Option<Cursor>,
    #[serde(default, deserialize_with = "cursor_from_url")]
    pub previous: Option<Cursor>,
    #[serde(default)]
    pub results: Vec<T>,
}

/// The query string of a `next` or `previous` link, kept without its scheme, host or path.
///
/// Safe returns those links as absolute `http://` urls pointing at its own deployment, so
/// replaying one verbatim would downgrade the transport and ignore a self-hosted base url.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor(String);

impl Cursor {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn cursor_from_url<'de, D>(deserializer: D) -> Result<Option<Cursor>, D::Error>
where
    D: Deserializer<'de>,
{
    let Some(link) = Option::<String>::deserialize(deserializer)? else {
        return Ok(None);
    };

    let url = Url::parse(&link).map_err(de::Error::custom)?;

    match url.query() {
        Some(query) => Ok(Some(Cursor(query.to_owned()))),
        None => Err(de::Error::custom(format!(
            "pagination link carries no query: {link}"
        ))),
    }
}
