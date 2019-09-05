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
    let mut response = reqwest::get(&url).or(Err(LookupError::NetworkIssues))?;
    let text = response.text().unwrap();
    let result = roxmltree::Document::parse(&text).unwrap();
    let mut entries: Vec<Book> = vec![];
    for node in result
        .descendants()
        .filter(|n| n.tag_name().name() == "recordData")
        .map(|n| roxmltree::Document::parse(n.text().unwrap_or("")))
        .filter(|info| info.is_ok())
    {
        let mut b = Book::empty_from_isbn(isbn);
        println!("-->\n{:?}", node);
    }
    Err(LookupError::TitleNotIncluded)
}
