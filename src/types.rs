pub mod request;
pub mod response;

/// A generic List Response
///
#[derive(Debug)]
pub struct List<T> {
    /// A list of items. Less than total.
    ///
    pub list: Vec<T>,

    /// The total number of items in this list.
    ///
    pub total: u64,
}
