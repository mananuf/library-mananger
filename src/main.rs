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

#[derive(Debug)]
enum BookStatus {
    Available,
    CheckedOut,
    OnHold,
    InRepair,
}

#[derive(Debug)]
struct Libary {
    books: Vec<Book>,
}

impl Libary {
    fn new() -> Self {
        Self { books: Vec::new() }
    }

    fn add_book(&mut self, book: Book) -> () {
        self.books.push(book)
    }

    fn checkout(&mut self, title: String) -> () {
        if let Some(book) = self.books.iter_mut().find(|book| book.title == title) {
            book.status = BookStatus::CheckedOut;
            println!("Book Status updated for {}", book.title);
        } else {
            println!("Book does not exists");
        }
    }

    fn all_books(&self) -> &Vec<Book> {
        &self.books
    }
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

    let mut library: Libary = Libary::new();

    library.add_book(things_fall_apart);
    library.add_book(purple_hibiscus);

    dbg!(library.all_books());

    library.checkout(String::from("purple hibiscus"));
    dbg!(library.all_books());
}
