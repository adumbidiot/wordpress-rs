pub mod request;
pub mod response;

#[derive(Debug)]
pub struct PostList {
    pub list: Vec<crate::Post>,
    pub total: u64,
}
