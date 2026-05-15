use proc_macros::{private, public, Greet};
use crate::exercises::tic_tac_toe;

mod mod_001a_comments_variables_mutability_scope_shadowing;
mod mod_001b_constants_statics;
mod mod_002_data_types;
mod mod_003_operators;
mod mod_004_functions_and_control_flow;
mod mod_005_text_formatting_and_compiler_directives;
mod mod_006_ownership_and_lifetimes;
mod mod_007_structs_enums_and_collections;
mod mod_008_generics_and_traits;
mod mod_009_error_handling;
mod mod_010_text_processing_file_system_and_env;
mod mod_011_access_control_and_code_organization;
mod mod_012_testing;
mod mod_013_smart_pointers;
mod mod_014_threads_and_concurrency;

mod exercises;

macro_rules! make_struct {
    ($name:ident) => {
        struct $name {
            value: i32,
        }
    };
}

macro_rules! log_all {
    ($($arg:expr),*) => {
        $(println!("log: {}", $arg);)*
    };
}

macro_rules! say {
    () => {
        println!("Hello");
    };
    ($msg:expr) => {
        println!("Message: {}", $msg);
    };
    ($who:expr, $msg:expr) => {
        println!("Message: {} from {}", $msg, $who);
    };
}

macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

fn main() {
    /*let mut input = String::new();
    stdin().read_line(&mut input)
        .expect("Failed to read line");
    let value: i32 = input.parse()
        .expect("Please type a number!");
    println!("echo: {}", value);*/

    make_struct!(Point);

    let p = Point { value: 5 };

    log_all!("hello", 44, true);

    say!();
    say!("abc");
    say!("Jan", "Hello");

    let data = hashmap! {
        "Jan" => 3,
        "Hello" => 5,
    };

    macro_rules! transfer_money {
        (Give $amount:literal) => {
            println!("Give money: {}", $amount);
        };
        (Take $amount:literal) => {
            println!("Take money: {}", $amount);
        }
    }

    transfer_money!(Give 1);
    transfer_money!(Take 2);

    pub trait Greet {
        fn greet(&self);
    }


    // derive macro
    #[derive(Greet)]
    struct Task {
        name: String,
    }

    let a = Task {
        name: String::from("test"),
    };
    a.greet();

    // attribute macro
    #[public(replace=true)]
    struct Training;

    // let t = Training;

    // function like macro
    private!(Training);
}
