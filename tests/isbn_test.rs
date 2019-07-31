use isbn_register::book::{ISBNError, ISBN};
use std::str::FromStr;

#[test]
fn isbn_example_10digits() {
    assert_eq!(ISBN::from_str("0764526413").unwrap().as_str(), "0764526413");
}
#[test]
fn isbn_example_10digits_fail() {
    assert_eq!(
        ISBN::from_str("0754526413"),
        Err(ISBNError::CheckDigitNotValid)
    );
}

#[test]
fn isbn_example_13digits() {
    assert_eq!(
        ISBN::from_str("9780702055560").unwrap().as_str(),
        "9780702055560"
    );
}

#[test]
fn isbn_example_13digits_fail() {
    assert_eq!(
        ISBN::from_str("9783702055560"),
        Err(ISBNError::CheckDigitNotValid)
    );
}

#[test]
fn isbn_example_digit_tooshort() {
    assert_eq!(ISBN::from_str("97837550"), Err(ISBNError::FormNotValid));
}

#[test]
fn isbn_example_digit_toolong() {
    assert_eq!(
        ISBN::from_str("978392394323755560"),
        Err(ISBNError::FormNotValid)
    );
}

#[test]
fn isbn_example_13digits_with_hyphen() {
    assert_eq!(
        ISBN::from_str("978-0702055560").unwrap().as_str(),
        "9780702055560"
    );
}

#[test]
fn isbn_example_invalid_char() {
    assert_eq!(
        ISBN::from_str("978a0k02055560"),
        Err(ISBNError::CharsetNotValid { c: 'a' })
    );
}
