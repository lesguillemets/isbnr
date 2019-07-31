extern crate reqwest;
extern crate serde_json;
// use serde_json::{Result, Value};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Book {
    title: String,
    authors: Vec<String>,
    publisher: String,
    edition: String,
    volume: Option<u16>,
    year: Option<u16>,
    month: Option<u8>,
    isbn: String,
}

pub fn lookup_google(isbn: &str) -> Option<Book> {
    let url = format!(
        "https://www.googleapis.com/books/v1/volumes?q=isbn:{}",
        isbn
    );
    let mut response = reqwest::get(&url).unwrap();
    let result: serde_json::Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    if result["totalItems"].as_u64() == Some(1) {
        let thisbook = &result["items"][0];
        let volume_info = &thisbook["volumeInfo"];
        let title = String::from(volume_info["title"].as_str().unwrap());
        let publisher = String::from(volume_info["publisher"].as_str().unwrap());
        let authors: Vec<String> = volume_info["authors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| String::from(e.as_str().unwrap()))
            .collect();
        let book = Book {
            title,
            authors,
            publisher,
            edition: String::from(""),
            volume: None,
            year: None,
            month: None,
            isbn: String::from(""),
        };
        Some(book)
    } else {
        None
    }
}

#[derive(Debug)]
pub struct ISBN(String);

#[derive(Debug)]
pub enum ISBNError {
    CheckDigitNotValid,
    FormNotValid,
    CharsetNotValid { c: char },
}

impl FromStr for ISBN {
    type Err = ISBNError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut isbn = String::from("");
        for c in s.chars() {
            if c == '-' {
                continue;
            } else if c.is_digit(10) {
                let x = 3;
                isbn.push(c);
            } else {
                return Err(ISBNError::CharsetNotValid { c });
            }
        }
        let digits = (&isbn).chars().count();
        if digits != 10 && digits != 13 {
            return Err(ISBNError::FormNotValid);
        }
        Ok(ISBN(isbn))
    }
}
