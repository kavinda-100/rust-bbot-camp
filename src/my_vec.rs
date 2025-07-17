pub fn start(){
    // Create a vector with initial values
    let mut my_vec = vec![1, 2, 3, 4, 5];

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

    // Clear the vector
    my_vec.clear();
    println!("After clear: {:?}", my_vec);
}