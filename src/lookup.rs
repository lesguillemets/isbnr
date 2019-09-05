pub mod google_books;
pub mod ndl_search;

#[derive(Debug, Eq, PartialEq)]
pub enum LookupError {
    ResultNotSingle { n: usize },
    TitleNotIncluded,
}
