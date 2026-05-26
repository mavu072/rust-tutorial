// pub exports a function
pub fn data_types_declaration() {
    println!("Declaring Data Types in Rust");

    let my_num = 5; // integer
    let my_double = 5.99; // float
    let my_letter = 'D'; // char
    let my_bool = true; // boolean
    let my_text = "Hello"; // string

    println!(
        "Integer: {}; Float: {}; Char: {}; Boolean: {}, String: {}",
        my_num, my_double, my_letter, my_bool, my_text
    );
}
