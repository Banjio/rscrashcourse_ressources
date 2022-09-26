# Why Rust

Rust is a statically typed compiled language without a garbage collector. Because of this design decisions rust is fast, memory and thread safe as well as reliable making it one of the 

# Rust in vs code

* In 2022 the recommended rust extension is the **rust-analyzer**, and no longer *rls*.

More on this topic here: https://code.visualstudio.com/docs/languages/rust

* You can format your code by running `STRG + SHIFT + P` and type in format document. On the first run you may need to select a debugger we either recommend rust-analyzer or prettier for that

# Crash Course by Traversy Media

**General Information**:

* He will create a new file for every topic he will cover in his video

## Initiaize a project and compile files

Generally you almost always want to use cargo for this job which is rusts build and package management system:

* `cargo init`:  Creates a new cargo project in an existing folder
* `cargo new package_name`: Creates a new package with a new folder package_name
* `cargo build`: Builds a package
* `cargo build --release`: Will build with optimized version
* `cargo run`: Builds ans runs a package

All your dependencies and meta information are handled inside the `cargo.toml`file. 
More on this topic here: https://doc.rust-lang.org/cargo/

## Rust sandbox

Using `cargo run` will create the binary inside ./target/debug/nameofpackage

# Further ressources

* The most comprehensive guide is the official book https://doc.rust-lang.org/book/index.html
* Learning rust by example https://doc.rust-lang.org/rust-by-example/index.html is similiar to go by example. Teachning you typical patterns with practical examples
* Rustlings. Learn rust by small excercices https://github.com/rust-lang/rustlings
* Learn rust in 30 minutes https://fasterthanli.me/articles/a-half-hour-to-learn-rust