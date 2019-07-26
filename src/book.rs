extern crate reqwest;

#[derive(Debug, Clone)]
pub struct Book<'a> {
    title: &'a str,
    author: Vec<String>,
    publisher: &'a str,
    edition: &'a str,
    volume: Option<u16>,
    year: Option<u16>,
    month: Option<u8>,
    isbn: &'a str,
}
