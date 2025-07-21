/// Generics example in Rust
/// This module demonstrates how to define and use generic structs and methods in Rust.
/// It shows how to create a struct that can store values of any type and how to implement
/// methods for generic types.
///
/// # Example
/// ```sh
/// cargo run --bin rust-bbot-camp
/// ```
/// Output:
/// ```
/// Point(5, 10)
/// Point(3.5, 7.2)
/// Point(x, y)
/// ```

pub fn start() {
    // Generic structs in Rust
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    // Implementing methods for the generic struct
    impl<T: std::fmt::Display> Point<T> {
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
        fn display_info(&self) {
            println!("Point({}, {})", self.x, self.y);
        }
    }

    // Using the generic struct with different types
    let int_point = Point::new(5, 10);
    let float_point = Point::new(3.5, 7.2);
    let string_point = Point::new("x", "y");
    // Displaying the points
    int_point.display_info();
    float_point.display_info();
    string_point.display_info();
}
