pub fn else_conditions() {
    println!("Conditions and If..Else in Rust");
    // Used to perform different actions for different decisions

    // an 'if' specifies a block that should execute when a condition is true.
    if 7 > 5 {
        println!("7 is greater than 5");
    }

    let x = 7;
    let y = 5;

    if x > y {
        println!("x is greater than y");
    }

    // In an 'if...else', the 'else' runs different code if the condition is not true.
    let age: i32 = 14;
    if age >= 15 {
        println!("You're a wizard Harry.");
    } else {
        println!("Go now! Under the stairwell boy!")
    }

    // Add an 'else if' for multiple conditions.
    let score: i32 = 84;

    if score >= 90 {
        println!("Grade: A")
    } else if score >= 80 {
        println!("Grade: B")
    } else if score >= 70 {
        println!("Grade: C")
    } else {
        println!("Grade: F")
    }

    // Use an if...else as an expression. You can do it in Rust.
    let time = 20;
    let greeting = if time < 18 {
        "Good day sir."
    } else {
        "Good evening stranger."
    };

    println!("{}", greeting);

    // Do not mix types, the value from an if and else block must be the same type.
    // let result = if 5 < 10 { "Too small" } else { 100 }; // error[E0308]: `if` and `else` have incompatible types
}

pub fn match_condition() {
    println!("Conditions Match in Rust");
    // When you have many options its easier to use a Match than a lot of if...else's.

    let day = 2;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }
}
