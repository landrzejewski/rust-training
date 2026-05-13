use std::io::stdin;
use exercises::fibonacci;

mod mod_001a_comments_variables_mutability_scope_shadowing;
mod mod_001b_constants_statics;
mod mod_002_data_types;
mod mod_003_operators;
mod mod_004_functions_and_control_flow;
mod mod_005_text_formatting_and_compiler_directives;
mod mod_006_ownership_and_lifetimes;
mod exercises;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input)
        .expect("Failed to read line");
    let value: i32 = input.parse()
        .expect("Please type a number!");
    println!("echo: {}", value);

}
