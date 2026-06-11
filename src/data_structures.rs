use std::collections::HashMap;

pub fn simple_data_structures() {
    println!("Data Structures in Rust");
    // Data Structures are used to store and organize values.

    // Arrays
    // - is a fixed-size list
    // - values of the same type
    // - can not grow size
    let fruits = ["apple", "pear", "orange", "banana"];
    println!("Last fruit: {}", fruits[3]);

    // Vectors
    // - a resizable array
    // - can grow or shrink
    let mut veggies = vec!["onion", "pumpkin", "lettuce"];
    veggies.push("potato");
    println!("Last vegetables: {}", veggies[3]);

    // Tuples
    // - can hold multiple values of different types
    // - useful when grouping data
    // - can not grow size
    let person = ("John", 30, true);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Can Vote: {}", person.2);

    // Hashmaps
    // - stores key-value pairs
    // - can look up values with key
    // - must be imported from the standard library
    // - can grow
    // use std::collections::HashMap;
    let mut capital_cities = HashMap::new();
    capital_cities.insert("South Africa", "Cape Town");
    capital_cities.insert("Japan", "Tokyo");
    capital_cities.insert("France", "Paris");

    println!("Capital city of France is: {}", capital_cities["France"]);
}
