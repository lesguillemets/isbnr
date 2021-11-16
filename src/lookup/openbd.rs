extern crate reqwest;
extern crate serde_json;

use crate::book::Book;
use crate::isbn::ISBN;
use crate::lookup::LookupError;

trait AsString {
    fn as_string(&self) -> Option<String>;
    fn as_string_or_empty(&self) -> String {
        (self.as_string().unwrap_or_else(|| String::from("")))
    }
}

impl AsString for serde_json::Value {
    fn as_string(&self) -> Option<String> {
        self.as_str().map(String::from)
    }
}

pub fn lookup_openbd(isbn: &ISBN) -> Result<Book, LookupError> {
    let url = format!("https://api.openbd.jp/v1/get?isbn={}", isbn.as_str());
    let mut response = reqwest::blocking::get(&url).or(Err(LookupError::NetworkIssues))?;
    let result: serde_json::Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    let book_info = &result[0];
    if book_info.is_null() {
        return Err(LookupError::ResultNotSingle { n: 0 });
    }
    // for now we'll ignore "hanmoto" results
    let onix_info = &book_info["onix"];
    let descriptive_detail = &onix_info["DescriptiveDetail"];
    let summary = &onix_info["summary"];
    // this is different from what specified by the url (https://openbd.jp/spec/)
    // TODO: consider "collationkey"
    let title = descriptive_detail["TitleDetail"]["TitleElement"]["TitleText"]["content"]
        .as_str()
        .map(|s| String::from(s).normalise_whitespace())
        .ok_or(LookupError::TitleNotIncluded)?;
    let subtitle = descriptive_detail["TitleDetail"]["TitleElement"]["TitleText"]["SubTitle"]
        .as_string_or_empty()
        .normalise_whitespace();
    let authors: Vec<String> = descriptive_detail["Contributor"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|n| {
            n["PersonName"]["content"]
                .as_string_or_empty()
                .normalise_whitespace()
        })
        .collect();
    let publisher = summary["publisher"]
        .as_string_or_empty()
        .normalise_whitespace();
    let volume = summary["volume"].as_str().and_then(|t| t.parse().ok());
    // FIXME : publishing date has (at least two) different codings.
    // yyyy-mm and yyyymmdd etc.
    Ok(Book {
        title,
        subtitle,
        authors,
        publisher,
        edition: String::from(""),
        volume,
        year: None,
        month: None,
        isbn: (*isbn).clone(),
    })
}

// dirty hack for method chain
trait NormaliseWhitespace {
    fn normalise_whitespace(&self) -> String;
}
impl NormaliseWhitespace for String {
    fn normalise_whitespace(&self) -> String {
        self.chars()
            .map(|c| if c.is_whitespace() { ' ' } else { c })
            .collect()
    }
}
