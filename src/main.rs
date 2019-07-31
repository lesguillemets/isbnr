extern crate reqwest;

use isbn_register::book;
use isbn_register::book::Book;

use std::str::FromStr;
use std::vec::Vec;

fn main() {
    let isbn = book::ISBN::from_str("9780702055560").unwrap();
    let book = book::lookup_google(&isbn);
    println!("{:?}", book);
}
