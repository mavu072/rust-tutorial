pub fn declare_variables() {
    println!("Declaring and Mutating Variables in Rust");
    // Numbers - Whole numbers and decimal numbers (i32, f64)
    // Characters - Single letters or symbols (char)
    // Strings - Text, a sequence of characters (&str)
    // Booleans - True or false values (bool)

    let x = 5; // let declares a variable
    // x = 5 throw an error - variables can not be changed by default after their created
    // To change the value of a variable - you must use mut keyword(which means mutable/changeable)

    let mut changeable = 9;
    println!("Mutable value before: {}", changeable);
    changeable = 10;
    println!("Mutable value after: {}", changeable);

    // Combining data types
    let name = "Avuyile";
    let age = 26;
    println!("Name: {}; Age: {}; X: {};", name, age, x);
}
