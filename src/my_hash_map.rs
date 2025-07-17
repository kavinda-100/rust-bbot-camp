use std::collections::HashMap;

pub fn start(){
    
    let mut my_hash_map: HashMap<i32, String> = HashMap::new();
    
    my_hash_map.insert(1, String::from("One"));
    my_hash_map.insert(2, String::from("Two"));
    my_hash_map.insert(3, String::from("Three"));

    let value = my_hash_map.get(&2);
    match value {
        Some(v) => println!("Value for key 2: {}", v),
        None => println!("Key not found"),
    }
}