pub fn ownership_rules() {
    println!("Ownership in Rust");

    // Rust uses Ownership to manage memory in a safe way.
    // Every value in Rust has an owner. The owner is usually a variable.

    // Ownership Rules:
    // - Each value has one owner.
    // - When the owner goes out of scope

    let a = String::from("Hello"); // a owns the string.
    let b = a; // we move it to b.

    // println!("{}", a); // Should error: a no longer owns the value.
    println!("{}", b); // Ok: b now owns the value.

    // When we assign a to b, the ownership moves. This means only b can use the value now and a is no longer valid.
    // Note: Simple types like numbers, characters, and booleans are copied, not movied.

    let x = 5;
    let y = x;

    println!("x = {}", x); // Ok
    println!("y = {}", y); // Ok

    // Cloning
    // For other types, like String, if you really want to keep the original value and assign it to another variable,
    // you can use .clone method which copies the data.

    let s1 = String::from("Hello");
    let s1_clone = s1.clone(); // Now both have the same value.

    println!("original = {}", s1); // Ok
    println!("clone = {}", s1_clone); // Ok

    // Why Ownership matters:
    // - Rust uses it to automatically free memory when its no longer needed.
    // - It prevents bugs like using memory thats already been deleted.
    // - It is one of the reasons Rust is so safe and fast.
}
