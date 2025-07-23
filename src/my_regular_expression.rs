use regex::Regex;

pub fn start() {
    // Print a message indicating the start of the regex test
    println!("Testing regular expressions...");

    // Call the test_regex function to perform the regex test
    test_regex();
}

fn test_regex() {
    // Define a regular expression pattern
    let pattern = "[A-Z]{1}[a-z]{2,8}";
    let input = "Kavinda";

    // Compile the regular expression
    let regex_pattern = Regex::new(pattern);
    // Check if the regex was compiled successfully
    match &regex_pattern {
        Ok(re) => {
            println!("re: {:?}", re);
        }
        Err(e) => {
            println!("Error compiling regex: {}", e);
            return;
        }
    }
    // Check if the input matches the pattern
    let res = regex_pattern.unwrap().is_match(input);
    // Print the result
    if res {
        println!("The input '{}' matches the pattern '{}'", input, pattern);
    } else {
        println!(
            "The input '{}' does not match the pattern '{}'",
            input, pattern
        );
    }
}
