pub fn string_type() {
    println!("Strings in Rust");
    // String store text.

    let greeting: &str = "Hello";
    println!("{}", greeting);

    // There are two types of strings in Rust:
    // - &str - is called "string slices", and is used for fixed text like "Hello".
    // - String - used when you need a string that can change.

    // Create a String
    // You can create a String from string literal using the to_string and String::from function:
    let text1 = "Hello World".to_string();
    let text2 = String::from("Hello World");
    println!("Text 1: {}", text1);
    println!("Text 2: {}", text2);

    // Note: A string literal is a fixed sequence of characters directly hardcoded into a program's source code.
    // It is called a "literal" because it is interpreted exactly as written rather than being evaluated as a variable or computed expression

    // Change a String
    // - Use push_str to add text to a string:
    let mut greeting = String::from("Hello");
    greeting.push_str(" World");
    println!("{}", greeting); // Hello World

    // - Use push to add on character to a string:
    let mut word = String::from("Hi");
    word.push('!');
    println!("{}", word); // Hi!

    // Concatenate Strings
    // You can combin strings using the format! macro:
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");
    let result = format!("{} {} {}", s1, s2, s3);
    println!("{}", result);

    // You can also use the + operator but gets messy with many values:
    let result2 = s1 + " " + &s2 + " " + &s3;
    println!("{}", result2);

    // String Length
    // You can use the .len() method to get the length of a string:
    let name = String::from("John");
    println!("Length: {}", name.len());
}
