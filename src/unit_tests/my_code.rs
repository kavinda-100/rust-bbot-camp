pub fn start() {
    // This function can be used to initialize or run tests
    println!("Starting unit tests...");

    // Example usage of the get_full_name function
    let first_name = "Alice";
    let last_name = "Smith";
    let full_name = get_full_name(first_name, last_name);
    println!("Full name: {}", full_name);
}
pub fn get_full_name(first_name: &str, last_name: &str) -> String {
    // Check if first name or last name is empty
    if first_name.is_empty() || last_name.is_empty() {
        panic!("First name and last name cannot be empty");
    }
    else if first_name.contains(&['*', '%', '$', '#', '`', '^', '+']) || last_name.contains(&['*', '%', '$', '#', '`', '^', '+']) {
        panic!("First name or last name contains special characters");
    }
    // Return the full name by concatenating first and last names
    format!("{} {}", first_name, last_name)
}

pub fn get_add_numbers(a: i32, b: i32) -> i32 {
    // if a or b is negative, panic with a message
    if a < 0 || b < 0 {
        panic!("Both numbers must be non-negative");
    }
    // Return the sum of two numbers
    a + b
}

pub fn get_divide_numbers(a: i32, b: i32) -> Result<f64, String> {
    // if b is zero, panic with a message
    if b == 0 {
        return Err("Division by zero is not allowed".to_string());
    }
    // Return the division of two numbers
    Ok(a as f64 / b as f64)
}