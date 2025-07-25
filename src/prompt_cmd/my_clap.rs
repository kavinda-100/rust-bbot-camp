/// This file demonstrates how to use the `clap` crate for command line argument parsing in Rust.

pub fn start(){
    // age_validation();
    type_validator();
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