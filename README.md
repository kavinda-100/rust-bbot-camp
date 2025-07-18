# Rust Boot Camp 🦀

A comprehensive Rust learning project designed to explore fundamental concepts and best practices in Rust programming.

## 📚 Project Overview

This project serves as a hands-on learning environment for Rust programming language fundamentals. It demonstrates various Rust concepts including:

- Module system and code organization
- Function definitions and usage
- Generic programming
- String handling and formatting
- Basic I/O operations

## 🚀 Getting Started

### Prerequisites

- Rust installed on your system (https://rustup.rs/)
- Basic understanding of programming concepts

### Installation

1. Clone or download this repository
2. Navigate to the project directory:
   ```bash
   cd rust-boot-camp
   ```

### Running the Project

```bash
# Compile and run the project
cargo run

# Check for compilation errors without running
cargo check

# Build the project (creates optimized executable)
cargo build --release
```

## 📁 Project Structure

```
rust-bbot-camp/
├── .gitignore              # Git ignore patterns
├── Cargo.toml              # Project configuration and dependencies
├── Cargo.lock              # Dependency lock file
├── README.md               # Project documentation
├── makefile                # Makefile for custom build commands
├── src/                    # Source code directory
│   ├── main.rs             # Main entry point
│   ├── my_functions.rs     # Custom functions module
│   ├── control_flow.rs     # Control flow examples (if/else, loops)
│   ├── user_inputs.rs      # User input handling examples
│   ├── my_args.rs          # Command-line argument parsing
│   ├── my_closures.rs      # Closures and functional programming
│   ├── my_enums.rs         # Enum definitions and pattern matching
│   ├── my_generic.rs       # Generic programming examples
│   ├── my_hash_map.rs      # HashMap collection usage
│   ├── my_iterator.rs      # Iterator trait and usage
│   ├── my_match_expression.rs # Match expressions
│   ├── my_methods.rs       # Struct methods and impl blocks
│   ├── my_option.rs        # Option<T> usage and error handling
│   ├── my_struct.rs        # Struct definitions and implementations
│   ├── my_traits.rs        # Trait definitions and usage
│   ├── my_vec.rs           # Vec collection usage
│   ├── clap/               # Command-line argument parsing with clap
│   │   ├── mod.rs
│   │   └── my_cli.rs
│   ├── file_system/        # File I/O and filesystem operations
│   │   ├── mod.rs
│   │   ├── my_file_system.rs
│   │   └── data/           # Sample data files
│   │       ├── temp_file.json
│   │       └── temp_file.txt
│   ├── tasks/              # Task modules
│   │   ├── mod.rs
│   │   └── sort_values.rs
│   └── time_module/        # Time and date handling
│       ├── mod.rs
│       ├── my_chrono.rs
│       └── my_time.rs
└── target/                 # Compiled artifacts (auto-generated)
```

### Key Rust Concepts Demonstrated

- **Modules & Submodules**: Organizing code into files and folders, including nested modules (e.g., file_system, tasks, time_module, clap)
- **Functions & Methods**: Defining and using functions, methods, and associated functions
- **Generics**: Type-safe generic programming with `<T>`
- **Traits**: Defining and implementing traits, using trait bounds
- **Enums & Pattern Matching**: Using enums and match expressions
- **Collections**: Working with Vec, HashMap, and iterators
- **Closures**: Functional programming with closures
- **Option & Result**: Error handling and optional values
- **Command-line Arguments**: Parsing with std and the clap crate
- **File I/O**: Reading and writing files, handling filesystem data
- **Time & Date**: Using chrono and std::time for date/time operations
- **Ownership & Borrowing**: Safe memory management without garbage collection

## 📖 Learning Objectives

For a comparison of Rust's trait-based polymorphism with OOP in C#, see the [TRAITSANDOOP.md](./TRAITSANDOOP.md) file.

By working with this project, you'll learn:

1. **Rust Project Structure**: How to organize a Rust project with modules and submodules
2. **Module System**: How to split code across multiple files and folders
3. **Function & Method Definitions**: Creating reusable code blocks and struct methods
4. **Generic Programming**: Writing type-safe, reusable code
5. **Trait Implementation**: Defining and using traits for polymorphism
6. **Enum & Pattern Matching**: Handling multiple data types and control flow
7. **Collections & Iterators**: Using Vec, HashMap, and iterators
8. **Error Handling**: Using Option and Result types
9. **Command-line Parsing**: Handling arguments with std and clap
10. **File I/O**: Reading and writing files
11. **Time & Date Handling**: Working with chrono and std::time
12. **Cargo Tool**: Building, running, and managing Rust projects

## 🎯 Next Steps for Learning

Consider extending this project by adding:

- [x] Error handling with `Result<T, E>` and `Option<T>`
- [x] Struct definitions and implementations
- [x] Enums and pattern matching
- [x] Collections (Vec, HashMap, etc.)
- [x] File I/O operations
- [x] External crate dependencies (e.g., chrono, clap)
- [x] More advanced generic programming
- [x] Command-line argument parsing (with std and clap)
- [x] Time and date handling
- [ ] Unit and integration tests
- [ ] Asynchronous programming (async/await)
- [ ] Error propagation and custom error types
- [ ] More advanced trait usage and lifetimes


## 📄 License

This project is for learning purposes. Feel free to use, modify.

## 🧒 Author

- Kavinda Rathnayake

### Happy Coding with **_Rust_**! 🦀 🎉
