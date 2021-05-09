# Simple multithreaded web server build with Rust.

## Made with [last chapter from the book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)

## Code is divided into *main.rs* and *lib.rs*:
- *lib.rs* defines a structure *ThreadPool* that contains logic for handling many requests with multithreading
- *main.rs* simply takes the requests and passes them to the instance of *ThreadPool* for execution