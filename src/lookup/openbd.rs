extern crate reqwest;
extern crate serde_json;

use crate::book::Book;
use crate::isbn::ISBN;
use crate::lookup::LookupError;

fn field_as_String_ok(v: &serde_json::Value, field: &str) -> Option<String> {
    v[field].as_str().map(String::from)
}

pub fn lookup_openbd(isbn: &ISBN) -> Result<Book, LookupError> {
    let url = format!("https://api.openbd.jp/v1/get?isbn={}", isbn.as_str());
    let mut response = reqwest::get(&url).or(Err(LookupError::NetworkIssues))?;
    let result: serde_json::Value = serde_json::from_str(&response.text().unwrap()).unwrap();
    let book_info = &result[0];
    if book_info.is_null() {
        return Err(LookupError::ResultNotSingle { n: 0 });
    }
    // for now we'll ignore "hanmoto" results
    let onix_info = &book_info["onix"];
    let descriptive_detail = &onix_info["DescriptiveDetail"];
    // this is different from what specified by the url (https://openbd.jp/spec/)
    // TODO: consider "collationkey"
    println!(
        "{:?}",
        descriptive_detail["TitleDetail"]["TitleElement"]["TitleText"]["content"]
    );
    let title = &descriptive_detail["TitleDetail"]["TitleElement"]["TitleText"]["content"]
        .as_str()
        .map(String::from)
        .ok_or(LookupError::TitleNotIncluded)?;
    Err(LookupError::NetworkIssues)
}
