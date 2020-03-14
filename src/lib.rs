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
            OrderBy,
			Order,
        },
        response::Post,
    },
};
