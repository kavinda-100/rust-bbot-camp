//! # This is Rust Bootcamp, a collection of Rust code examples and exercises.
//! This project is designed to help you learn Rust by providing practical examples and tasks.
//! Each module contains a specific topic or task, and you can run them individually to see how they work.
//!
//! ### Features:
//!
//! - [x] Error handling with `Result<T, E>` and `Option<T>`
//! - [x] Struct definitions and implementations
//! - [x] Enums and pattern matching
//! - [x] Collections (Vec, HashMap, etc.)
//! - [x] File I/O operations
//! - [x] External crate dependencies (e.g., chrono, clap, serde)
//! - [x] More advanced generic programming
//! - [x] Command-line argument parsing (with std and clap)
//! - [x] Time and date handling
//! - [x] Serialization/deserialization (serde, JSON)
//! - [x] HTTP requests and networking
//! - [ ] Unit and integration tests
//! - [ ] Asynchronous programming (asyncs/await)
//! - [ ] Error propagation and custom error types
//! - [ ] More advanced trait usage and lifetimes
//! - [ ] Benchmarking and performance profiling
//! - [ ] Documentation with rustdoc

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
mod tasks;
mod time_module;
mod prompt_cmd;
mod unit_tests;

// ================================ without async ================================
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
    unit_tests::my_code::my_code::start();

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
