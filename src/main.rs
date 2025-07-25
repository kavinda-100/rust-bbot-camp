//! # Rust Boot Camp 🦀
//!
//! A comprehensive Rust learning project that demonstrates fundamental to advanced concepts
//! in Rust programming through practical, hands-on examples.
//!
//! ## 🎯 Project Purpose
//! This project serves as a structured learning environment for mastering Rust programming.
//! Each module focuses on specific concepts, progressing from basic language features to
//! advanced topics like async programming, external integrations, and system programming.
//!
//! ## 📚 Module Organization
//!
//! ### 🔧 Core Language Features
//! - [`control_flow`] - Control structures: if/else, loops, while, break/continue
//! - [`my_functions`] - Function definitions, parameters, return types
//! - [`my_struct`] - Struct definitions and basic usage patterns
//! - [`my_methods`] - Implementation blocks and struct methods
//! - [`my_traits`] - Trait definitions, implementations, and polymorphism
//! - [`my_generic`] - Generic programming with type parameters and bounds
//! - [`my_enums`] - Enum variants and associated data
//! - [`my_match_expression`] - Pattern matching with match and if let
//! - [`my_closures`] - Anonymous functions and functional programming
//! - [`my_option`] - Option<T> for safe nullable value handling
//! - [`my_option_overload`] - Advanced Option usage patterns and chaining
//! - [`my_regular_expression`] - Regex pattern matching for text processing
//!
//! ### 📦 Collections & Data Structures
//! - [`my_vec`] - Vector operations: push, pop, iteration, slicing
//! - [`my_hash_map`] - HashMap key-value storage and manipulation
//! - [`my_iterator`] - Iterator patterns, lazy evaluation, adapters/consumers
//!
//! ### 🌐 External Integrations & I/O
//! - [`my_args`] - Command-line argument parsing with std::env
//! - [`my_http`] - HTTP requests with reqwest crate
//! - [`my_serde`] - JSON serialization/deserialization
//!
//! ### 🕒 Time & Date Management
//! - [`time_module::my_time`] - Standard library time operations
//! - [`time_module::my_chrono`] - Advanced date/time with chrono crate
//!
//! ### 📁 File & Data Operations
//! - [`file_system::my_file_system`] - File I/O and directory operations
//! - [`csv::my_csv`] - CSV file reading and writing
//!
//! ### 🎯 User Interface & CLI
//! - [`prompt_cmd::user_inputs`] - Basic user input handling
//! - [`prompt_cmd::my_inquire`] - Interactive prompts with inquire crate
//! - [`prompt_cmd::my_clap`] - Advanced CLI with clap integration
//! - [`clap::my_cli`] - Command-line interface examples
//!
//! ### ⚡ Asynchronous Programming
//! - [`asyncs::my_smol`] - Async programming with smol runtime
//! - [`asyncs::my_tokio`] - Async programming with tokio runtime
//!
//! ### 🔄 Process & Task Management
//! - [`external_process::my_external_proccess`] - Running external commands
//! - [`tasks::sort_values`] - Sorting algorithms and data manipulation
//!
//! ### 🧪 Testing & Quality Assurance
//! - [`unit_tests::my_code`] - Code to be tested
//! - [`unit_tests::my_test`] - Unit test implementations
//!
//! ## 🚀 Getting Started
//!
//! ### Running Individual Modules
//! To explore a specific concept, uncomment the corresponding line in the `main()` function:
//!
//! ```rust,no_run
//! fn main() {
//!     // Uncomment any of these to run specific examples:
//!     // my_functions::start();           // Basic function usage
//!     // my_struct::start();              // Struct definitions
//!     // my_traits::start();              // Trait implementations
//!     // asyncs::my_tokio::start().await; // Async programming (requires #[tokio::main])
//! }
//! ```
//!
//! ### Command-Line Arguments
//! Some modules support command-line arguments for interactive learning:
//!
//! ```bash
//! # Run with arguments for specific modules
//! cargo run -- arg1 arg2
//!
//! # Example: Testing command-line parsing
//! cargo run -- "John Doe" 25
//! ```
//!
//! ## 🎓 Learning Path Recommendations
//!
//! ### Beginner Track
//! 1. Start with `my_functions` and `control_flow`
//! 2. Learn data structures with `my_struct` and `my_methods`
//! 3. Explore collections with `my_vec` and `my_hash_map`
//! 4. Practice error handling with `my_option`
//!
//! ### Intermediate Track
//! 1. Master pattern matching with `my_enums` and `my_match_expression`
//! 2. Dive into generic programming with `my_generic`
//! 3. Understand traits and polymorphism with `my_traits`
//! 4. Explore functional programming with `my_closures` and `my_iterator`
//!
//! ### Advanced Track
//! 1. Learn async programming with `asyncs` modules
//! 2. Practice external integrations with `my_http` and `my_serde`
//! 3. Build CLI applications with `clap` and `prompt_cmd` modules
//! 4. Explore system programming with `external_process`
//!
//! ## 📖 Additional Resources
//! - For Rust vs C# comparisons, see the `TRAITSANDOOP.md` file
//! - Each module contains comprehensive examples and documentation
//! - Run `cargo doc --open` to generate and view full API documentation
//!
//! ## ⚙️ Features Demonstrated
//! - ✅ Memory safety without garbage collection
//! - ✅ Zero-cost abstractions and compile-time optimizations
//! - ✅ Fearless concurrency with ownership system
//! - ✅ Pattern matching and algebraic data types
//! - ✅ Trait-based polymorphism
//! - ✅ Functional programming patterns
//! - ✅ Async/await for concurrent programming
//! - ✅ Rich ecosystem integration
//! - ✅ Cross-platform development
//! - ✅ Performance-critical system programming

mod asyncs;
mod clap;
mod control_flow;
mod csv;
mod external_process;
mod file_system;
mod my_args;
mod my_closures;
mod my_enums;
mod my_functions;
mod my_generic;
mod my_hash_map;
mod my_http;
mod my_iterator;
mod my_match_expression;
mod my_methods;
mod my_option;
mod my_option_overload;
mod my_regular_expression;
mod my_serde;
mod my_struct;
mod my_traits;
mod my_vec;
mod prompt_cmd;
mod tasks;
mod time_module;
mod unit_tests;

// ================================ without async ================================
#[allow(dead_code)]
fn main() {
    // Call the start function from the my_functions module
    // my_functions::start();

    // Call the start function from the control_flow module
    // control_flow::start();

    // Call the start function from the user_inputs module
    // prompt_cmd::user_inputs::start();

    // Call the start function from the my_closures module
    // my_closures::start();

    // Call the start function from the my_match_expression module
    // my_match_expression::start();

    // Call the start function from the my_enums module
    // my_enums::start();

    // Call the start function from the my_option module
    // my_option::start();

    // Call the start function from the my_struct module
    // my_struct::start();

    // Call the start function from my_methods module
    // my_methods::start()

    // Call the start function from the my_generic module
    // my_generic::start();

    // Call the start function from the my_traits module
    // my_traits::start();

    // Call the start function from the my_vec module
    // my_vec::start();

    // Call the start function from the my_hash_map
    // my_hash_map::start();

    // Call the start function from the my_iterator module
    // my_iterator::start();

    // Call the start function from the time_module module
    // time_module::my_time::start();
    // time_module::my_chrono::start();

    // Call the start function from the file_system module
    // file_system::my_file_system::start();

    // Call the start function from the my_args module
    // my_args::start();

    // Call the start function from the clap module
    // clap::my_cli::start();

    // Call the start function from the my_serde module
    // my_serde::start();

    // Call the start function from the my_http module
    // my_http::start();

    // Call the start function from the my_option_overload module
    // my_option_overload::start();

    // Call the start function from the external_process module
    // external_process::my_external_proccess::start();

    // Call the start function from the csv module
    // csv::my_csv::start();

    // Call the start function from the my_regular_expression module
    // my_regular_expression::start();

    // Call the start function from the unit_tests module
    // unit_tests::my_code::start();

    // Call the start function from the prompt_cmd module
    // prompt_cmd::my_inquire::start();
    prompt_cmd::my_clap::start();

    // Call the start function from the tasks::sort_values module
    // tasks::sort_values::start();

    println!("End of main function");
}

// ================================ with async ================================

// ===== with Smol =====
// fn main() {
//     // Call the start function from the my_smol module
//     asyncs::my_smol::start();
// }

// ===== with Tokio =====
// #[tokio::main]
// async fn main() {
//     // Call the start function from the my_tokio module
//     asyncs::my_tokio::start().await;
// }
