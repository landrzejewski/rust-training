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

mod mod_001a_comments_variables_mutability_scope_shadowing;
mod mod_001b_constants_statics;
mod mod_002_data_types;

fn main() {
   // println!("Hello, world!");\
   mod_001a_comments_variables_mutability_scope_shadowing::run();
}
