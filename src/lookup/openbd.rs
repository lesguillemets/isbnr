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
    println!("{:?}", result);
    Err(LookupError::NetworkIssues)
}
