extern crate reqwest;
extern crate roxmltree;

use crate::book::Book;
use crate::isbn::ISBN;
use crate::lookup::LookupError;

pub fn lookup_ndl_search(isbn: &ISBN) -> Result<Book, LookupError> {
    let url = format!(
        "http://iss.ndl.go.jp/api/sru?operation=searchRetrieve&query=isbn={}",
        isbn.as_str()
    );
    let mut response = reqwest::get(&url).unwrap();
    let text = response.text().unwrap();
    let result = roxmltree::Document::parse(&text).unwrap();
    for node in result
        .descendants()
        .filter(|n| n.tag_name().name() == "recordData")
    {
        let record_text = node.text();
        println!(
            "-->\n{:?}",
            roxmltree::Document::parse(&record_text.unwrap_or(""))
        );
    }
    Err(LookupError::TitleNotIncluded)
}
