extern crate reqwest;

use isbn_register::isbn::ISBN;
use isbn_register::lookup;

use std::str::FromStr;

fn main() {
    let isbn = ISBN::from_str("978-4-00-080131-7").unwrap();
    let book_google = lookup::google_books::lookup_google(&isbn);
    println!("Google has:{:?}", book_google);
    let book_ndl = lookup::ndl_search::lookup_ndl_search(&isbn);
    println!("ndl has: {:?}", book_ndl);
    let book = book_google.unwrap().merge_with_longer(&book_ndl.unwrap());
    println!("result:\n\t {:?}", book);
}
