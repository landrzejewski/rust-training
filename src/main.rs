/*
https://github.com/landrzejewski/rust-training

Installation/environment setup
- rustup tool from https://rustup.rs
- Visual Code + Rust extension, alternatively RustRover
- git

Important commands:
rustup --version                       # check rustup and rustc version
rustc main.rs                          # compile a file
rustfmt main.rs                        # format source a file
cargo new training_project             # create new project with cargo tool
cargo build                            # build an application in debug mode
cargo run                              # build and run an application in debug mode
cargo build --release                  # build an application in release mode
cargo check                            # check/build code without generating executables
cargo fmt                              # format source files in the project
cargo clippy                           # lint project
cargo clean                            # clean project
*/

mod basic_io;
mod collections_generics_traits;
mod exercises;
mod language_basics;
mod memory_management;
mod wrappers;
mod threads;
mod async_functions;
mod integration;
mod unsafe_code;

fn main() {
    // println!("Hello, world!");

    // language_basics::run();
    // memory_management::run();
    threads::run()
}
