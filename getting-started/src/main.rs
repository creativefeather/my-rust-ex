use std::env;

#[derive(Debug)]
struct Book {
    name: String,
    author: String,
    subject: String,
    book_id: i32
}

fn main() {
    let book1 = Book {
        name: "Rust Programming".to_string(),
        author: "John Doe".to_string(),
        subject: "Programming".to_string(),
        book_id: 12
    };

    let book2 = Book {
        name: "C Programming".to_string(),
        author: "Pete Jones".to_string(),
        subject: "Programming".to_string(),
        book_id: 13
    };

    let books = vec![book1, book2];
    let mut new_books: Vec<&Book> = vec![];

    for book in books.iter() {
        let book_1 = book;
        // do something wiht book_1
        new_books.push(book_1);
    }

    println!("{:?}", books);

    log_env_args();
}

fn log_env_args() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        println!("{}", arg);
    }
}