use isbn_register::book::ISBN;
use std::str::FromStr;

#[test]
fn isbn_examples() {
    assert_eq!(
        ISBN::from_str("9780702055560").unwrap().as_str(),
        "9780702055560"
    );
}
