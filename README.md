# Advanced Solana Bootcamp Homework 2
## Sample Library Program:
Write a Rust program that represents a file management system. This system should be designed to have different sources of books and magazines. Your system must meet the following standards:
Number the publication one. This number should not contain two: Book and Magazine. Both should contain different data types:
The book content must contain a structure with title, author and page_num values.
The content of the journal should include a structure with the content of the title, issue and subject.
Creates book and magazine instances, creating a Vec<Publication> array containing these instances.
Write a function that prints each type in the array in a different format based on its type (book or magazine). For example, use the format "Book: [title] author: [author], [page_count] page" for books and "Journal: [title] - Issue: [issue], Subject: [topic]" for magazines.

##  For testing locally : 
```bash
cd advanced_solana_bootcamp_hw2

```
```bash
cargo build
```
```bash
cargo run
```
