use clap::{Arg, command};

pub fn start() {
    let match_result = command!()
        .about("A simple CLI application")
        .version("1.0")
        .arg(
            Arg::new("first_name")
                .short('f')
                .long("first_name")
                .help("First name"),
        )
        .arg(
            Arg::new("last_name")
                .short('l')
                .long("last_name")
                .help("Last name"),
        )
        .arg(Arg::new("age").short('a').long("age").help("Age"))
        .arg(Arg::new("city").short('c').long("city").help("City"))
        .get_matches();

    // Extracting values from the matches
    let first_name = match_result.get_one::<String>("first_name");
    let last_name = match_result.get_one::<String>("last_name");
    let age = match_result.get_one::<String>("age");
    let city = match_result.get_one::<String>("city");
    // Printing the values
    println!("First Name: {:?}", first_name);
    println!("Last Name: {:?}", last_name);
    println!("Age: {:?}", age);
    println!("City: {:?}", city);
}
