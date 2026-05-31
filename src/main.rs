mod borrowing;
mod conditions;
mod constants; // module import in rust
mod data_types; // module import in rust
mod data_types_explicit;
mod for_loops;
mod functions;
mod loops;
mod operators;
mod ownership;
mod strings;
mod variable_scope;
mod variables;
mod while_loops;

use constants::create_constants; // use imports a function
use data_types::data_types_declaration; // import function
use data_types_explicit::dt_explicit_declaration;
use operators::use_assignment_operators;
use operators::use_operators;
use variables::declare_variables;

use crate::borrowing::borrowing_and_references;
use crate::conditions::else_conditions;
use crate::conditions::match_condition;
use crate::for_loops::for_loop;
use crate::functions::simple_functions;
use crate::loops::simple_loop;
use crate::operators::use_comparison_operators;
use crate::operators::use_logical_operators;
use crate::ownership::ownership_rules;
use crate::strings::string_type;
use crate::variable_scope::variable_scoping;
use crate::while_loops::while_loop;

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
        match_condition,
        simple_loop,
        while_loop,
        for_loop,
        simple_functions,
        variable_scoping,
        string_type,
        ownership_rules,
        borrowing_and_references,
    ];

    border();
    for fnc in fn_arr {
        fnc();
        border();
    }
}
