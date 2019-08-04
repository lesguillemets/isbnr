extern crate reqwest;
extern crate serde_json;
// use serde_json::{Result, Value};

use crate::isbn::ISBN;

#[derive(Debug, Clone)]
pub struct Book {
    title: String,
    subtitle: String,
    authors: Vec<String>,
    publisher: String,
    edition: String,
    volume: Option<u16>,
    year: Option<u16>,
    month: Option<u8>,
    isbn: ISBN,
}

fn field_as_String_ok(v: &serde_json::Value, field: &str) -> Option<String> {
    v[field].as_str().map(String::from)
}

pub fn lookup_google(isbn: &ISBN) -> Option<Book> {
    let url = format!(
        "https://www.googleapis.com/books/v1/volumes?q=isbn:{}",
        isbn.as_str()
    );
    let mut response = reqwest::get(&url).unwrap();
    let result: serde_json::Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    if result["totalItems"].as_u64() == Some(1) {
        let thisbook = &result["items"][0];
        let volume_info = &thisbook["volumeInfo"];
        let title = field_as_String_ok(&volume_info, "title")?;
        let subtitle = field_as_String_ok(&volume_info, "subtitle")?;
        let publisher = field_as_String_ok(&volume_info, "publisher")?;
        let published = field_as_String_ok(&volume_info, "publishedDate")?;
        let (year, month) = parse_hyphen_date(&published);
        let authors: Vec<String> = volume_info["authors"]
            // errors can be ignored here
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|e| (e.as_str().map(String::from)))
            .collect();
        let book = Book {
            title,
            subtitle,
            authors,
            publisher,
            edition: String::from(""),
            volume: None,
            year,
            month,
            isbn: isbn.clone(),
        };
        Some(book)
    } else {
        None
    }
}

fn parse_hyphen_date(date: &str) -> (Option<u16>, Option<u8>) {
    // assumes yyyy-mm
    let parsed: Vec<Option<u16>> = date.split('-').map(|d| d.parse().ok()).collect();
    (
        *parsed.get(0).unwrap_or(&None),
        parsed.get(1).unwrap_or(&None).map(|n| n as u8),
    )
}
