pub fn start(){
    // Example of using match expressions in Rust
    let number = 5;
    // Using match to handle different values of a number
    println!("--- Match Expression Example with numbers---");
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Not between 1 and 5"),
    }

    // Using match with a tuple
    let point = (0, 0);
    println!("--- Match Expression Example with tuples ---");
    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On the x-axis at {}", x),
        (0, y) => println!("On the y-axis at {}", y),
        (x, y) => println!("Point at ({}, {})", x, y),
    }

    // Using match with an enum
    #[derive(Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    let direction = Direction::Up;
    println!("--- Match Expression Example with enums ---");
    match direction {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }
}