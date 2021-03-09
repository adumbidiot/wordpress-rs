use chrono::NaiveDateTime;
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;
use url::Url;

/// A wordpress post
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub date: NaiveDateTime,
    pub date_gmt: NaiveDateTime,
    guid: serde_json::Value,
    pub id: u64,
    pub link: Url,
    pub modified: NaiveDateTime,
    pub modified_gmt: NaiveDateTime,
    pub slug: String,
    pub status: String,
    #[serde(rename = "type")]
    kind: serde_json::Value,
    pub password: Option<String>,
    pub permalink_template: Option<String>,
    pub generated_slug: Option<String>,
    pub title: PostTitle,
    pub content: PostContent,
    pub author: u64,
    excerpt: serde_json::Value,
    pub featured_media: u64,
    pub comment_status: String,
    pub ping_status: String,
    pub format: Option<String>,
    meta: serde_json::Value,
    pub sticky: Option<bool>,
    pub template: String,
    pub categories: Option<Vec<u64>>,
    tags: Option<Vec<serde_json::Value>>,

    /// Unknown KVs
    ///
    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTitle {
    pub rendered: String,

    /// Unknown KVs
    ///
    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostContent {
    pub rendered: String,
    pub protected: bool,

    /// Unknown KVs
    ///
    #[serde(flatten)]
    pub unknown: HashMap<String, serde_json::Value>,
}
