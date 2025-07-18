# Traits and OOP: Rust vs C#

This document compares Rust's approach to polymorphism and code reuse (using traits, structs, and impl blocks) with traditional Object-Oriented Programming (OOP) in C# (using classes and interfaces). The examples are based on the `my_traits.rs` file in this project.

---

## Rust: Traits, Structs, and impl

```rust
trait Animal {
    fn new(name: String) -> Self where Self: Sized;
    fn name(&self) -> &String;
    fn sound(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
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
        format!("Hello, my name is {} and my pet says {}", self.name, self.pet.sound())
    }
}
```

### Key Points in Rust
- **Traits** are like interfaces in OOP, defining required methods.
- **Structs** are data containers (like classes without methods).
- **impl blocks** provide method implementations for structs and traits.
- **Generics** allow for type-safe code reuse (e.g., `Person<T: Animal>`).
- No inheritance; composition and trait bounds are used instead.

---

## C#: Classes and Interfaces

```csharp
public interface IAnimal {
    string Name { get; }
    string Sound();
}

public class Dog : IAnimal {
    public string Name { get; private set; }
    public Dog(string name) { Name = name; }
    public string Sound() => "Woof!";
}

public class Cat : IAnimal {
    public string Name { get; private set; }
    public Cat(string name) { Name = name; }
    public string Sound() => "Meow!";
}

public class Person<T> where T : IAnimal {
    public string Name { get; private set; }
    public T Pet { get; private set; }
    public Person(string name, T pet) {
        Name = name;
        Pet = pet;
    }
    public string Introduce() => $"Hello, my name is {Name} and my pet says {Pet.Sound()}";
}
```

### Key Points in C#
- **Interfaces** define required methods/properties (like traits).
- **Classes** combine data and behavior, and can implement interfaces.
- **Inheritance** is available (not shown here), but composition is also common.
- **Generics** allow for type-safe code reuse (e.g., `Person<T> where T : IAnimal`).

---

## Comparison Table

| Concept         | Rust                      | C#                        |
|-----------------|--------------------------|---------------------------|
| Interface       | Trait                     | Interface                 |
| Data Structure  | Struct                    | Class                     |
| Implementation  | impl block                | Class implements interface|
| Inheritance     | Not supported (composition)| Supported                 |
| Generics        | Yes (trait bounds)        | Yes (constraints)         |
| Method Syntax   | `fn` in impl              | Methods in class          |

---

## Summary
- Rust uses traits and structs for polymorphism and code reuse, focusing on composition over inheritance.
- C# uses interfaces and classes, supporting both composition and inheritance.
- Both languages support generics and type-safe abstractions.

---

**See `src/my_traits.rs` for the full Rust example.**

