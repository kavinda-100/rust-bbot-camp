/// Command-line arguments example in Rust
/// This module demonstrates how to parse command-line arguments using std::env::args.
/// It expects a dog name and age as arguments, constructs a Dog struct, and prints its info.
///
/// # Example
/// ```sh
/// cargo run -- blacky 5
/// ```
/// Output:
/// ```
/// Dog Name: blacky, Age: 5
/// ...
/// ```
use std::env::{Args, args};

struct Dog {
    name: String,
    age: u8,
}
impl Dog {
    fn new(name: String, age: u8) -> Self {
        Dog { name, age }
    }
    fn info(&self) -> String {
        format!("Dog Name: {}, Age: {}", self.name, self.age)
    }
}

pub fn start() {
    let mut args: Args = args();

    let mut dog_name: String = String::from("");
    let mut dog_age: u8 = 0;

    let dog_name_res = args.by_ref().skip(1).take(1).next();
    let dog_age_res = args.by_ref().take(1).next();

    // Check if the arguments are provided
    match dog_name_res {
        Some(name) => {
            dog_name = name;
        }
        None => {
            eprintln!("Please provide a dog name.");
            return;
        }
    }
    match dog_age_res {
        Some(age) => {
            // Try to parse the age as an u8
            match age.parse::<u8>() {
                Ok(parsed_age) => dog_age = parsed_age,
                Err(_) => {
                    eprintln!("Invalid age provided. Please provide a valid number.");
                    return;
                }
            }
        }
        None => {
            eprintln!("Please provide a dog age.");
            return;
        }
    }

    // Construct the Dog instance
    let dog = Dog::new(dog_name, dog_age);
    println!("{}", dog.info());

    println!("{:?}", args);
}
