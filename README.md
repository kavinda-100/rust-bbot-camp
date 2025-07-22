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
├── TRAITSANDOOP.md         # Rust vs C# traits and OOP comparison
├── makefile                # Makefile for custom build commands
├── src/                    # Source code directory
│   ├── main.rs                 # Main entry point
│   ├── control_flow.rs         # Control flow examples (if/else, loops)
│   ├── my_args.rs              # Command-line argument parsing
│   ├── my_closures.rs          # Closures and functional programming
│   ├── my_enums.rs             # Enum definitions and pattern matching
│   ├── my_functions.rs         # Custom functions module
│   ├── my_generic.rs           # Generic programming examples
│   ├── my_hash_map.rs          # HashMap collection usage
│   ├── my_http.rs              # HTTP requests and networking
│   ├── my_iterator.rs          # Iterator trait and usage
│   ├── my_match_expression.rs  # Match expressions
│   ├── my_methods.rs           # Struct methods and impl blocks
│   ├── my_option.rs            # Option<T> usage and error handling
│   ├── my_option_overload.rs   # Advanced Option usage
│   ├── my_serde.rs             # Serialization/deserialization with serde
│   ├── my_struct.rs            # Struct definitions and implementations
│   ├── my_traits.rs            # Trait definitions and usage
│   ├── my_vec.rs               # Vec collection usage
│   ├── user_inputs.rs          # User input handling examples
│   ├── asyncs/                 # Async programming modules
│   │   ├── mod.rs
│   │   ├── my_smol.rs
│   │   └── my_tokio.rs
│   ├── clap/                   # Command-line argument parsing with clap
│   │   ├── mod.rs
│   │   └── my_cli.rs
│   ├── external_process/       # Running external processes
│   │   ├── mod.rs
│   │   ├── my_external_proccess.rs
│   │   └── my_folder/
│   ├── file_system/            # File I/O and filesystem operations
│   │   ├── mod.rs
│   │   ├── my_file_system.rs
│   │   └── data/               # Sample data files
│   ├── tasks/                  # Task modules
│   │   ├── mod.rs
│   │   └── sort_values.rs
│   └── time_module/            # Time and date handling
│       ├── mod.rs
│       ├── my_chrono.rs
│       └── my_time.rs
└── target/                 # Compiled artifacts (auto-generated)
```

## 🦀 Key Rust Concepts Demonstrated

- **Modules & Submodules**: Organizing code into files and folders, including nested modules (e.g., asyncs, file_system, tasks, time_module, clap, external_process)
- **Functions & Methods**: Defining and using functions, struct methods, and associated functions
- **Generics**: Type-safe generic programming with `<T>` (see my_generic.rs)
- **Traits**: Defining and implementing traits, using trait bounds (see my_traits.rs)
- **Enums & Pattern Matching**: Using enums and match expressions (see my_enums.rs, my_match_expression.rs)
- **Collections**: Working with Vec, HashMap, and iterators (see my_vec.rs, my_hash_map.rs, my_iterator.rs)
- **Closures**: Functional programming with closures (see my_closures.rs)
- **Option & Result**: Error handling and optional values (see my_option.rs, my_option_overload.rs)
- **Command-line Arguments**: Parsing with std and the clap crate (see my_args.rs, clap/)
- **File I/O**: Reading and writing files, handling filesystem data (see file_system/)
- **Time & Date**: Using chrono and std::time for date/time operations (see time_module/)
- **Ownership & Borrowing**: Safe memory management without garbage collection
- **Serialization/Deserialization**: Using serde for working with JSON and other formats (see my_serde.rs)
- **HTTP Requests**: Making HTTP requests and handling responses (see my_http.rs)
- **Async Programming**: Using async/await with Smol and Tokio (see asyncs/)
- **External Processes**: Running and managing external processes (see external_process/)

## 📖 Learning Objectives

➡️ For a comparison of Rust's trait-based polymorphism with OOP in C#, see the [TRAITSANDOOP.md](./TRAITSANDOOP.md) file.

By working with this project, you'll learn:

1. **Project Structure & Modules**: How to organize a Rust project with modules, submodules, and folders (see src/)
2. **Function & Method Definitions**: Creating reusable code blocks and struct methods (see my_functions.rs, my_methods.rs)
3. **Generic Programming**: Writing type-safe, reusable code (see my_generic.rs)
4. **Trait Implementation**: Defining and using traits for polymorphism (see my_traits.rs)
5. **Enum & Pattern Matching**: Handling multiple data types and control flow (see my_enums.rs, my_match_expression.rs)
6. **Collections & Iterators**: Using Vec, HashMap, and iterators (see my_vec.rs, my_hash_map.rs, my_iterator.rs)
7. **Closures & Functional Programming**: Using closures for concise code (see my_closures.rs)
8. **Error Handling**: Using Option and Result types (see my_option.rs, my_option_overload.rs)
9. **Command-line Parsing**: Handling arguments with std and clap (see my_args.rs, clap/)
10. **File I/O**: Reading and writing files (see file_system/)
11. **Time & Date Handling**: Working with chrono and std::time (see time_module/)
12. **Serialization/Deserialization**: Working with serde for JSON and other formats (see my_serde.rs)
13. **HTTP Networking**: Making HTTP requests and processing responses (see my_http.rs)
14. **Async Programming**: Using async/await with Smol and Tokio (see asyncs/)
15. **External Processes**: Running and managing external processes (see external_process/)
16. **Cargo Tool**: Building, running, and managing Rust projects

## 🎯 Next Steps for Learning

Consider extending this project by adding:

- [x] Error handling with `Result<T, E>` and `Option<T>` (see my_option.rs, my_option_overload.rs)
- [x] Struct definitions and implementations (see my_struct.rs, my_methods.rs)
- [x] Enums and pattern matching (see my_enums.rs, my_match_expression.rs)
- [x] Collections (Vec, HashMap, etc.) (see my_vec.rs, my_hash_map.rs)
- [x] File I/O operations (see file_system/)
- [x] External crate dependencies (e.g., chrono, clap, serde)
- [x] More advanced generic programming (see my_generic.rs)
- [x] Command-line argument parsing (with std and clap) (see my_args.rs, clap/)
- [x] Time and date handling (see time_module/)
- [x] Serialization/deserialization (serde, JSON) (see my_serde.rs)
- [x] HTTP requests and networking (see my_http.rs)
- [x] Async programming (async/await, Smol, Tokio) (see asyncs/)
- [x] External process management (see external_process/)
- [ ] Unit and integration tests (add tests/ folder and sample tests)
- [ ] Error propagation and custom error types (create custom error modules)
- [ ] More advanced trait usage and lifetimes (expand my_traits.rs)
- [ ] Benchmarking and performance profiling (use criterion crate)
- [ ] Documentation with rustdoc (add doc comments)

Explore the [TRAITSANDOOP.md](./TRAITSANDOOP.md) file for a deeper understanding of Rust traits vs C# OOP.

## 📄 License

This project is for learning purposes. Feel free to use, modify.

## 🧒 Author

- Kavinda Rathnayake

### Happy Coding with **_Rust_**! 🦀 🎉
