use isbn_register::book::Book;
use isbn_register::isbn::{ISBNError, ISBN};
use isbn_register::lookup;

use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input_isbn = line.expect("unwrapping a line");
        match ISBN::from_str((&input_isbn).trim()) {
            Err(ISBNError::FormNotValid) => {
                println!("Form not valid; maybe less/more characters than  it actually has?");
            }
            Err(ISBNError::CheckDigitNotValid) => {
                println!("Check digit doesn't match.");
            }
            Err(ISBNError::CharsetNotValid { c }) => {
                println!("this input has unexpected char, {}", c);
            }
            Ok(isbn) => {
                let book = lookup(&isbn);
                println!("RESULT==== \n{:?}", book);
            }
        }
    }
}

fn lookup(isbn: &ISBN) -> Option<Book> {
    let book_google = lookup::google_books::lookup_google(&isbn);
    println!("Google has:{:?}", book_google);
    let book_openbd = lookup::openbd::lookup_openbd(&isbn);
    println!("openbd has: {:?}", book_openbd);
    let book_ndl = lookup::ndl_search::lookup_ndl_search(&isbn);
    println!("ndl has: {:?}", book_ndl);
    let mut book = Book::empty_from_isbn(&isbn);
    for result in &[book_google, book_openbd, book_ndl] {
        if let Ok(b) = result {
            book = book.merge_with_longer(b).unwrap();
        }
    }
    Some(book)
}
