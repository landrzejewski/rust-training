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


fn main() {
    tic_tac_toe::run();
    
    /*let mut input = String::new();
    stdin().read_line(&mut input)
        .expect("Failed to read line");
    let value: i32 = input.parse()
        .expect("Please type a number!");
    println!("echo: {}", value);*/

}
