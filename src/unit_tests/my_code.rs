pub mod my_code {
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
        format!("{} {}", first_name, last_name)
    }
}