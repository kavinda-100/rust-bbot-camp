pub fn start(){

    // Example of using Option with a closure
    let divided = |x: i32, y: i32| -> Option<i32> {
        if y == 0 {
            None
        } else {
            Some(x / y)
        }
    };

    // Using the closure with Some and None
    let result1 = divided(10, 2);
    let result2 = divided(10, 0);

    // println!("Result of 10 / 2: {:?}", result1); // Should print Some(5)
    // println!("Result of 10 / 0: {:?}", result2); // Should print None

    // Using Option with a match expression
    match result1 {
        Some(value) => println!("Result of 10 / 2: {}", value), // Should print 5
        None => println!("Division by zero"),
    }
    match result2 {
        Some(value) => println!("Result of 10 / 0: {}", value),
        None => println!("Division by zero"), // Should print "Division by zero"
    }
}