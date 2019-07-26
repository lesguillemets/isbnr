extern crate reqwest;
use std::str::FromStr;

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

pub fn lookup_google(isbn: &str) -> Option<()> {
    let url = format!(
        "https://www.googleapis.com/books/v1/volumes?q=isbn:{}",
        isbn
    );
    let mut response = reqwest::get(&url).unwrap();
    let result = &response.text().unwrap();
    println!("{:?}", result);
    Some(())
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
