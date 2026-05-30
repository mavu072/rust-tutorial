pub fn while_loop() {
    println!("While Loops in a Rust");
    // Use while when you want to repeat code until something happens.

    let mut count: i32 = 1;

    while count <= 5 {
        println!("Loading... {}", count);
        count += 1;
    }

    // False conditions will never run
    count = 10;

    while count <= 5 {
        println!("This will never be printed.")
    }

    // Use the 'break' keyword to stop a while loop.
    let mut num: i32 = 1;

    while num <= 10 {
        if num == 6 {
            println!("I'm tired boss.");
            break;
        }
        println!("Counting: {}", num);
        num += 1;
    }

    // Use the 'continue' keyword to skip an iteration.
    let mut number: i32 = 1;

    while number <= 10 {
        if number == 3 {
            println!("Skipped");
            number += 1; // Make sure to increment before skipping otherwise this loops forever.
            continue;
        }
        println!("Counting: {}", number);
        number += 1;
    }
}
