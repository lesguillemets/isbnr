pub mod google_books;

#[derive(Debug, Eq, PartialEq)]
pub enum LookupError {
    ResultNotSingle { n: usize },
    TitleNotIncluded,
}
