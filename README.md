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
├── Cargo.toml              # Project configuration and dependencies
├── Cargo.lock              # Dependency lock file
├── README.md               # Project documentation
├── TRAITSANDOOP.md         # Rust vs C# traits and OOP comparison
├── makefile                # Custom build commands
├── promts.txt              # Learning prompts and notes
└── src/                    # Source code directory
    ├── main.rs                     # Main entry point with module orchestration
    │
    ├── 🔧 Core Language Features
    ├── control_flow.rs             # Control flow (if/else, loops, while)
    ├── my_functions.rs             # Function definitions and usage
    ├── my_generic.rs               # Generic programming and type parameters
    ├── my_struct.rs                # Struct definitions and basic usage
    ├── my_methods.rs               # Struct methods and impl blocks
    ├── my_traits.rs                # Trait definitions and implementations
    ├── my_enums.rs                 # Enum definitions and variants
    ├── my_match_expression.rs      # Pattern matching with match
    ├── my_closures.rs              # Closures and functional programming
    ├── my_option.rs                # Option<T> for nullable values
    ├── my_option_overload.rs       # Advanced Option usage patterns
    ├── my_regular_expression.rs    # Regex pattern matching
    │
    ├── 📦 Collections & Data Structures
    ├── my_vec.rs                   # Vector collection usage
    ├── my_hash_map.rs              # HashMap key-value storage
    ├── my_iterator.rs              # Iterator trait and lazy evaluation
    │
    ├── 🌐 External Integrations
    ├── my_args.rs                  # Command-line argument parsing (std)
    ├── my_http.rs                  # HTTP requests with reqwest
    ├── my_serde.rs                 # JSON serialization/deserialization
    │
    ├── 🕒 Time & Date Modules
    ├── time_module/
    │   ├── mod.rs                  # Time module declarations
    │   ├── my_time.rs              # Standard library time operations
    │   └── my_chrono.rs            # Chrono crate for advanced date/time
    │
    ├── 📁 File & Data Operations
    ├── file_system/
    │   ├── mod.rs                  # File system module declarations
    │   ├── my_file_system.rs       # File I/O and directory operations
    │   └── data/                   # Sample data files for testing
    ├── csv/
    │   ├── mod.rs                  # CSV module declarations
    │   ├── my_csv.rs               # CSV reading and writing operations
    │   └── vehicle.csv             # Sample CSV data file
    │
    ├── 🎯 User Interface & CLI
    ├── prompt_cmd/
    │   ├── mod.rs                  # User input module declarations
    │   ├── user_inputs.rs          # Basic user input handling
    │   ├── my_inquire.rs           # Interactive prompts with inquire
    │   └── my_clap.rs              # Advanced CLI with clap crate
    ├── clap/
    │   ├── mod.rs                  # Clap module declarations
    │   └── my_cli.rs               # Command-line interface examples
    │
    ├── ⚡ Asynchronous Programming
    ├── asyncs/
    │   ├── mod.rs                  # Async module declarations
    │   ├── my_smol.rs              # Async programming with smol runtime
    │   └── my_tokio.rs             # Async programming with tokio runtime
    │
    ├── 🔄 Process & Task Management
    ├── external_process/
    │   ├── mod.rs                  # External process module declarations
    │   ├── my_external_proccess.rs # Running external commands
    │   └── my_folder/              # Sample folder for process operations
    ├── tasks/
    │   ├── mod.rs                  # Task module declarations
    │   └── sort_values.rs          # Sorting algorithms and data manipulation
    │
    └── 🧪 Testing & Quality Assurance
        └── unit_tests/
            ├── mod.rs              # Unit test module declarations
            ├── my_code.rs          # Code to be tested
            └── my_test.rs          # Unit test implementations
```

## 🦀 Key Rust Concepts Demonstrated

### 🔧 Core Language Features
- **Ownership & Borrowing**: Safe memory management without garbage collection
- **Control Flow**: if/else statements, loops (for, while, loop) with break/continue (see control_flow.rs)
- **Functions & Methods**: Function definitions, struct methods, associated functions, and impl blocks (see my_functions.rs, my_methods.rs)
- **Generics & Type Parameters**: Type-safe generic programming with `<T>`, bounds, and constraints (see my_generic.rs)
- **Traits**: Defining custom traits, implementing standard traits, trait bounds, and polymorphism (see my_traits.rs)
- **Structs**: Data structure definitions, field access, destructuring (see my_struct.rs)
- **Enums & Pattern Matching**: Enum variants, match expressions, if let patterns (see my_enums.rs, my_match_expression.rs)
- **Closures**: Anonymous functions, capturing environment, functional programming patterns (see my_closures.rs)
- **Regular Expressions**: Pattern matching with regex crate for text processing (see my_regular_expression.rs)

### 📦 Collections & Data Structures
- **Vec<T>**: Dynamic arrays, push/pop, iteration, slicing (see my_vec.rs)
- **HashMap<K, V>**: Key-value storage, insertion, lookup, iteration (see my_hash_map.rs)
- **Iterators**: Lazy evaluation, map/filter/collect, iterator adapters and consumers (see my_iterator.rs)

### 🛡️ Error Handling & Safety
- **Option<T>**: Handling nullable values safely, Some/None patterns (see my_option.rs)
- **Advanced Option Usage**: Chaining, unwrap variants, error propagation (see my_option_overload.rs)
- **Result<T, E>**: Error handling with Ok/Err, ? operator for error propagation

### 🌐 External Integrations & I/O
- **Command-line Arguments**: Parsing with std::env and advanced CLI with clap crate (see my_args.rs, clap/)
- **HTTP Networking**: Making HTTP requests with reqwest, handling responses (see my_http.rs)
- **JSON Serialization**: Working with serde for serialization/deserialization (see my_serde.rs)
- **File System Operations**: Reading/writing files, directory traversal (see file_system/)
- **CSV Processing**: Reading and writing CSV files with csv crate (see csv/)

### 🕒 Time & Date Management
- **Standard Time**: Using std::time for basic time operations (see time_module/my_time.rs)
- **Advanced Date/Time**: Using chrono crate for parsing, formatting, timezones (see time_module/my_chrono.rs)

### 🎯 User Interface & Interaction
- **Basic Input**: Reading user input with std::io (see prompt_cmd/user_inputs.rs)
- **Interactive Prompts**: Rich CLI interactions with inquire crate (see prompt_cmd/my_inquire.rs)
- **Advanced CLI**: Subcommands, argument validation with clap (see prompt_cmd/my_clap.rs, clap/)

### ⚡ Concurrency & Asynchronous Programming
- **Async/Await**: Asynchronous programming patterns with async functions
- **Smol Runtime**: Lightweight async runtime for concurrent operations (see asyncs/my_smol.rs)
- **Tokio Runtime**: Full-featured async runtime for network and I/O operations (see asyncs/my_tokio.rs)

### 🔄 Process & Task Management
- **External Processes**: Running system commands, process spawning and management (see external_process/)
- **Data Processing Tasks**: Sorting algorithms, data manipulation patterns (see tasks/)

### 🧪 Testing & Quality Assurance
- **Unit Testing**: Writing and organizing unit tests with #[test] (see unit_tests/)
- **Module Organization**: Separating code and tests for maintainability

### 📦 Package Management & Build System
- **Cargo**: Dependency management, build configuration, feature flags
- **External Crates**: Integration with ecosystem crates (chrono, clap, serde, reqwest, etc.)
- **Module System**: pub/private visibility, mod declarations, use statements

## 📖 Learning Objectives

➡️ For a comparison of Rust's trait-based polymorphism with OOP in C#, see the [TRAITSANDOOP.md](./TRAITSANDOOP.md) file.

By working with this project, you'll master these essential Rust concepts:

### 🎯 Foundational Skills
1. **Project Structure & Module System**: 
   - Organizing code with `mod` declarations and `use` statements
   - Creating nested modules and managing visibility with `pub`
   - Understanding the difference between library crates and binary crates
   - *Practice with*: All module files, especially nested ones like `time_module/`, `asyncs/`, etc.

2. **Ownership, Borrowing & Lifetimes**:
   - Understanding move semantics and borrowing rules
   - Working with references (`&` and `&mut`)
   - Avoiding common ownership pitfalls
   - *Practice with*: All modules, especially `my_struct.rs`, `my_methods.rs`

3. **Functions & Control Flow**:
   - Defining functions with parameters and return types
   - Control structures (if/else, loops, match)
   - Early returns and error propagation
   - *Practice with*: `my_functions.rs`, `control_flow.rs`

### 🔧 Core Language Features
4. **Data Types & Structures**:
   - Creating and using structs with named fields
   - Implementing methods with `impl` blocks
   - Understanding tuple structs and unit structs
   - *Practice with*: `my_struct.rs`, `my_methods.rs`

5. **Enums & Pattern Matching**:
   - Defining enums with variants and associated data
   - Exhaustive pattern matching with `match`
   - Using `if let` for specific pattern matching
   - *Practice with*: `my_enums.rs`, `my_match_expression.rs`

6. **Generic Programming**:
   - Writing generic functions and structs with type parameters
   - Understanding trait bounds and where clauses
   - Working with generic collections
   - *Practice with*: `my_generic.rs`, collections modules

7. **Traits & Polymorphism**:
   - Defining custom traits and implementing them for types
   - Using standard library traits (Clone, Debug, Display, etc.)
   - Trait objects and dynamic dispatch
   - *Practice with*: `my_traits.rs`, compare with `TRAITSANDOOP.md`

### 🛡️ Error Handling & Safety
8. **Option<T> for Null Safety**:
   - Handling optional values with Some/None
   - Chaining operations with map, and_then, or_else
   - Converting between Option and Result
   - *Practice with*: `my_option.rs`, `my_option_overload.rs`

9. **Result<T, E> for Error Handling**:
   - Propagating errors with the `?` operator
   - Handling multiple error types
   - Creating custom error types
   - *Practice with*: File I/O, HTTP requests, CSV parsing modules

### 📦 Collections & Functional Programming
10. **Collections Mastery**:
    - Vector operations: push, pop, indexing, slicing
    - HashMap operations: insert, get, iteration
    - Choosing the right collection for the task
    - *Practice with*: `my_vec.rs`, `my_hash_map.rs`

11. **Iterator Patterns**:
    - Understanding lazy evaluation
    - Using iterator adapters (map, filter, take, etc.)
    - Consuming iterators (collect, fold, for_each, etc.)
    - *Practice with*: `my_iterator.rs`, `my_closures.rs`

12. **Closures & Functional Programming**:
    - Creating closures with different capture modes
    - Using closures with iterator methods
    - Understanding Fn, FnMut, and FnOnce traits
    - *Practice with*: `my_closures.rs`

### 🌐 Real-World Integration
13. **External Crate Integration**:
    - Managing dependencies with Cargo.toml
    - Working with popular crates (serde, chrono, clap, reqwest)
    - Understanding semantic versioning
    - *Practice with*: All external integration modules

14. **File System & I/O Operations**:
    - Reading and writing files safely
    - Working with paths and directories
    - Handling I/O errors gracefully
    - *Practice with*: `file_system/`, `csv/`

15. **Network Programming & APIs**:
    - Making HTTP requests with reqwest
    - Handling JSON with serde
    - Async programming patterns
    - *Practice with*: `my_http.rs`, `my_serde.rs`, `asyncs/`

16. **Command-Line Interface Development**:
    - Parsing arguments with std::env
    - Building robust CLIs with clap
    - Creating interactive prompts
    - *Practice with*: `my_args.rs`, `clap/`, `prompt_cmd/`

### ⚡ Advanced Topics
17. **Asynchronous Programming**:
    - Understanding async/await syntax
    - Working with different async runtimes (Tokio, Smol)
    - Handling concurrent operations safely
    - *Practice with*: `asyncs/my_tokio.rs`, `asyncs/my_smol.rs`

18. **Process Management & System Integration**:
    - Running external commands
    - Handling process output and errors
    - Working with environment variables
    - *Practice with*: `external_process/`

19. **Testing & Code Quality**:
    - Writing unit tests with #[test]
    - Organizing test code
    - Using assert macros effectively
    - *Practice with*: `unit_tests/`

20. **Regular Expressions & Text Processing**:
    - Creating and using regex patterns
    - Capturing groups and replacements
    - Performance considerations with regex
    - *Practice with*: `my_regular_expression.rs`

## 🎯 Next Steps for Learning

### ✅ Completed Features
- [x] **Core Language Fundamentals**
  - Error handling with `Result<T, E>` and `Option<T>` (see my_option.rs, my_option_overload.rs)
  - Struct definitions and implementations (see my_struct.rs, my_methods.rs)
  - Enums and pattern matching (see my_enums.rs, my_match_expression.rs)
  - Generic programming (see my_generic.rs)
  - Trait definitions and implementations (see my_traits.rs)

- [x] **Collections & Data Structures**
  - Vec, HashMap, and iterators (see my_vec.rs, my_hash_map.rs, my_iterator.rs)
  - Closures and functional programming (see my_closures.rs)

- [x] **External Integrations**
  - Command-line argument parsing with std and clap (see my_args.rs, clap/)
  - File I/O operations (see file_system/)
  - Time and date handling (see time_module/)
  - JSON serialization/deserialization with serde (see my_serde.rs)
  - HTTP requests and networking (see my_http.rs)
  - CSV file processing (see csv/)

- [x] **Advanced Features**
  - Async programming with Smol and Tokio runtimes (see asyncs/)
  - External process management (see external_process/)
  - Regular expression processing (see my_regular_expression.rs)
  - Interactive CLI development (see prompt_cmd/)

### 🚀 Intermediate Enhancements
- [ ] **Enhanced Error Handling**
  - Create custom error types with `thiserror` crate
  - Implement error conversion with `From` trait
  - Add comprehensive error propagation examples
  - *Suggested files*: `src/errors/mod.rs`, `src/errors/custom_errors.rs`

- [ ] **Advanced Testing**
  - Expand unit tests with comprehensive test coverage
  - Add integration tests in `tests/` directory
  - Property-based testing with `proptest` crate
  - Benchmarking with `criterion` crate
  - *Suggested files*: `tests/integration_tests.rs`, `benches/benchmarks.rs`

- [ ] **Memory Management & Performance**
  - Reference counting with `Rc<T>` and `Arc<T>`
  - Interior mutability with `RefCell<T>` and `Mutex<T>`
  - Zero-copy string handling with `Cow<str>`
  - Memory profiling and optimization techniques
  - *Suggested files*: `src/memory/`, `src/performance/`

- [ ] **Advanced Trait Usage**
  - Trait objects and dynamic dispatch
  - Associated types vs generic parameters
  - Higher-ranked trait bounds (HRTB)
  - Blanket implementations
  - *Suggested files*: `src/advanced_traits.rs`

### 🎓 Advanced Learning Paths

#### 🌐 Systems Programming Track
- [ ] **Low-Level Programming**
  - Unsafe Rust for performance-critical code
  - FFI (Foreign Function Interface) with C libraries
  - Memory layout and representation
  - SIMD operations for parallel computation

- [ ] **Network Programming**
  - TCP/UDP socket programming
  - WebSocket implementations
  - Custom protocol development
  - Network security and TLS

- [ ] **Concurrency & Parallelism**
  - Thread pools and work stealing
  - Message passing with channels
  - Lock-free data structures
  - Parallel iterators with `rayon`

#### 🖥️ Application Development Track
- [ ] **GUI Development**
  - Desktop applications with `egui` or `tauri`
  - Cross-platform development strategies
  - Event-driven programming patterns

- [ ] **Web Development**
  - Web servers with `axum` or `warp`
  - REST API development
  - WebAssembly (WASM) compilation
  - Database integration with `sqlx`

- [ ] **CLI Tool Development**
  - Advanced argument parsing with subcommands
  - Configuration file handling
  - Plugin architectures
  - Cross-platform deployment

#### 🔬 Specialized Domains
- [ ] **Game Development**
  - Game engines with `bevy`
  - Real-time systems programming
  - Physics simulation
  - Graphics programming

- [ ] **Data Science & ML**
  - Data processing with `polars`
  - Machine learning with `candle`
  - Scientific computing patterns
  - Numerical computation libraries

- [ ] **Blockchain & Crypto**
  - Cryptographic primitives
  - Blockchain development
  - Smart contract platforms
  - Decentralized applications

### 📚 Learning Resources & Next Steps
1. **Read the Official Rust Book**: Complete chapters on lifetimes, smart pointers, and concurrency
2. **Explore Rust by Example**: Practice with more advanced examples
3. **Join the Community**: Participate in Rust forums, Discord, and contribute to open source
4. **Build Real Projects**: Create applications that solve actual problems
5. **Study Popular Crates**: Examine source code of well-known Rust libraries

### 🔄 Continuous Improvement
- **Code Reviews**: Share your code with the Rust community for feedback
- **Performance Profiling**: Learn to identify and optimize bottlenecks
- **Documentation**: Practice writing clear, comprehensive documentation
- **Testing**: Develop habits of writing tests alongside code
- **Refactoring**: Regularly improve code structure and design

➡️ **Compare with other languages**: See [TRAITSANDOOP.md](./TRAITSANDOOP.md) for Rust vs C# comparisons to understand Rust's unique approaches.
