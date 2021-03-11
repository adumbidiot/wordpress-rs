use crate::{
    types::request::GetTypesBuilder,
    Error,
    Post,
    Result,
};
use reqwest::header::HeaderMap;
use tokio::io::{
    AsyncWrite,
    AsyncWriteExt,
};
use url::Url;

/// The Wordpress API client
///
#[derive(Debug, Clone)]
pub struct Client {
    /// The inner HTTP Client
    ///
    /// You probably shouldn't touch this unless you want to piggyback off of it to send requests.
    ///
    pub client: reqwest::Client,
    base_url: Url,
}

impl Client {
    /// Make a new [`Client`] from the given `base_url`.
    ///
    pub fn new(base_url: Url) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }

    /// Get a type by id
    ///
    pub async fn get_type_by_id<T>(&self, type_name: &str, id: u64) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        Ok(self
            .get_json(&format!("/wp-json/wp/v2/{}/{}", type_name, id))
            .await?
            .1)
    }

    /// Get a builder for a request that fetches objects of the given type.
    ///
    pub fn get_types_builder<'a, 'b, T>(&'a self, type_name: &'b str) -> GetTypesBuilder<'a, 'b, T>
    where
        T: serde::de::DeserializeOwned,
    {
        GetTypesBuilder::new(self, type_name)
    }

    /// Get a builder for a request that fetches posts.
    ///
    pub fn get_posts_builder(&self) -> GetTypesBuilder<Post> {
        self.get_types_builder("posts")
    }

    /// Get the json at the `path_fragment`.
    ///
    pub async fn get_json<T>(&self, path_fragment: &str) -> Result<(HeaderMap, T)>
    where
        T: serde::de::DeserializeOwned,
    {
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

    /// Send a GET request to a url and copy the response to the writer.
    ///
    pub async fn get_to<W>(&self, url: &str, mut writer: W) -> Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        let mut res = self.client.get(url).send().await?;
        let status = res.status();
        if !status.is_success() {
            return Err(Error::InvalidStatus(status));
        }

        while let Some(chunk) = res.chunk().await? {
            writer.write_all(&chunk).await?;
        }

        Ok(())
    }
}
