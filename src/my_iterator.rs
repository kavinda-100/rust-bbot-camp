pub fn start(){
    // Create a vector of integers
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers2 = vec![6, 7, 8, 9, 10];

    // Create an iterator from the vector
    let mut iter = numbers.iter();

    // Use the next method to get the first element
    if let Some(first) = iter.next() {
        println!("First element: {}", first);
    }

    // Use the for loop to iterate over the remaining elements
    for number in iter {
        println!("Next element: {}", number);
    }

    // Create a new iterator that doubles each value
    let doubled_iter = numbers.iter().map(|&x| x * 2);

    // Collect the results into a new vector
    let doubled_numbers: Vec<i32> = doubled_iter.collect();

    println!("Doubled numbers: {:?}", doubled_numbers);

    // chain multiple iterator methods
    let chained_iter = numbers.iter().chain(&numbers2);
    // print the chained iterator
    for number in chained_iter {
        println!("Chained element: {}", number);
    }
}