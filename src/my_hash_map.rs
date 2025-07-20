/// HashMap example in Rust
/// This module demonstrates how to use the HashMap collection in Rust, including
/// inserting, retrieving, removing, and iterating over key-value pairs.
/// 
/// # Example
/// ```sh
/// cargo run --bin rust-bbot-camp
/// ```
/// Output:
/// ```
/// Value for key 2: Two
/// Key: 1, Value: One
/// Key: 2, Value: Two
/// Key: 3, Value: Three
/// HashMap length is: 3
/// ```


use std::collections::HashMap;

pub fn start() {
    let mut my_hash_map: HashMap<i32, String> = HashMap::new();

    // Inserting values into the HashMap
    my_hash_map.insert(1, String::from("One"));
    my_hash_map.insert(2, String::from("Two"));
    my_hash_map.insert(3, String::from("Three"));

    // remove a value
    // my_hash_map.remove(&2);

    let value = my_hash_map.get(&2);
    match value {
        Some(v) => println!("Value for key 2: {}", v),
        None => println!("Key not found"),
    }

    // print the map using a for loop
    for (key, value) in my_hash_map.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
    // Print the length of the HashMap
    println!("HashMap length is: {}", my_hash_map.len());
}
