pub fn create_constants() {
    println!("Creating Constants in Rust");

    const BIRTHYEAR: i32 = 2000; // const creates a constant
    const MINUTES_PER_HOUR: i32 = 60; // a constant is a value that can not be changed or re-assigned
    // Rust constants must have a type, or Rust throws an error
    // Uppercase is good practice

    println!(
        "BIRTHYEAR: {}; MINUTES_PER_HOUR: {}",
        BIRTHYEAR, MINUTES_PER_HOUR
    );
}
