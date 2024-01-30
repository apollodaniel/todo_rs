# Todo App Rust
This is a todo cli app made in Rust that uses sqlite3 for data storage.


Usage
```
todo_rs <command> <name or index>
```

Some commands like remove, mark and unmark can be used using the index.

Install using
```
cargo build --release && sudo cp target/release/todoapp /usr/local/bin/todo_rs && sudo chmod a+x /usr/local/bin/todo_rs
```

### Preview
![preview](image/preview.gif)

### Dependencies
- [rusqlite](https://crates.io/crates/rusqlite) -  Ergonomic wrapper for SQLite 
- [simple-home-dir](https://crates.io/crates/simple-home-dir) - Retrieve user directory