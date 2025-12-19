use super::*;

#[test]
fn book_new_and_title() {
    let book = Book::new(1, "Rust 101", "Ada", 2021);
    assert_eq!(book.id, 1);
    assert_eq!(book.author, "Ada");
    assert_eq!(book.year, 2021);
    assert_eq!(book.title(), "Rust 101");
}

#[test]
fn index_and_lookup() {
    let books = vec![
        Book::new(1, "Alpha", "A. Author", 1999),
        Book::new(2, "Beta", "B. Author", 2005),
    ];
    let index = build_index(books);
    assert_eq!(index.len(), 2);
    let found = get_book(&index, 2).expect("book id 2 should exist");
    assert_eq!(found.title(), "Beta");
    assert!(get_book(&index, 99).is_none());
}

#[test]
fn parse_id_result() {
    assert_eq!(parse_id("42").unwrap(), 42);
    assert!(parse_id("nope").is_err());
}

#[test]
fn filter_titles_with_closure() {
    let books = vec![
        Book::new(1, "Old Tales", "C. Author", 1980),
        Book::new(2, "Modern Rust", "D. Author", 2020),
        Book::new(3, "Future Ideas", "E. Author", 2024),
    ];
    let titles = filter_titles(&books, |book| book.year >= 2000);
    assert_eq!(titles.len(), 2);
    assert!(titles.contains(&"Modern Rust"));
}

#[test]
fn title_parts_split() {
    let (main, sub) = title_parts("Rust:Book");
    assert_eq!(main, "Rust");
    assert_eq!(sub, Some("Book"));
}
