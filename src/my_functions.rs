pub fn start(){
    println!("Hello, world! from myFunctions.rs");

    let result = add(5, 3);
    println!("The result of adding 5 and 3 is: {}", result);

    let generic_result = generic_add(10, 20);
    println!("The result of generic adding 10 and 20 is: {}", generic_result);

    print_name("John", "Doe");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn generic_add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn print_name(f_name: &str, l_name: &str) {
    println!("First Name: {}, Last Name: {}", f_name, l_name);
}