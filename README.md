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
rust-boot-camp/
├── .gitignore              # Git ignore patterns
├── Cargo.toml              # Project configuration and dependencies
├── Cargo.lock              # Dependency lock file
├── README.md               # This file
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

- **Modules**: Organization of code into separate files
- **Public Functions**: Using `pub` keyword for external access
- **Generics**: Type-safe generic programming with `<T>`
- **Traits**: Using trait bounds (`std::ops::Add<Output = T>`)
- **String Slices**: Working with `&str` references
- **Ownership**: Safe memory management without garbage collection

## 📖 Learning Objectives

By working with this project, you'll learn:

1. **Rust Project Structure**: How to organize a Rust project
2. **Module System**: How to split code across multiple files
3. **Function Definitions**: Creating reusable code blocks
4. **Generic Programming**: Writing type-safe, reusable code
5. **String Handling**: Working with Rust's string types
6. **Cargo Tool**: Building, running, and managing Rust projects

## 🎯 Next Steps for Learning

Consider extending this project by adding:

- [x] Error handling with `Result<T, E>` and `Option<T>`
- [x] Struct definitions and implementations
- [x] Enums and pattern matching
- [x] Collections (Vec, HashMap, etc.)
- [x] File I/O operations
- [x] External crate dependencies
- [x] More advanced generic programming
- [ ] Command-line argument parsing

## 🛠️ Development Commands

```bash
# Format code according to Rust standards
cargo fmt

# Run the linter for code quality
cargo clippy

# Run tests (when implemented)
cargo test

# Generate documentation
cargo doc --open
```

## 📚 Resources for Further Learning

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings - Interactive Exercises](https://github.com/rust-lang/rustlings)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

## 📄 License

This project is for educational purposes. Feel free to use and modify as needed for your learning journey.

---

**Happy Coding with Rust! 🦀**
