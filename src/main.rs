/*
Installation/environment setup:
- rustup tool from https://rustup.rs
- Visual Code + Rust analyzer extension, alternatively RustRover
- git

Important commands:
rustup --version                       # check rustup and rustc version
rustc main.rs                          # compile a file
rustfmt main.rs                        # format a source file
cargo new training_project             # create new project with cargo tool
cargo build                            # build an application in debug mode
cargo run                              # build and run an application in debug mode
cargo build --release                  # build an application in release mode
cargo check                            # check/build code without generating executables
cargo fmt                              # format source files in the project
cargo clippy                           # lint project
cargo clean                            # clean project
*/

use std::io;
use exercises::fibonacci;
use crate::exercises::tictactoe;

mod mod_001a_comments_variables_mutability_scope_shadowing;
mod mod_001b_constants_statics;
mod mod_002_data_types;
mod mod_004_functions_and_control_flow;
mod exercises;
mod mod_006_ownership_and_lifetimes;
mod mod_007_structs_enums_and_collections;
mod mod_008_generics_and_traits;
mod mod_009_error_handling;
mod mod_010_text_processing_file_system_and_env;

fn main() {
   // println!("Hello, world!");

   // Input from cmd

  /* let mut input = String::new();
   let count = io::stdin()
       .read_line(&mut input)
       .unwrap_or(0);
   let value = input.parse::<i32>()
       .unwrap_or(-1);

   println!("{value}");*/
    
   // exercises::tictactoe::run();
    exercises::cat::run();
}
