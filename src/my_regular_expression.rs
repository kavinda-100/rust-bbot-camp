use regex::Regex;

pub fn start() {
    // Print a message indicating the start of the regex test
    println!("Testing regular expressions...");

    // Call the test_regex function to perform the regex test
    // Define a regular expression pattern
    let pattern = "[A-Z]{1}[a-z]{2,8}"; // Pattern to match a string that starts with an uppercase letter followed by 2 to 8 lowercase letters.
    let input = "Kavinda";
    test_regex(pattern, input);

    println!("=====================");
    let pattern = "[A-Z]{1}[a-z]{2,8}"; // Pattern to match a string that starts with an uppercase letter followed by 2 to 8 lowercase letters.
    let input = "Kavinda is going to the market";
    find_regex(pattern, input);
}

fn find_regex(patten: &str, input: &str) {
    // Compile the regular expression
    let regex_pattern = Regex::new(patten);
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
    let res = regex_pattern.unwrap().find(input);
    // Print the result
    match res {
        Some(matched) => {
            println!(
                "The input '{}' contains a match for the pattern '{}': '{}'",
                input,
                patten,
                matched.as_str()
            );
        }
        None => {
            println!(
                "The input '{}' does not contain a match for the pattern '{}'",
                input, patten
            );
        }
    }
}

fn test_regex(patten: &str, input: &str) {
    // Compile the regular expression
    let regex_pattern = Regex::new(patten);
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
        println!("The input '{}' matches the pattern '{}'", input, patten);
    } else {
        println!(
            "The input '{}' does not match the pattern '{}'",
            input, patten
        );
    }
}
