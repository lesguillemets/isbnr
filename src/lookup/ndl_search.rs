use reqwest;
use roxmltree;

use crate::book::Book;
use crate::isbn::ISBN;
use crate::lookup::LookupError;

use std::collections::HashMap;
use std::iter::FromIterator;

pub fn lookup_ndl_search(isbn: &ISBN) -> Result<Book, LookupError> {
    let url = format!(
        "http://iss.ndl.go.jp/api/sru?operation=searchRetrieve&query=isbn={}",
        isbn.as_str()
    );
    let response = reqwest::blocking::get(&url).or(Err(LookupError::NetworkIssues))?;
    let response_text = response.text().unwrap();
    // println!("-==> \n {}", response_text);
    let result = roxmltree::Document::parse(&response_text).unwrap();

    let mut books: Vec<Book> = vec![];

    // intended to extract valid results from the xml
    for record_data in result
        .descendants()
        .filter(|n| n.tag_name().name() == "recordData")
        .map(|n| roxmltree::Document::parse(n.text().unwrap_or("")))
        .filter_map(|info| info.ok())
    {
        // xml to hashmap
        let n: HashMap<&str, &str> =
            HashMap::from_iter(record_data.descendants().filter_map(|n| {
                if let Some(text) = n.text() {
                    Some((n.tag_name().name(), text))
                } else {
                    None
                }
            }));
        // gather info from hashmap. I put it here because things to consider changes from sources
        // to sources
        let book = Book {
            title: String::from(*n.get("title").unwrap_or(&"")),
            subtitle: String::from(""),
            authors: vec![String::from(*n.get("creator").unwrap_or(&""))],
            publisher: String::from(*n.get("publisher").unwrap_or(&"")),
            edition: String::from(""),
            volume: None,
            year: None,
            month: None,
            isbn: (*isbn).clone(),
        };
        books.push(book);
    }
    // would like to merge results, but the Kojien results has some unrelated book
    // so I'd rather use the first result for now
    if let Some(b) = books.get(0) {
        if (&b.title).is_empty() {
            // we have a result, but the title is empty!
            Err(LookupError::TitleNotIncluded)
        } else {
            // otherwise, return it
            // FIXME: why do we need a clone?
            Ok(b.clone())
        }
    } else {
        // not a single hit
        Err(LookupError::ResultNotSingle { n: 0 })
    }
}
