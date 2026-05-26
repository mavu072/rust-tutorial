// pub exports a function
pub fn dt_explicit_declaration() {
    println!("Explicit Declaration of Data Types in Rust");

    let my_num: i32 = 5; // integer
    let my_double: f64 = 5.99; // float
    let my_letter: char = 'D'; // char
    let my_bool: bool = true; // boolean
    let my_text: &str = "Hello"; // string

    println!(
        "Integer: {}; Float: {}; Char: {}; Boolean: {}, String: {}",
        my_num, my_double, my_letter, my_bool, my_text
    );
}
