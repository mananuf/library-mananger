use std::fmt::Debug;

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    isbn: u32,
    publication_date: String,
    status: BookStatus,
}

impl Book {
    fn new(title: String, author: String, isbn: u32, publication_date: String) -> Self {
        Self {
            title,
            author,
            isbn,
            publication_date,
            status: BookStatus::Available,
        }
    }
}

impl Readable for Book {
    fn title(&self) -> &str {
        &self.title
    }

    fn author(&self) -> &str {
        &self.author
    }

    fn publication_year(&self) -> u16 {
        2025
    }
}

#[derive(Debug)]
enum BookStatus {
    Available,
    CheckedOut,
    OnHold,
    InRepair,
}

#[derive(Debug)]
struct Libary {
    books: Vec<Box<dyn Readable>>,
}

trait Readable: Debug {
    fn title(&self) -> &str;
    fn author(&self) -> &str;
    fn publication_year(&self) -> u16;
    fn summary(&self) -> String {
        format!(
            "{} by {}, published in {}",
            self.title(),
            self.author(),
            self.publication_year()
        )
    }
}

#[derive(Debug)]
struct Magazine {
    title: String,
    author: String,
    publication_date: u16,
}

impl Magazine {
    fn new(title: String, author: String, publication_date: u16) -> Self {
        Self {
            title,
            author,
            publication_date,
        }
    }
}

impl Readable for Magazine {
    fn author(&self) -> &str {
        &self.author
    }

    fn publication_year(&self) -> u16 {
        self.publication_date
    }

    fn title(&self) -> &str {
        &self.title
    }
}

impl Libary {
    fn new() -> Self {
        Self { books: Vec::new() }
    }

    fn add_to_collection<T: Readable + 'static>(&mut self, item: T) -> () {
        self.books.push(Box::new(item))
    }

    fn checkout(&mut self, title: String) -> () {
        if let Some(book) = self.books.iter_mut().find(|book| book.title() == title) {
            // book.status = BookStatus::CheckedOut;
            println!("Book Status updated for {}", book.title());
        } else {
            println!("Book does not exists");
        }
    }

    fn all_books(&self) -> &Vec<Box<dyn Readable + 'static>> {
        &self.books
    }
}

fn print_details<T: Readable>(item: &T) -> String {
    format!(
        "TITLE: {} \nAUTHOR: {} \nYEAR: {}",
        item.title(),
        item.author(),
        item.publication_year()
    )
}

fn main() {
    let things_fall_apart: Book = Book::new(
        String::from("things fall apart"),
        String::from("Chinua Achebe"),
        204,
        String::from("10th april 1984"),
    );

    let purple_hibiscus: Book = Book::new(
        String::from("purple hibiscus"),
        String::from("Chiamanda"),
        204,
        String::from("10th april 2008"),
    );

    let forbes: Magazine = Magazine::new(
        "forbes Magazine".to_string(),
        "The Grammys".to_string(),
        2020,
    );

    let mut library: Libary = Libary::new();

    print_details(&purple_hibiscus);

    library.add_to_collection(things_fall_apart);
    library.add_to_collection(purple_hibiscus);
    library.add_to_collection(forbes);

    dbg!(library.all_books());

    library.checkout(String::from("purple hibiscus"));
    dbg!(library.all_books());
}
