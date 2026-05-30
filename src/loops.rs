pub fn simple_loop() {
    println!("Loops in Rust");
    // Use loop when you don't know in advance how many times to repeat.

    // loop {
    //     println!("This will repeat forever!");
    // }

    // A loop needs an exit condition
    // Use 'break' to stop a loop.
    let mut count = 0;
    loop {
        println!("Hello master! {}", count);

        if count == 3 {
            break;
        }

        count += 1;
    }

    // Return a value
    count = 1;

    let result = loop {
        println!("Why are you ignoring me my master!");

        if count == 3 {
            break count; // Stop looping and return count.
        }

        count += 1;
    }; // When you return the result to a value you must include a semi colon, but a good formatter takes care of this for you.

    println!("The loop stopped at: {}", result);
}
