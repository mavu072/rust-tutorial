mod constants; // module import in rust
mod data_types; // module import in rust
mod data_types_explicit;
mod operators;
mod variables;

use constants::create_constants; // use imports a function
use data_types::data_types_declaration; // import function
use data_types_explicit::dt_explicit_declaration;
use operators::use_assignment_operators;
use operators::use_operators;
use variables::declare_variables;

use crate::operators::use_comparison_operators;
use crate::operators::use_logical_operators;

fn border() {
    println!(
        "-------------------------------------------------------------------------------------------------------------"
    );
}

fn main() {
    let name = "Avuyile";
    println!(
        "Hello, {}! This the Rust main function - your entry point",
        name
    ); // {} is a placeholder to show variables;
    // println is a macro
    // A macro is like a function, but with an exclamation mark (!) after it. They're like functions but some time have different rules

    border();
    declare_variables();
    border();
    create_constants();
    border();
    data_types_declaration();
    border();
    dt_explicit_declaration();
    border();
    use_operators();
    border();
    use_assignment_operators();
    border();
    use_comparison_operators();
    border();
    use_logical_operators();
    border();
}
