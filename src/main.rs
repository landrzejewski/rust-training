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
cargo clean                            # clean project
*/
use rust_training::threads;

fn main() {
    // language_basics::run();
    // fibonacci::run();
    // money::run();
    // money_with_enums::run();
    // memory_management::run();
    // guess_game::run();
    // tic_tac_toe::run()
    // collections_generics_traits::run()
    // basic_io::run();
    // budget::run();
    // echo::run();
    // cat::run();
    // wc::run();
    // find::run();
    // grep::run();
    // wrappers::run();
    threads::run();
}
