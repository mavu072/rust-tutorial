pub fn simple_functions() {
    println!("Functions in Rust");
    // Creating a function in Rust
    // fn function_name() {
    //     // code to execute
    // }

    // Calling function
    say_hello();

    // Calling function with params
    let name = "Doe";
    greet(name);

    // Calling fn with a return
    let sum = add(3, 2);
    println!("Sum is: {}", sum);

    println!("Sum if V2 is: {}", add_v2(6, 8));

    // Why Use Functions?
    // To organize your code
    // To avoid repeating the same code
    // To make your programs easier to read and change
}

fn say_hello() {
    println!("Hello from the say_hello function!");
}

// Function with parameters
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with return value
fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

// In Rust, you can omit the return keyword. Just write the value on the last line of the function, without a semicolon:
fn add_v2(a: i32, b: i32) -> i32 {
    a + b
}
