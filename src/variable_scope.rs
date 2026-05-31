pub fn variable_scoping() {
    println!("Variable Scope in Rust.");
    // Scope refers to where a variable is allowed to be used.
    //
    // A variable only lives inside the block it was created in.
    // A block is anything inside curly braces {}.

    // - A variable created inside a function only exists in that function.

    fn my_function() {
        let message = "Hello!";
        println!("{}", message); // You can access the message variable here
    }

    my_function();

    // println!("{}", message); // Error - you cannot access the message variable outside of the function

    // - You can create blocks inside other code, like ifs or loops. Variables created inside these blocks can only be used within them.

    let score = 80;

    if score > 50 {
        let result = "Pass";
        println!("Result: {}", result);
    }

    // println!("Result: {}", result); // Error: result is out of scope here

    // - In Rust, you can declare a new variable with the same name in the same scope using let keyword.
    // This is called shadowing, it's used to transform or update values safely.
    let x = 5;
    println!("x is: {}", x); // prints 5
    let x = 10;
    println!("x is: {}", x); // prints 10

    // You can also reuse a variable name inside a new block:
    let y = 5;
    {
        let y = 10;
        println!("Inside block: {}", y);
    }

    println!("Outside block: {}", y);
}
