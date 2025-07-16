pub fn start() {
    println!("Starting control flow example...");
    control_flow_example();
    while_loop_example();
    for_loop_example();
    match_example();
    println!("Control flow example completed.");
}

pub fn control_flow_example() {
    let number = 5;
    println!("Checking if {} is even or odd: with if else statement", number);
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}

pub fn while_loop_example() {
    let mut count = 0;
    println!("Counting from 0 to 4 using a while loop:");
    while count < 5 {
        println!("Count is: {}", count);
        count += 1;
    }
}

pub fn for_loop_example() {
    let numbers = [1, 2, 3, 4, 5];
    println!("Iterating over an array with a for loop:");
    for number in numbers.iter() {
        println!("Number: {}", number);
    }
}

pub fn match_example() {
    let value = 3;
    println!("Using match to check the value:");
    match value {
        1 => println!("Value is one"),
        2 => println!("Value is two"),
        3 => println!("Value is three"),
        _ => println!("Value is something else"),
    }
}