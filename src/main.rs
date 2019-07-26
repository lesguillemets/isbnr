extern crate reqwest;

use isbn_register::book;
use isbn_register::book::Book;

use std::vec::Vec;

fn main() {
    book::lookup_google("9780702055560");
}
