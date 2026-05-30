pub fn for_loop() {
    println!("For Loops in Rust");
    // Repeats code a fixed number of times.

    for i in 1..6 {
        println!("i is: {}", i);
    }

    // Inclusive Range: if you want to include the last number use '..=' (two dots and an equals sign)
    for i in 1..=6 {
        println!("i is: {}", i);
    }

    // You can use 'break' and 'continue' just like other loops.
    for i in 1..=10 {
        if i == 3 {
            continue; // Skip
        }
        if i == 5 {
            break; // Stop
        }
        println!("i is: {}", i);
    }
}
