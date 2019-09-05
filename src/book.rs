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

impl Book {
    pub fn empty_from_isbn(isbn: &ISBN) -> Self {
        Book {
            title: String::from(""),
            subtitle: String::from(""),
            authors: vec![],
            publisher: String::from(""),
            edition: String::from(""),
            volume: None,
            year: None,
            month: None,
            isbn: (*isbn).clone(),
        }
    }
}
