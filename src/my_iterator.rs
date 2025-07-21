/// Iterators example in Rust
/// This module demonstrates how to use iterators in Rust, including creating iterators,
/// using iterator methods like map, chain, zip, and fold, and collecting results.
///
/// # Example
/// ```sh
/// cargo run --bin rust-bbot-camp
/// ```
/// Output:
/// ```
/// First element: 1
/// Next element: 2
/// ...
/// Chained element: 6
/// ...
/// Sum of numbers: 15
/// ```

pub fn start() {
    // Create a vector of integers
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers2 = vec![6, 7, 8, 9, 10];

    // Create an iterator from the vector
    let mut iter = numbers.iter();

    // Use the next method to get the first element
    if let Some(first) = iter.next() {
        println!("First element: {}", first);
    }

    // Use the for loop to iterate over the remaining elements
    for number in iter {
        println!("Next element: {}", number);
    }

    // Create a new iterator that doubles each value
    let doubled_iter = numbers.iter().map(|&x| x * 2);

    // Collect the results into a new vector
    let doubled_numbers: Vec<i32> = doubled_iter.collect();

    println!("Doubled numbers: {:?}", doubled_numbers);

    // chain multiple iterator methods
    let chained_iter = numbers.iter().chain(&numbers2);
    // print the chained iterator
    for number in chained_iter {
        println!("Chained element: {}", number);
    }

    // create a tuple
    let first_names = vec!["Alice", "Bob", "Charlie"];
    let last_names = vec!["Smith", "Johnson", "Williams"];
    // create an iterator that zips the first and last names
    let full_names = first_names.iter().zip(last_names);
    // print the full names using for each loop
    full_names.for_each(|(first_name, last_name)| {
        println!("First: {} Last: {}", first_name, last_name);
    });

    // fold method (similar to reduce in javascript)
    let foods = vec!["Pizza", "Burger", "Pasta"];
    // calculate the total length of all food names
    let foods_count = foods.iter().fold(0u32, |mut acc, &food| {
        acc += food.len() as u32;
        acc
    });
    println!("Total length of food names: {}", foods_count);
    // more example
    let numbers = vec![1, 2, 3, 4, 5];
    // calculate the sum of all numbers
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum of numbers: {}", sum);
}
