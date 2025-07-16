pub fn start(){
    let mut values = vec![5, 2, 9, 1, 5, 6];
    sort_values(&mut values);
    
    println!("\n--- Alternative sorting with custom swap function ---");
    let mut values2 = vec![64, 34, 25, 12, 22, 11, 90];
    sort_values_with_custom_swap(&mut values2);
}

pub fn sort_values(values: &mut Vec<i32>) {
    if values.is_empty() {
        println!("No values to sort.");
        return;
    }

    println!("Original values: {:?}", values);

    // Bubble sort implementation
    let len = values.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if values[j] > values[j + 1] {
                // Swap elements directly without the swap_fn for simplicity
                values.swap(j, j + 1);
            }
        }
    }

    println!("Sorted values: {:?}", values);
}

pub fn sort_values_with_custom_swap(values: &mut Vec<i32>) {
    if values.is_empty() {
        println!("No values to sort.");
        return;
    }

    println!("Original values: {:?}", values);

    // Bubble sort implementation using custom swap function
    let len = values.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if values[j] > values[j + 1] {
                // Use split_at_mut to avoid borrowing issues
                let (left, right) = values.split_at_mut(j + 1);
                swap_fn(&mut left[j], &mut right[0]);
            }
        }
    }

    println!("Sorted values (with custom swap): {:?}", values);
}

pub fn swap_fn(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}