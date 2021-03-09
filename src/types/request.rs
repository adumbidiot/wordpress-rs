use crate::{
    client::Client,
    types::List,
    Error,
    Result,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    fmt::Write,
    marker::PhantomData,
    result::Result as StdResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum OrderBy {
    #[serde(rename = "author")]
    Author,

    #[serde(rename = "date")]
    Date,

    #[serde(rename = "id")]
    Id,

    #[serde(rename = "include")]
    Include,

    #[serde(rename = "modified")]
    Modified,

    #[serde(rename = "parent")]
    Parent,

    #[serde(rename = "relevance")]
    Relevance,

    #[serde(rename = "slug")]
    Slug,

    #[serde(rename = "include_slugs")]
    IncludeSlugs,

    #[serde(rename = "title")]
    Title,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Order {
    #[serde(rename = "asc")]
    Ascending,

    #[serde(rename = "desc")]
    Descending,
}

impl Default for Order {
    fn default() -> Self {
        Self::Descending
    }
}

#[derive(Clone)]
pub struct GetTypesBuilder<'a, 'b, T> {
    client: &'a Client,
    data: GetTypesData<'a>,
    type_name: &'b str,

    _return: PhantomData<T>,
}

impl<'a, 'b, T: serde::de::DeserializeOwned> GetTypesBuilder<'a, 'b, T> {
    pub fn new(client: &'a Client, type_name: &'b str) -> Self {
        Self {
            client,
            data: Default::default(),
            type_name,

            _return: PhantomData,
        }
    }

    pub fn context(mut self, context: &'a str) -> Self {
        self.data.context = Some(context);
        self
    }

    pub fn page(mut self, page: u64) -> Self {
        self.data.page = Some(page);
        self
    }

    pub fn per_page(mut self, per_page: u8) -> Self {
        self.data.per_page = Some(per_page);
        self
    }

    pub fn search(mut self, search: &'a str) -> Self {
        self.data.search = Some(search);
        self
    }

    pub fn order_by(mut self, order_by: OrderBy) -> Self {
        self.data.order_by = Some(order_by);
        self
    }

    pub fn category(mut self, category: u64) -> Self {
        self.data.categories = Some(category);
        self
    }

    pub fn order(mut self, order: Order) -> Self {
        self.data.order = Some(order);
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.data.offset = Some(offset);
        self
    }

    /// Add a tag to the filter
    ///
    pub fn tag(mut self, tag: u64) -> Self {
        let tags = self.data.tags.get_or_insert_with(String::new);
        write!(tags, "{}", tag).expect("valid tag append");
        self
    }

    pub fn get_path(&self) -> StdResult<String, serde_urlencoded::ser::Error> {
        Ok(format!(
            "/wp-json/wp/v2/{}?{}",
            self.type_name,
            serde_urlencoded::to_string(&self.data)?
        ))
    }

    pub async fn send(&self) -> Result<List<T>> {
        let (headers, json) = self
            .client
            .get_json(&self.get_path().map_err(Error::QueryEncode)?)
            .await?;
        let total = headers
            .get("x-wp-total")
            .and_then(|s| s.to_str().ok()?.parse().ok())
            .ok_or(Error::MissingTotal)?;
        Ok(List { list: json, total })
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTypesData<'a> {
    pub context: Option<&'a str>,
    pub page: Option<u64>,
    pub per_page: Option<u8>, // Capped at 100
    pub search: Option<&'a str>,
    pub categories: Option<u64>,
    pub order: Option<Order>,
    pub offset: Option<u64>,
    pub tags: Option<String>,

    #[serde(rename = "orderby")]
    pub order_by: Option<OrderBy>,
}
