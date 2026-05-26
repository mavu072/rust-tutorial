pub fn use_operators() {
    println!("Using Operators in Rust");

    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Subtract: {}", sub);
    println!("Multiply: {}", mul);
    println!("Divide: {}", div);
    println!("Remainder: {}", rem);
}

pub fn use_assignment_operators() {
    println!("Using Assignment Operators in Rust");
    // Assignment operators assign or modify values

    let mut x = 10;
    println!("Start: {}", x);

    x += 5;
    println!("After +=5: {}", x);

    x -= 2;
    println!("After -=2: {}", x);

    x *= 2;
    println!("After *=2: {}", x);

    x /= 3;
    println!("After /=3: {}", x);

    x %= 4;
    println!("After %=4: {}", x);

    // Operator	Example	Same As
    // =	x = 5	Assign 5 to x
    // +=	x += 3	x = x + 3
    // -=	x -= 2	x = x - 2
    // *=	x *= 4	x = x * 4
    // /=	x /= 2	x = x / 2
    // %=	x %= 2	x = x % 2
}

pub fn use_comparison_operators() {
    println!("Using Comparison Operators in Rust");
    // Comparison operators compare values

    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);

    // Operator	Meaning	Example
    // ==	Equal to	5 == 5 is true
    // !=	Not equal to	5 != 3 is true
    // >	Greater than	7 > 3 is true
    // <	Less than	2 < 5 is true
    // >=	Greater than or equal to	5 >= 5 is true
    // <=	Less than or equal to	3 <= 4 is true
}

pub fn use_logical_operators() {
    println!("Using Logical Operators in Rust");
    // Logical operators are used to work with booleans

    let logged_in = true;
    let is_admin = false;

    println!("Is regular use: {}", logged_in && is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);

    // Operator	Name	Description
    // &&	AND	true if both values are true
    // ||	OR	true if at least one is true
    // !	NOT	inverts the boolean value
}
