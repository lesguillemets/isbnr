extern crate reqwest;

use isbn_register::book;
use isbn_register::isbn::ISBN;
use isbn_register::lookup;

use std::str::FromStr;
use std::vec::Vec;

fn main() {
    let isbn = ISBN::from_str("9780702055560").unwrap();
    let book = lookup::google_books::lookup_google(&isbn);
    println!("{:?}", book);
}
