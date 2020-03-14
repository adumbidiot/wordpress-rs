use crate::{
    client::Client,
    types::PostList,
    Error,
    Result,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

pub struct GetPostsBuilder<'a> {
    client: &'a Client,
    data: GetPostsData<'a>,
}

impl<'a> GetPostsBuilder<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            data: Default::default(),
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

    pub fn get_path(&self) -> std::result::Result<String, serde_urlencoded::ser::Error> {
        let mut ret = String::from("/wp-json/wp/v2/posts?");
        ret += &serde_urlencoded::to_string(&self.data)?;
        Ok(ret)
    }

    pub async fn send(&self) -> Result<PostList> {
        let (headers, json) = self
            .client
            .get_json(&self.get_path().map_err(Error::QueryEncode)?)
            .await?;
        let total = headers
            .get("x-wp-total")
            .and_then(|s| s.to_str().ok()?.parse().ok())
            .ok_or(Error::MissingTotal)?;
        Ok(PostList { list: json, total })
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GetPostsData<'a> {
    pub context: Option<&'a str>,
    pub page: Option<u64>,
    pub per_page: Option<u8>, // Capped at 100
    pub search: Option<&'a str>,
    pub categories: Option<u64>,
	pub order: Option<Order>,
	pub offset: Option<u64>,

    #[serde(rename = "orderby")]
    pub order_by: Option<OrderBy>,
}
