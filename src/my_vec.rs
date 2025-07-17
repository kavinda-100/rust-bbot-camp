pub fn start() {
    // Create a vector with initial values
    let mut my_vec: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Print the original vector
    println!("Original vector: {:?}", my_vec);

    // Add an element to the end of the vector
    my_vec.push(6);
    println!("After push: {:?}", my_vec);

    // Remove the last element from the vector
    my_vec.pop();
    println!("After pop: {:?}", my_vec);

    // Access an element by index
    if let Some(value) = my_vec.get(2) {
        println!("Element at index 2: {}", value);
    } else {
        println!("No element at index 2");
    }

    // Iterate over the elements in the vector
    for (index, value) in my_vec.iter().enumerate() {
        println!("Element at index {}: {}", index, value);
    }

    for value in my_vec.iter() {
        println!("Value: {}", value + 1);
    }

    // Clear the vector
    my_vec.clear();
    println!("After clear: {:?}", my_vec);

    println!("== Append to the Vec ===");
    append_to_vector();
    retain_vec();
}

fn retain_vec() {
    #[derive(Debug)]
    struct Car {
        name: String,
        year: u32,
    }

    let mut cars: Vec<Car> = Vec::new();

    for i in 1..20 {
        if (i % 2 == 0) {
            let car = Car {
                name: String::from("BMW"),
                year: 2024,
            };
            cars.push(car);
        } else {
            let car = Car {
                name: String::from("Toyota"),
                year: 2020,
            };

            cars.push(car);
        }
    }

    let keep = |x: &Car| {
        return if (x.name.contains("BMW")) {
            true
        } else {
            false
        };
    };

    cars.retain(keep);
    println!(" === filter vec ===");
    println!("{:?}", cars);
}

fn append_to_vector() {
    #[derive(Debug)]
    struct Car {
        name: String,
        year: u32,
    }

    // Create a vector to hold Car objects
    let mut cars_one: Vec<Car> = Vec::new();

    for _ in 0..5 {
        // Create a new Car object
        let car = Car {
            name: String::from("Toyota"),
            year: 2020,
        };

        // Append the Car object to the vector
        cars_one.push(car);
    }
    // Send the vector to another function
    let mut cars_two: Vec<Car> = Vec::new();
    for _ in 0..5 {
        // Create a new Car object
        let car = Car {
            name: String::from("BMW"),
            year: 2024,
        };

        // Append the Car object to the vector
        cars_two.push(car);
    }

    // append the car_two the car_one
    cars_one.append(&mut cars_two);

    println!("{:?}", cars_one);
}
