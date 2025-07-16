pub fn start() {
    let closure = |x| x + 1;
    let result = closure(5);
    println!("Result: {}", result);
    
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("----------------------------------");
    println!("Original: {:?}", nums);
    println!("Doubled: {:?}", doubled);

    // with Options
    let divided = |x: i32, y: i32| {
        if y == 0 {
            println!("Cannot divide by zero");
            None
        } else {
            Some(x / y)
        }
    };

    let result = divided(10, 0);
    println!("----------------------------------");
    match result {
        Some(value) => println!("Division result: {}", value),
        None => println!("Division failed"),
    }

    // with Results
    let safe_divide = |x: i32, y: i32| {
        if y == 0 {
            Err("Cannot divide by zero")
        } else {
            Ok(x / y)
        }
    };
    let result = safe_divide(10, 0);
    println!("----------------Safe Divide with Result------------------");
    match result {
        Ok(value) => println!("Safe division result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}