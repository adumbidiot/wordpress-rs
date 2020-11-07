mod client;
mod error;
mod types;

pub use crate::{
    client::Client,
    error::{
        Error,
        Result,
    },
    types::{
        request::{
            GetPostsData,
            Order,
            OrderBy,
        },
        response::Post,
    },
};

#[cfg(test)]
mod test {
    use super::*;

    const BASE: &str = "https://www.codeinwp.com/";

    #[tokio::test]
    async fn get_all_posts() {
        let client = Client::new(BASE.parse().unwrap());
        let ret = client.get_posts_builder().send().await.unwrap();
        assert!(!ret.list.is_empty());
    }

    #[tokio::test]
    async fn search_posts() {
        let client = Client::new(BASE.parse().unwrap());
        let ret = client
            .get_posts_builder()
            .search("hope")
            .order_by(OrderBy::Relevance)
            .send()
            .await
            .unwrap();
        assert!(!ret.list.is_empty());
    }
}
