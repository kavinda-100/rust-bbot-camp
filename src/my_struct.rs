pub fn start() {
    // Define a struct named `Person`
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    // Create an instance of `Person`
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Print the person instance
    println!("===== Print the person instance =====");
    println!("Immutable struct : {:#?}", person);

    // Access fields of the struct
    println!("===== Access fields of the struct =====");
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);

    // Create a mutable instance of `Person`
    let mut mutable_person = Person {
        name: String::from("Bob"),
        age: 25,
    };

    // Modify the age field
    mutable_person.age += 1;

    // Print the modified person instance
    println!("===== Print the modified person instance =====");
    println!("Mutable struct : {:#?}", mutable_person);
}
