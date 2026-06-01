pub fn structures() {
    println!("Structs in Rust");

    // Create a struct
    struct Person {
        name: String,
        age: u32,
        can_vote: bool,
    }

    // Create an object
    let user = Person {
        name: String::from("John"),
        age: 26,
        can_vote: true,
    };

    println!(
        "Name: {}, Age: {}, Can Vote: {}",
        user.name, user.age, user.can_vote
    );

    // Fields
    // To change a field, you must make the object mutable.

    let mut admin = Person {
        name: String::from("Jane"),
        age: 22,
        can_vote: true,
    };

    admin.name = String::from("Jabu");

    println!(
        "Name: {}, Age: {}, Can Vote: {}",
        admin.name, admin.age, admin.can_vote
    );

    // Structs:
    // Group related data.
    // Make code easier to read and maintainable.
    // Create realworld examples.
}
