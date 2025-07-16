pub fn start(){
    // Define an enum to represent different types of vehicles
    #[derive(Debug)]
    enum Vehicle {
        Car(String, u32), // Car with model and year
        Truck(String, u32), // Truck with model and year
        Motorcycle(String, u32), // Motorcycle with model and year
    }

    // Create instances of the Vehicle enum
    let my_car = Vehicle::Car(String::from("Toyota Corolla"), 2020);
    let my_truck = Vehicle::Truck(String::from("Ford F-150"), 2018);
    let my_motorcycle = Vehicle::Motorcycle(String::from("Harley-Davidson"), 2021);

    // Function to display vehicle information
    fn display_vehicle_info(vehicle: &Vehicle) {
        match vehicle {
            Vehicle::Car(model, year) => println!("Car: {} ({})", model, year),
            Vehicle::Truck(model, year) => println!("Truck: {} ({})", model, year),
            Vehicle::Motorcycle(model, year) => println!("Motorcycle: {} ({})", model, year),
        }
    }

    // Display information for each vehicle
    display_vehicle_info(&my_car);
    display_vehicle_info(&my_truck);
    display_vehicle_info(&my_motorcycle);
    
}