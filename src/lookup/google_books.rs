extern crate reqwest;
extern crate serde_json;
// use serde_json::{Result, Value};

use crate::book::Book;
use crate::isbn::ISBN;
use crate::lookup::LookupError;

fn field_as_String_ok(v: &serde_json::Value, field: &str) -> Option<String> {
    v[field].as_str().map(String::from)
}

pub fn lookup_google(isbn: &ISBN) -> Result<Book, LookupError> {
    let url = format!(
        "https://www.googleapis.com/books/v1/volumes?q=isbn:{}",
        isbn.as_str()
    );
    let mut response = reqwest::blocking::get(&url).or(Err(LookupError::NetworkIssues))?;
    let result: serde_json::Value =
        serde_json::from_str(&response.text().expect("g response text")).expect("g parse as json");
    let totalItems = result["totalItems"].as_u64().expect("g total items as u64");
    if totalItems == 1 {
        let thisbook = &result["items"][0];
        let volume_info = &thisbook["volumeInfo"];
        let title =
            field_as_String_ok(&volume_info, "title").ok_or(LookupError::TitleNotIncluded)?;
        let subtitle =
            field_as_String_ok(&volume_info, "subtitle").unwrap_or_else(|| String::from(""));
        let publisher =
            field_as_String_ok(&volume_info, "publisher").unwrap_or_else(|| String::from(""));
        let published =
            field_as_String_ok(&volume_info, "publishedDate").unwrap_or_else(|| String::from(""));
        let (year, month) = parse_hyphen_date(&published);
        let authors: Vec<String> = volume_info["authors"]
            // errors can be ignored here
            .as_array()
            .unwrap_or(&vec![])
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
        Ok(book)
    } else {
        Err(LookupError::ResultNotSingle {
            n: totalItems as usize,
        })
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
