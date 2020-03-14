use crate::{
    types::request::GetPostsBuilder,
    Error,
    Result,
};
use reqwest::header::HeaderMap;
use url::Url;

pub struct Client {
    client: reqwest::Client,
    base_url: Url,
}

impl Client {
    pub fn new(base_url: Url) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    pub fn get_posts_builder(&self) -> GetPostsBuilder {
        GetPostsBuilder::new(self)
    }

    pub async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        path_fragment: &str,
    ) -> Result<(HeaderMap, T)> {
        let url = self
            .base_url
            .join(&path_fragment)
            .map_err(Error::InvalidPathFragment)?;
        let res = self.client.get(url).send().await?;
        let status = res.status();
        if !status.is_success() {
            return Err(Error::InvalidStatus(status));
        }
        let headers = res.headers().clone();
        let body = res.bytes().await?;
        let json = serde_json::from_slice(&body)?;
        Ok((headers, json))
    }
}
