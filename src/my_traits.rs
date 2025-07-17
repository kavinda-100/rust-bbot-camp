pub fn start() {
    trait Animal {
        fn new(name: String) -> Self
        where
            Self: Sized;
        fn name(&self) -> &String;
        fn sound(&self) -> String;
    }
    #[derive(Debug)]
    struct Dog {
        name: String,
    }
    #[derive(Debug)]
    struct Cat {
        name: String,
    }

    impl Animal for Dog {
        // Implementing the Animal trait for Dog
        fn new(name: String) -> Self {
            Dog { name }
        }
        fn name(&self) -> &String {
            &self.name
        }
        fn sound(&self) -> String {
            "Woof!".to_string()
        }
    }

    impl Animal for Cat {
        // Implementing the Animal trait for Cat
        fn new(name: String) -> Self {
            Cat { name }
        }
        fn name(&self) -> &String {
            &self.name
        }
        fn sound(&self) -> String {
            "Meow!".to_string()
        }
    }

    struct Person<T: Animal> {
        name: String,
        pet: T,
    }

    impl<T: Animal> Person<T> {
        fn new(name: String, pet: T) -> Self {
            Person { name, pet }
        }

        fn introduce(&self) -> String {
            format!(
                "Hello, my name is {} and my pet says {}",
                self.name,
                self.pet.sound()
            )
        }
        fn pet_name(&self) -> String {
            format!("My pet's name is {}", self.pet.name())
        }
    }

    let dog = Dog::new("Buddy".to_string());
    let cat = Cat::new("Whiskers".to_string());
    let person_with_dog = Person::new("Alice".to_string(), dog);
    let person_with_cat = Person::new("Bob".to_string(), cat);

    println!("=== Person with Dog ===");
    println!("{}", person_with_dog.introduce());
    println!("{}", person_with_dog.pet_name());
    println!("=== Person with Cat ===");
    println!("{}", person_with_cat.introduce());
    println!("{}", person_with_cat.pet_name());
}
