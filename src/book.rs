extern crate reqwest;
extern crate serde_json;
// use serde_json::{Result, Value};
use std::str::FromStr;

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

fn unwrap_field_as_String(v: &serde_json::Value, field: &str) -> String {
    String::from(v[field].as_str().unwrap())
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
        let title = unwrap_field_as_String(&volume_info, "title");
        let subtitle = unwrap_field_as_String(&volume_info, "subtitle");
        let publisher = unwrap_field_as_String(&volume_info, "publisher");
        let published = unwrap_field_as_String(&volume_info, "publishedDate");
        let (year, month) = parse_hyphen_date(&published);
        let authors: Vec<String> = volume_info["authors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| String::from(e.as_str().unwrap()))
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ISBN(String);

impl ISBN {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ISBNError {
    CheckDigitNotValid,
    FormNotValid,
    CharsetNotValid { c: char },
}

impl FromStr for ISBN {
    type Err = ISBNError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // skip hyphens
        let isbn: String = s.chars().filter(|&c| c != '-').collect();
        // any invalid character? (can't use Iterator::any because I want the character)
        for c in (&isbn).chars() {
            if !c.is_digit(10) {
                return Err(ISBNError::CharsetNotValid { c });
            }
        }
        let digits = (&isbn).chars().count();
        // check digits for 10
        if digits == 10 {
            let check: u32 = (&isbn)
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .enumerate()
                .map(|(i, n)| (10 - i as u32) * n)
                .sum();
            if check % 11 == 0 {
                Ok(ISBN(isbn))
            } else {
                Err(ISBNError::CheckDigitNotValid)
            }
        }
        //check digits for 13
        else if digits == 13 {
            let check: u32 = (&isbn)
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .enumerate()
                .map(|(i, n)| if i % 2 == 0 { n } else { 3 * n })
                .sum();
            if check % 10 == 0 {
                Ok(ISBN(isbn))
            } else {
                Err(ISBNError::CheckDigitNotValid)
            }
        } else {
            Err(ISBNError::FormNotValid)
        }
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
