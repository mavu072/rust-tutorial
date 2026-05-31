pub fn borrowing_and_references() {
    println!("Borrowing and References in Rust");
    // Sometimes you want to use a value without taking ownership of it.
    // Rust lets you do this using a reference - this is called borrowing.

    // A reference lets you look at a value without owning it. You create a reference using the & symbol.
    let a = String::from("Hello");
    let b = &a;

    println!("a = {}", a);
    println!("b = {}", b);

    // Mutable References
    // To change a valaue through a reference, you need to make the reference mut:
    let mut name = String::from("John");
    let name_ref = &mut name;
    name_ref.push_str(" Doe");

    // println!("{}", name); // This errors because Rust does not allowing using an owner while an active mutable reference exists.
    println!("{}", name_ref); // John Doe
    println!("{}", name); // Using owner after last use of reference should work.
    // Observed that mutating the reference updated the original/owner.
}
