# Learning Rust

This is a git repository started by [Michael Vorburger.ch](http://www.vorburger.ch) to learn the [Rust programming language](https://www.rust-lang.org), similar to [my other learning repos](https://github.com/vorburger?tab=repositories&q=Learning&type=&language=&sort=).

[comprehensive-rust](https://google.github.io/comprehensive-rust) (generated from [comprehensive-rust.git](https://github.com/google/comprehensive-rust)) is a great tutorial to get started with Rust!

[Rust by Example](https://doc.rust-lang.org/rust-by-example/) (RBE) is another way to get started.

Install Rust using https://rustup.rs (into `~/.rustup` and `~/.cargo`; uninstall via `rustup self uninstall`), and install [the Rust VSC plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

`cargo new LearningRust` created the initial files here.

`cargo run` will run the [`main.rs`](src/main.rs).

`std` lib docs [such as this](https://doc.rust-lang.org/std/macro.print.html) are fun to browse through.

[docs.rs](https://docs.rs) has documentation of a lot of other libraries (AKA "[crates](https://crates.io)").

`cargo doc --document-private-items` generates `target/doc/help.html`.

[Compiler Explorer](https://godbolt.org) shows generated assembly instructions.
