use crate::isbn::ISBN;
use std::cmp::Eq;
use std::collections::HashMap;

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

    pub fn merge_with_longer(self, other: &Self) -> Option<Self> {
        if self.isbn != other.isbn {
            None
        } else {
            Some(Book {
                title: f(&self.title, &other.title),
                subtitle: f(&self.subtitle, &other.subtitle),
                // for authors, prefer the longer list
                // (if same, entry from self is used)
                authors: if self.authors.len() >= other.authors.len() {
                    self.authors.clone()
                } else {
                    other.authors.clone()
                },
                publisher: f(&self.publisher, &other.publisher),
                edition: f(&self.edition, &other.edition),
                volume: *g(&self.volume, &other.volume),
                year: *g(&self.year, &other.year),
                month: *g(&self.month, &other.month),
                isbn: self.isbn.clone(),
            })
        }
    }
}

fn g<'a, T: Eq>(a: &'a Option<T>, b: &'a Option<T>) -> &'a Option<T> {
    match (a, b) {
        (Some(ai), Some(bi)) => {
            if ai == bi {
                a
            } else {
                &None
            }
        }
        (Some(_), None) => a,
        (None, Some(_)) => b,
        _ => &None,
    }
}

fn f<'a>(a: &'a str, b: &'a str) -> String {
    String::from(max_by_length(a, b))
}

fn max_by_length<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.chars().count() >= b.chars().count() {
        a
    } else {
        b
    }
}
