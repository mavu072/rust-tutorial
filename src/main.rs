mod conditions;
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

use crate::conditions::else_conditions;
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

    let fn_arr = [
        declare_variables,
        create_constants,
        data_types_declaration,
        dt_explicit_declaration,
        use_operators,
        use_assignment_operators,
        use_comparison_operators,
        use_logical_operators,
        else_conditions,
    ];

    border();
    for fnc in fn_arr {
        fnc();
        border();
    }
}
