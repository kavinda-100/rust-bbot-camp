mod control_flow;
mod my_closures;
mod my_functions;
mod user_inputs;

mod my_enums;
mod my_generic;
mod my_hash_map;
mod my_iterator;
mod my_match_expression;
mod my_methods;
mod my_option;
mod my_struct;
mod my_traits;
mod my_vec;
mod tasks;
mod time_module;
mod file_system;

fn main() {
    // Call the start function from the my_functions module
    // my_functions::start();

    // Call the start function from the control_flow module
    // control_flow::start();

    // Call the start function from the user_inputs module
    // user_inputs::start();

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
    file_system::my_file_system::start();

    // Call the start function from the tasks::sort_values module
    // tasks::sort_values::start();

    println!("End of main function");
}
