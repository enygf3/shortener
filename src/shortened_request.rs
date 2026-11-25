use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct ShortenedRequest {
    pub(crate) url: String,
}

impl ShortenedRequest {
    pub(crate) fn new(url: String) -> Self {
        Self { url }
    }
}