/// Methods and structs example in Rust
/// This module demonstrates how to define structs and implement methods for them in Rust.
/// It shows how to create, display, and modify struct instances using methods.
///
/// # Example
/// ```sh
/// cargo run --bin rust-bbot-camp
/// ```
/// Output:
/// ```
/// ===== Create an instance of Vehicle =====
/// ===== Display vehicle information =====
/// Vehicle Information:
/// Manufacture: BMW
/// Model: M5 CS
/// Year: 2024
/// Color: Blue
/// ===== Change the color of the vehicle =====
/// Color changed to: Red
/// ```

pub fn start() {
    // Define enum named `VehicleColor`
    #[derive(Debug)]
    enum VehicleColor {
        Red,
        Blue,
        Green,
        Black,
        White,
    }
    // define a struct named `Vehicle`
    #[derive(Debug)]
    struct Vehicle {
        manufacture: String,
        model: String,
        year: u32,
        color: VehicleColor,
    }
    // Implement methods for the `Vehicle` struct
    impl Vehicle {
        fn new(manufacture: &str, model: &str, year: u32, color: VehicleColor) -> Self {
            Vehicle {
                manufacture: String::from(manufacture),
                model: String::from(model),
                year,
                color,
            }
        }
        fn display_info(&self) {
            println!("Vehicle Information:");
            println!("Manufacture: {}", self.manufacture);
            println!("Model: {}", self.model);
            println!("Year: {}", self.year);
            println!("Color: {:?}", self.color);
        }
        fn change_color(&mut self, new_color: VehicleColor) {
            self.color = new_color;
            println!("Color changed to: {:?}", self.color);
        }
    }

    // Create an instance of `Vehicle`
    println!("===== Create an instance of Vehicle =====");
    let mut my_vehicle = Vehicle::new("BMW", "M5 CS", 2024, VehicleColor::Blue);
    // Display vehicle information
    println!("===== Display vehicle information =====");
    my_vehicle.display_info();
    // Change the color of the vehicle
    println!("===== Change the color of the vehicle =====");
    my_vehicle.change_color(VehicleColor::Red);
}
