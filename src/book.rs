use crate::isbn::ISBN;

#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub subtitle: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub edition: String,
    pub volume: Option<u16>,
    pub year: Option<u16>,
    pub month: Option<u8>,
    pub isbn: ISBN,
}
