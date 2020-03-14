#[tokio::test]
async fn get_all_posts() {
    let client = wordpress::Client::new("http://www.angrybirds.com/".parse().unwrap());
    let ret = client.get_posts_builder().send().await.unwrap();
    assert!(!ret.list.is_empty());
}

#[tokio::test]
async fn search_posts() {
    let client = wordpress::Client::new("http://www.angrybirds.com/".parse().unwrap());
    let ret = client
        .get_posts_builder()
        .search("pig")
        .order_by(wordpress::OrderBy::Relevance)
        .send()
        .await
        .unwrap();
    assert!(!ret.list.is_empty());
}
