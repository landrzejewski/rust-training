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
*/

#![allow(dead_code)]
// crate scope attribute (entire application)
mod basic_io;
mod collections_generics_traits;
mod exercises;
mod language_basics;
mod memory_management;
mod async_functions;
mod threads;

fn main() {
    // println!("Hello World in Rust");

    // language_basics::run();
    // memory_management::run();
    // collections_generics_traits::run();
    // basic_io::run();
    // integration::run();
    //threads::run();
     async_functions::run();

    //exercises::tic_tac_toe::run();

}
