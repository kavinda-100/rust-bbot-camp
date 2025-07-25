/// This file demonstrates how to use the `clap` crate for command line argument parsing in Rust.

pub fn start(){
    // age_validation();
    // type_validator();
    first_name();
}

// first name validation using clap
// This function defines a command line argument for the user's first name with a default value.
// It uses the `clap` crate to create the argument and set its properties.
// The argument is defined with a long name `--first-name`, a short name `-f`, a help message, a default value of "John", and a value parser that expects a `String` type.
// The function does not return any value, it simply defines the argument.
fn first_name(){
    let first_name_arg = clap::Arg::new("first-name")
        .long("first-name")
        .short('f')
        .help("Your first name")
        .default_value("John")
        .value_parser(first_name_validator);
    // Create a command and add the age argument to it
    let cmd = clap::Command::new("base").arg(first_name_arg);
    // get the results
    let matches = cmd.get_matches();
    match matches.get_one::<String>("first-name") {
        Some(first_name) => {
            println!("first name is: {}", first_name);
        },
        None => {
            println!("first name not provided, using default value: John");
        }
    }
}
// validator for the first name argument
fn first_name_validator(value: &str) -> Result<String, std::io::Error>{
    // Check if the first name starts with an uppercase letter and is followed by lowercase letters
    if value.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "First name cannot be empty"));
    }
    // regex to validate the first name
    // The regex checks that the first character is uppercase and the rest are lowercase letters
    // Example: "John" is valid, but "john" or "JOHN"
    // or "J0hn" are not valid.
    // The regex pattern is: ^[A-Z][a-z]+$
    let first_name_regex = regex::Regex::new(r"^[A-Z][a-z]+$");
    return match first_name_regex {
        Ok(re) => {
            if re.is_match(value) {
                Ok(value.to_string())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "First name must start with an uppercase letter and be followed by lowercase letters"))
            }
        },
        Err(e) => {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Regex error: {}", e)))
        }
    }
}

// validate different types of products using clap
fn type_validator(){
    let coat_types = ["winter", "summer", "rainy"];
    // argument validation using clap
    let type_arg = clap::Arg::new("coat-type")
        .long("type")
        .short('t')
        .help("Type of coat")
        .default_value("winter")
        .value_parser(coat_types);

    // Create a command and add the age argument to it
    let cmd = clap::Command::new("base").arg(type_arg);
    // get the results
    let matches = cmd.get_matches();
    // get the value of the type argument
    match matches.get_one::<String>("coat-type") {
        Some(coat_type) => {
            println!("Coat type is: {}", coat_type);
        },
        None => {
            println!("Coat type not provided, using default value: winter");
        }
    }
}

// age validation using clap
fn age_validation(){
    // argument validation using clap
    let age_arg = clap::Arg::new("age")
        .long("age")
        .short('a')
        .help("Your age")
        .default_value("18")
        .value_parser(clap::value_parser!(u8).range(18..=80));

    // Create a command and add the age argument to it
    let cmd = clap::Command::new("base").arg(age_arg);
    // get the results
    let matches = cmd.get_matches();
    // get the value of the age argument
    match matches.get_one::<u8>("age") {
        Some(age) => {
            println!("Your age is: {}", age);
        },
        None => {
            println!("Age not provided, using default value: 18");
        }
    }
}