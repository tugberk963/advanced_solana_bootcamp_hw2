enum Publication {
    Book(Book),
    Magazine(Magazine),
}

struct Book {
    title: String,
    author: String,
    page_count: u32,
}

struct Magazine {
    title: String,
    topic: String,
    issue: u32,
}

fn print_publications(publications: Vec<Publication>) {
    for publication in publications {
        match publication {
            Publication::Book(book) => {
                println!(
                    "Book: {} author: {}, {} pages",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(magazine) => {
                println!(
                    "Magazine: {} - Topic: {} - Issue: {}",
                    magazine.title, magazine.topic, magazine.issue,
                );
            }
        }
    }
}

fn main() {
    let book = Book {title: "Book Title".to_string(),author: "Book Author".to_string(),page_count: 123123,};
    let magazine = Magazine {title: "Magazine Title".to_string(),topic: "Magazine Topic".to_string(),issue: 123123,};

    let mut pubs = Vec::new();
    pubs.push(Publication::Book(book));
    pubs.push(Publication::Magazine(magazine));

    print_publications(pubs);
}