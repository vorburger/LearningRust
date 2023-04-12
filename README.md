# Learning Rust

This is a git repository started by [Michael Vorburger.ch](http://www.vorburger.ch) to learn the [Rust programming language](https://www.rust-lang.org), similar to [my other learning repos](https://github.com/vorburger?tab=repositories&q=Learning&type=&language=&sort=).

* [comprehensive-rust](https://google.github.io/comprehensive-rust) (generated from [comprehensive-rust.git](https://github.com/google/comprehensive-rust)) is a great tutorial to get started with Rust!
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/) (RBE) is another good way to get started.

Install Rust using https://rustup.rs (into `~/.rustup` and `~/.cargo`; uninstall via `rustup self uninstall`), and install [the Rust VSC plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer), and do:

* `cargo new LearningRust` to create the initial files
* `cargo run` will run the [`main.rs`](src/main.rs).
* `cargo fmt` will run [`rustfmt`](https://github.com/rust-lang/rustfmt) (but you should [make your IDE do this on Save](https://github.com/vorburger/vorburger-dotfiles-bin-etc/commit/0a76bfe249a20980a7297e3aaeb5aaad951b035a))
* `cargo clippy` will run the [Clippy](https://github.com/rust-lang/rust-clippy) linter
* `cargo doc --document-private-items` generates `target/doc/help.html`

Find more information on:

* `std` lib docs [such as this](https://doc.rust-lang.org/std/macro.print.html) are fun to browse through.
* [docs.rs](https://docs.rs) has documentation of a lot of other libraries (AKA "[crates](https://crates.io)").
* [Compiler Explorer](https://godbolt.org) shows generated assembly instructions.
