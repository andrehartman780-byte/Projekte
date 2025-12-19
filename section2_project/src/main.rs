use std::collections::HashMap;

pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub year: u16,
}

impl Book {
    pub fn new(id: u32, title: &str, author: &str, year: u16) -> Self {
        todo!("construct a Book using the provided values")
    }

    pub fn title(&self) -> &str {
        todo!("return a string slice of the title")
    }
}

pub fn build_index(books: Vec<Book>) -> HashMap<u32, Book> {
    todo!("turn the vector into a HashMap keyed by id")
}

pub fn get_book<'a>(index: &'a HashMap<u32, Book>, id: u32) -> Option<&'a Book> {
    todo!("return Some(&Book) if id exists, otherwise None")
}

pub fn parse_id(input: &str) -> Result<u32, String> {
    todo!("parse a u32 from input, return Err with a helpful message on failure")
}

pub fn filter_titles<'a, F>(books: &'a [Book], predicate: F) -> Vec<&'a str>
where
    F: Fn(&Book) -> bool,
{
    todo!("use the closure to filter books and collect title slices")
}

pub fn title_parts<'a>(title: &'a str) -> (&'a str, Option<&'a str>) {
    todo!("split title on the first ':' and return (main, subtitle)")
}

fn main() {
    println!("Section 2 project: open instructions.html and run cargo test.");
}

#[cfg(test)]
mod tests;
